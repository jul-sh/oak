use crate::proto::{unary_session_client::UnarySessionClient, UnaryRequest};
use anyhow::Context;
use oak_remote_attestation::handshaker::ServerIdentityVerifier;
use oak_remote_attestation_sessions::{client::AttestationClient, SessionId};
use tonic::transport::Channel;

pub async fn makeGrpcAttestationClient(
    uri: &str,
    expected_tee_measurement: &[u8],
    server_verifier: ServerIdentityVerifier,
) {
    let channel = Channel::from_shared(uri.to_string())
        .context("Couldn't create gRPC channel")?
        .connect()
        .await?;
    let mut client = UnarySessionClient::new(channel);
    let x = async move |session_id: SessionId, body| -> anyhow::Result<Vec<u8>> {
        let response = client
            .message(UnaryRequest {
                session_id: session_id.to_vec(),
                body,
            })
            .await
            .context("Couldn't send client hello message")?
            .into_inner();

        Ok(response.body)
    };
    let ac = AttestationClient::create(x, expected_tee_measurement, server_verifier);
    Ok(ac)
}
