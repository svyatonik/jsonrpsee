// Copyright 2019 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use jsonrpsee::{client::Subscription, Client};
jsonrpsee::rpc_api! {
    System {
        /// Get the node's implementation name. Plain old string.
        fn system_name() -> String;

        /// Returns the roles the node is running as.
        #[rpc(method = "system_nodeRoles")]
        fn system_node_roles() -> Vec<String>;

        #[rpc(method = "chain_getBlockHash", positional_params)]
        fn chain_block_hash(id: Option<String>) -> Option<String>;
    }
}

#[derive(Debug, serde::Deserialize)]
struct Header {
    number: String,
}

fn main() {
    async_std::task::block_on(async move {
        let transport =
            jsonrpsee::transport::ws::WsTransportClient::new("wss://kusama-rpc.polkadot.io")
                .await
                .unwrap();
        let mut raw_client = jsonrpsee::raw::RawClient::new(transport);
        let v = System::system_name(&mut raw_client).await.unwrap();
        println!("{:?}", v);

        let client: Client = raw_client.into();

        let mut sub: Subscription<Header> = client
            .subscribe(
                "chain_subscribeNewHeads",
                jsonrpsee::common::Params::None,
                "chain_unsubscribeNewHeads",
            )
            .await
            .unwrap();

        while let ev = sub.next().await {
            println!("ev: {:?}", ev);
        }
    });
}
