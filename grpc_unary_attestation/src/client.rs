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

use crate::proto::{unary_session_client::UnarySessionClient, UnaryRequest, UnaryResponse};
use anyhow::Context;
use core::future::Future;
use oak_remote_attestation::handshaker::{
    AttestationBehavior, ClientHandshaker, Encryptor, ServerIdentityVerifier,
};
use oak_remote_attestation_sessions::SessionId;

/// gRPC Attestation Service client implementation.
pub struct AttestationClient<F, Fut>
where
    F: Fn(UnaryRequest) -> Fut,
    Fut: Future<Output = anyhow::Result<UnaryResponse>>,
{
    session_id: SessionId,
    encryptor: Encryptor,
    client: F,
}

impl<F, Fut> AttestationClient<F, Fut>
where
    F: Fn(UnaryRequest) -> Fut,
    Fut: Future<Output = anyhow::Result<UnaryResponse>>,
{
    pub async fn create(
        client: F,
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

        let mut response = client(UnaryRequest {
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
                response = client(UnaryRequest {
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
            client,
        })
    }

    /// Sends data encrypted by the [`Encryptor`] to the server and decrypts the server responses.
    pub async fn send(&mut self, payload: Vec<u8>) -> anyhow::Result<Option<Vec<u8>>> {
        let encrypted_request = self
            .encryptor
            .encrypt(&payload)
            .context("Couldn't encrypt request")?;
        let encrypted_response = (self.client)(UnaryRequest {
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
