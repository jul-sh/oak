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

mod utils;

extern crate web_sys;

use anyhow::{anyhow, Context};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use oak_functions_abi::proto::Response;
use oak_remote_attestation::handshaker::{
    AttestationBehavior, ClientHandshaker, Encryptor, ServerIdentityVerifier,
};
use oak_remote_attestation_sessions::SessionId;
use prost::Message;
use wasm_bindgen::prelude::*;

mod proto {
    #![allow(clippy::return_self_not_must_use)]
    include!(concat!(env!("OUT_DIR"), "/oak.session.unary.v1.rs"));
}

#[wasm_bindgen]
pub fn set_hook() {
    utils::set_panic_hook();
}

// TODO(#1867): Add remote attestation support.
const TEE_MEASUREMENT: &[u8] = br"Test TEE measurement";

/// gRPC Attestation Service client implementation.
pub struct AttestationClient {
    session_id: SessionId,
    encryptor: Encryptor,
}

impl AttestationClient {
    pub async fn create(
        expected_tee_measurement: &[u8],
        server_verifier: ServerIdentityVerifier,
    ) -> anyhow::Result<Self> {
        let session_id: SessionId = rand::random();

        let mut handshaker = ClientHandshaker::new(
            AttestationBehavior::create_peer_attestation(expected_tee_measurement),
            server_verifier,
        );
        let client_hello = handshaker
            .create_client_hello()
            .context("Couldn't create client hello message")?;

        let mut response = message(proto::UnaryRequest {
            body: client_hello,
            session_id: session_id.to_vec(),
        })
        .await
        .context("Couldn't send client hello message")?;

        while !handshaker.is_completed() {
            let request = handshaker
                .next_step(&response.body)
                .context("Couldn't process handshake message")?;

            if let Some(request) = request {
                response = message(proto::UnaryRequest {
                    body: request,
                    session_id: session_id.to_vec(),
                })
                .await
                .context("Couldn't send client hello message")?;
            }
        }

        let encryptor = handshaker
            .get_encryptor()
            .context("Couldn't get encryptor")?;

        Ok(Self {
            session_id,
            encryptor,
        })
    }

    /// Sends data encrypted by the [`Encryptor`] to the server and decrypts the server responses.
    pub async fn send(&mut self, payload: Vec<u8>) -> anyhow::Result<Option<Vec<u8>>> {
        let encrypted_request = self
            .encryptor
            .encrypt(&payload)
            .context("Couldn't encrypt request")?;
        let encrypted_response = message(proto::UnaryRequest {
            session_id: self.session_id.to_vec(),
            body: encrypted_request,
        })
        .await
        .context("Couldn't send encrypted data request")?;

        let encoded_response = self
            .encryptor
            .decrypt(&encrypted_response.body)
            .context("Couldn't decrypt response")?;

        Ok(Some(encoded_response))
    }
}

#[wasm_bindgen]
pub struct MyClient {
    inner: AttestationClient,
}

#[wasm_bindgen]
impl MyClient {
    pub async fn new() -> MyClient {
        let inner = AttestationClient::create(
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
        MyClient { inner }
    }

    // Instead of using `&mut self`, this method takes ownership of an instance
    // of `MyClient`, and then returns that instance as the first element of a
    // JavaScript array. The reason it cannot use `&mut self` is that being
    // async, this function returns a Future. Futures always outlive the stack.
    // But `&mut self` is not static, it's stack allocated.
    // Ref: https://github.com/rustwasm/wasm-bindgen/issues/1858
    pub async fn invoke(mut client: MyClient, request: Vec<u8>) -> js_sys::Array {
        web_sys::console::log_1(&format!("Request: {:?}", request).into());

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

        web_sys::console::log_1(&format!("Response: {:?}", response_slice).into());

        let return_value = js_sys::Array::new();

        return_value.push(&JsValue::from(client));
        return_value.push(&JsValue::from(js_sys::Uint8Array::from(
            response_slice.as_slice(),
        )));

        return_value
    }
}

// one byte for the compression flag plus four bytes for the length
const GRPC_HEADER_SIZE: usize = 5;

fn encode_body<T>(msg: T) -> Bytes
where
    T: prost::Message,
{
    let mut buf = BytesMut::with_capacity(1024);

    // first skip past the header
    // cannot write it yet since we don't know the size of the
    // encoded message
    buf.reserve(GRPC_HEADER_SIZE);
    unsafe {
        buf.advance_mut(GRPC_HEADER_SIZE);
    }

    // write the message
    msg.encode(&mut buf).unwrap();

    // now we know the size of encoded message and can write the
    // header
    let len = buf.len() - GRPC_HEADER_SIZE;
    {
        let mut buf = &mut buf[..GRPC_HEADER_SIZE];

        // compression flag, 0 means "no compression"
        buf.put_u8(0);

        buf.put_u32(len as u32);
    }

    buf.split_to(len + GRPC_HEADER_SIZE).freeze()
}

async fn decode_body<T>(mut body: Bytes) -> T
where
    T: Default + prost::Message,
{
    // ignore the compression flag
    body.advance(1);

    let len = body.get_u32();
    #[allow(clippy::let_and_return)]
    let msg = T::decode(&mut body.split_to(len as usize)).unwrap();

    msg
}

pub async fn message(message_proto: proto::UnaryRequest) -> anyhow::Result<proto::UnaryResponse> {
    let request_bytes = encode_body(message_proto);
    let response_bytes = send(request_bytes).await?;
    let reply = decode_body::<proto::UnaryResponse>(response_bytes).await;
    Ok(reply)
}

pub async fn send(message_bytes: bytes::Bytes) -> anyhow::Result<bytes::Bytes> {
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:8080/oak.session.unary.v1.UnarySession/Message")
        .header(reqwest::header::CONTENT_TYPE, "application/grpc-web")
        .header("x-grpc-web", "1")
        .body(message_bytes)
        .send()
        .await
        .map_err(|error| anyhow!("Couldn't get response {:?}", error))?;
    let bytes = resp
        .bytes()
        .await
        .map_err(|_error| anyhow!("Couldn't get response bytes"))?;
    Ok(bytes)
}
