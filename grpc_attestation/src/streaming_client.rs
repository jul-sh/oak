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

use crate::proto::{
    streaming_session_client::StreamingSessionClient, AttestationRequest, AttestationResponse,
};
use anyhow::Context;
use async_trait::async_trait;
use oak_remote_attestation_sessions::SessionId;
use oak_remote_attestation_sessions_client::AttestationTransport;
use tokio::sync::mpsc::Sender;
use tonic::{transport::Channel, Request, Streaming};

const MESSAGE_BUFFER_SIZE: usize = 1;

/// Convenience structure for sending requests to and receiving responses from the server.
struct StreamingGrpcChannel {
    sender: Sender<AttestationRequest>,
    response_stream: Streaming<AttestationResponse>,
}

impl StreamingGrpcChannel {
    async fn create(uri: &str) -> anyhow::Result<Self> {
        let channel = Channel::from_shared(uri.to_string())
            .context("Couldn't create gRPC channel")?
            .connect()
            .await
            .context("Couldn't connect via gRPC channel")?;
        let mut client = StreamingSessionClient::new(channel);

        let (sender, mut receiver) = tokio::sync::mpsc::channel(MESSAGE_BUFFER_SIZE);

        let request_stream = async_stream::stream! {
            while let Some(message) = receiver.recv().await {
                yield message;
            }
        };

        let response = client
            .message(Request::new(request_stream))
            .await
            .context("Couldn't send request")?;
        let response_stream = response.into_inner();

        Ok(Self {
            sender,
            response_stream,
        })
    }

    async fn send(&mut self, request: AttestationRequest) -> anyhow::Result<()> {
        self.sender
            .send(request)
            .await
            .context("Couldn't send request")
    }

    async fn receive(&mut self) -> anyhow::Result<Option<AttestationResponse>> {
        self.response_stream
            .message()
            .await
            .context("Couldn't receive response")
    }
}

/// gRPC implementation of [`AttestationTransport`] trait using a bidi gRPC stream.
pub struct StreamingGrpcClient {
    inner: StreamingGrpcChannel,
}

impl StreamingGrpcClient {
    pub async fn create(uri: &str) -> anyhow::Result<Self> {
        let inner = StreamingGrpcChannel::create(uri)
            .await
            .context("Couldn't create gRPC channel")?;
        Ok(Self { inner })
    }
}

// Async trait requires the definition and all implementations to be marked as
// optionally [`Send`] if one implementation is not.
#[async_trait(?Send)]
impl AttestationTransport for StreamingGrpcClient {
    async fn message(&mut self, session_id: SessionId, body: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        self.inner
            .send(AttestationRequest {
                body,
                session_id: session_id.to_vec(),
            })
            .await
            .context("Couldn't send request")?;

        let response = self
            .inner
            .receive()
            .await
            .context("Received an error while for waiting to receive response")?
            .context("Received no response")?
            .body;

        Ok(response)
    }
}
