mod utils;

extern crate wasm_bindgen;

use anyhow::Context;
use oak_functions_abi::proto::{Request, Response};
use oak_remote_attestation::handshaker::{
    AttestationBehavior, ClientHandshaker, Encryptor, ServerIdentityVerifier,
};
use oak_remote_attestation_sessions::SessionId;
use prost::message::Message;
use wasm_bindgen::prelude::*;

// TODO(#1867): Add remote attestation support.
const TEE_MEASUREMENT: &[u8] = br"Test TEE measurement";

/// gRPC Attestation Service client implementation.
#[wasm_bindgen]
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

        let mut response = message(UnaryRequest {
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
                response = message(UnaryRequest {
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
        let encrypted_response = message(UnaryRequest {
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

pub struct MyClient {
    inner: AttestationClient,
}

impl MyClient {
    pub async fn new(uri: &str) -> anyhow::Result<Self> {
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
        .context("Could not create Oak Functions client")?;
        Ok(MyClient { inner })
    }

    pub async fn invoke(&mut self, request: Request) -> anyhow::Result<Response> {
        self.inner
            .send(request.body)
            .await
            .context("Error invoking Oak Functions instance")?
            .ok_or_else(|| anyhow::anyhow!("Empty response"))
            .and_then(|rsp| Response::decode(rsp.as_ref()).context("Could not decode the response"))
    }
}
