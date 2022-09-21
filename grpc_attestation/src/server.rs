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

//! Server-side implementation of the bidirectional gRPC remote attestation handshake
//! protocol.

#[cfg(feature = "streaming-session")]
use crate::proto::streaming_session_server::StreamingSession;
#[cfg(feature = "unary-session")]
use crate::proto::unary_session_server::UnarySession;
use crate::proto::{AttestationRequest, AttestationResponse, PublicKeyInfo};
use anyhow::Context;
use oak_remote_attestation::{
    crypto::Signer,
    handshaker::{AttestationBehavior, AttestationGenerator, EmptyAttestationVerifier},
};
use oak_remote_attestation_sessions::{SessionId, SessionState, SessionTracker};
use oak_utils::LogError;
use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
};
use tonic;

/// Number of sessions that will be kept in memory.
const SESSIONS_CACHE_SIZE: usize = 10000;

/// gRPC Attestation Service implementation.
pub struct AttestationServer<F, L: LogError, A: AttestationGenerator> {
    /// Business logic processor, accepts decrypted request and returns responses.
    request_handler: F,
    /// Error logging function that is required for logging attestation protocol errors.
    /// Errors are only logged on server side and are not sent to clients.
    error_logger: L,
    session_tracker: Mutex<SessionTracker<A, EmptyAttestationVerifier>>,
    signing_public_key: Vec<u8>,
    attestation: Vec<u8>,
}

impl<F, S, L, A> AttestationServer<F, L, A>
where
    F: 'static + Send + Sync + Clone + FnOnce(Vec<u8>) -> S,
    S: std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send + Sync,
    L: Send + Sync + Clone + LogError + 'static,
    A: AttestationGenerator + 'static,
{
    pub fn create(
        request_handler: F,
        error_logger: L,
        attestation_generator: A,
    ) -> anyhow::Result<Self> {
        let transcript_signer = Arc::new(Signer::create().context("Couldn't create signer")?);
        let signing_public_key = transcript_signer.public_key()?.to_vec();
        let attestation_behavior =
            AttestationBehavior::create(attestation_generator, EmptyAttestationVerifier);
        let attestation = attestation_behavior
            .generator
            .generate_attestation(&signing_public_key)?;
        let session_tracker = Mutex::new(SessionTracker::create(
            SESSIONS_CACHE_SIZE,
            attestation_behavior,
            transcript_signer,
        ));
        Ok(Self {
            request_handler,
            error_logger,
            session_tracker,
            signing_public_key,
            attestation,
        })
    }

    async fn handle_request(
        &self,
        request: AttestationRequest,
    ) -> anyhow::Result<AttestationResponse, tonic::Status> {
        let error_logger = self.error_logger.clone();

        let session_id: SessionId = request.session_id.try_into().map_err(|error| {
            error_logger.log_error(&format!("Received malformed session_id: {:?}", error));
            tonic::Status::invalid_argument("")
        })?;

        let mut session_state = {
            self.session_tracker
                .lock()
                .expect("Couldn't lock session_state mutex")
                .pop_or_create_session_state(session_id)
                .map_err(|error| {
                    error_logger.log_error(&format!("Couldn't pop session state: {:?}", error));
                    tonic::Status::internal("")
                })?
        };

        let response_body = match session_state {
            SessionState::HandshakeInProgress(ref mut handshaker) => {
                handshaker
                    .next_step(&request.body)
                    .map_err(|error| {
                        error_logger
                            .log_error(&format!("Couldn't process handshake message: {:?}", error));
                        tonic::Status::aborted("")
                    })?
                    // After receiving a valid `ClientIdentity` message
                    // (the last step of the key exchange)
                    // ServerHandshaker.next_step returns `None`. For unary
                    // request we do want to send an explicit confirmation in
                    // the form of a status message. Hence in case of `None`
                    // fallback to a default (empty) response.
                    .unwrap_or_default()
            }
            SessionState::EncryptedMessageExchange(ref mut encryptor) => {
                let decrypted_request = encryptor.decrypt(&request.body).map_err(|error| {
                    error_logger.log_error(&format!("Couldn't decrypt request: {:?}", error));
                    tonic::Status::aborted("")
                })?;

                let response = (self.request_handler.clone())(decrypted_request)
                    .await
                    .map_err(|error| {
                        error_logger.log_error(&format!("Couldn't handle request: {:?}", error));
                        tonic::Status::aborted("")
                    })?;

                encryptor.encrypt(&response).map_err(|error| {
                    error_logger.log_error(&format!("Couldn't encrypt response: {:?}", error));
                    tonic::Status::aborted("")
                })?
            }
        };

        // Note that we only get here if no errors occured during the preceding
        // steps. If errors do occur the session state as tracked by the server
        // is effectively erased. This allows the client to negotiate a new
        // handshake.
        self.session_tracker
            .lock()
            .expect("Couldn't lock session_state mutex")
            .put_session_state(session_id, session_state);

        Ok(AttestationResponse {
            body: response_body,
        })
    }
}

#[cfg(feature = "unary-session")]
#[tonic::async_trait]
impl<F, S, L, A> UnarySession for AttestationServer<F, L, A>
where
    F: 'static + Send + Sync + Clone + FnOnce(Vec<u8>) -> S,
    S: std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send + Sync,
    L: Send + Sync + Clone + LogError + 'static,
    A: AttestationGenerator + 'static,
{
    async fn message(
        &self,
        request: tonic::Request<AttestationRequest>,
    ) -> anyhow::Result<tonic::Response<AttestationResponse>, tonic::Status> {
        let attestation_request = request.into_inner();
        let attestation_response = self.handle_request(attestation_request).await?;

        Ok(tonic::Response::new(attestation_response))
    }

    async fn get_public_key_info(
        &self,
        _request: tonic::Request<()>,
    ) -> anyhow::Result<tonic::Response<PublicKeyInfo>, tonic::Status> {
        Ok(tonic::Response::new(PublicKeyInfo {
            public_key: self.signing_public_key.clone(),
            attestation: self.attestation.clone(),
        }))
    }
}

#[cfg(feature = "streaming-session")]
#[tonic::async_trait]
impl<F, S, L, A> StreamingSession for AttestationServer<F, L, A>
where
    F: 'static + Send + Sync + Clone + FnOnce(Vec<u8>) -> S,
    S: std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send + Sync + 'static,
    L: Send + Sync + Clone + LogError + 'static,
    A: AttestationGenerator + 'static,
{
    type MessageStream = std::pin::Pin<
        Box<
            dyn futures::Stream<Item = Result<AttestationResponse, tonic::Status>> + Send + 'static,
        >,
    >;

    async fn message(
        &self,
        request_stream: tonic::Request<tonic::Streaming<AttestationRequest>>,
    ) -> Result<tonic::Response<Self::MessageStream>, tonic::Status> {
        let error_logger = self.error_logger.clone();

        let response_stream = async_stream::try_stream! {
            let mut request_stream = request_stream.into_inner();

            while let Some(attestation_request) = request_stream.message().await.map_err(|error| {
                error_logger.log_error(&format!("Couldn't get next message: {:?}", error));
                tonic::Status::internal("")
            })? {
                let attestation_response = self.handle_request(attestation_request).await?;
                yield attestation_response;
            }
        };

        Ok(tonic::Response::new(Box::pin(response_stream)))
    }
}
