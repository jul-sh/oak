//
// Copyright 2022 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

extern crate web_sys;

use crate::proto::{UnaryRequest, UnaryResponse};
use anyhow::Context;
use async_trait::async_trait;
use oak_functions_abi::proto::Response;
use oak_remote_attestation_sessions::SessionId;
use oak_remote_attestation_sessions_client::{GenericAttestationClient, UnaryClient};
use prost::Message;
use wasm_bindgen::prelude::*;

mod grpc_web;

mod proto {
    #![allow(clippy::return_self_not_must_use)]
    include!(concat!(env!("OUT_DIR"), "/oak.session.unary.v1.rs"));
}

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// TODO(#1867): Add remote attestation support.
const TEE_MEASUREMENT: &[u8] = br"Test TEE measurement";

struct GrpcWebClient {}

#[async_trait(? Send)]
impl UnaryClient for GrpcWebClient {
    async fn message(&mut self, session_id: SessionId, body: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let reply = grpc_web::grpc_web_unary::<UnaryRequest, UnaryResponse>(
            "http://localhost:8080/oak.session.unary.v1.UnarySession/Message",
            UnaryRequest {
                session_id: session_id.to_vec(),
                body,
            },
        )
        .await?;
        Ok(reply.body)
    }
}

#[wasm_bindgen]
pub struct AttestationClient {
    inner: GenericAttestationClient<GrpcWebClient>,
}

#[wasm_bindgen]
impl AttestationClient {
    pub async fn new() -> AttestationClient {
        let grpc_web_client = GrpcWebClient {};
        let inner = GenericAttestationClient::create(
            grpc_web_client,
            TEE_MEASUREMENT,
            Box::new(|server_identity| {
                if !server_identity.additional_info.is_empty() {
                    Ok(())
                } else {
                    anyhow::bail!("No additional info provided.")
                }
            }),
        )
        .await
        .context("Could not create Oak Functions client")
        .unwrap();
        AttestationClient { inner }
    }

    // Instead of using `&mut self`, this method takes ownership of an instance
    // of `AttestationClient`, and then returns that instance as the first element of a
    // JavaScript array. The reason it cannot use `&mut self` is that being
    // async, this function returns a Future. Futures always outlive the stack.
    // But `&mut self` is not static, it's stack allocated.
    // Ref: https://github.com/rustwasm/wasm-bindgen/issues/1858
    pub async fn invoke(mut client: AttestationClient, request: Vec<u8>) -> js_sys::Array {
        let response = client
            .inner
            .send(request)
            .await
            .context("Error invoking Oak Functions instance")
            .unwrap()
            .ok_or_else(|| anyhow::anyhow!("Empty response"))
            .and_then(|rsp| {
                Response::decode(rsp.as_ref())
                    .map_err(|_| anyhow::anyhow!("Couldn't decode response"))
            })
            .unwrap();

        let response_slice = response.body[0..response.length.try_into().unwrap()].to_vec();

        let return_value = js_sys::Array::new();

        return_value.push(&JsValue::from(client));
        return_value.push(&JsValue::from(js_sys::Uint8Array::from(
            response_slice.as_slice(),
        )));

        return_value
    }
}
