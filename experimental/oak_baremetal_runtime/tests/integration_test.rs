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

use oak_baremetal_runtime::schema::TrustedRuntime;
use oak_remote_attestation::handshaker::{
    AttestationBehavior, ClientHandshaker, EmptyAttestationVerifier,
};
use oak_remote_attestation_amd::PlaceholderAmdAttestationGenerator;

mod schema {
    #![allow(clippy::derivable_impls, clippy::needless_borrow)]
    #![allow(dead_code, unused_imports)]

    include!(concat!(env!("OUT_DIR"), "/schema_generated.rs"));
    include!(concat!(env!("OUT_DIR"), "/schema_services_clients.rs"));
}

const MOCK_SESSION_ID: &[u8; 8] = &[0, 0, 0, 0, 0, 0, 0, 0];

#[test]
fn it_should_not_handle_user_requests_before_initialization() {
    let attestation_behavior =
        AttestationBehavior::create(PlaceholderAmdAttestationGenerator, EmptyAttestationVerifier);
    let runtime = oak_baremetal_runtime::RuntimeImplementation::new(attestation_behavior.clone());
    let mut client = schema::TrustedRuntimeClient::new(TrustedRuntime::serve(runtime));

    let mut handshaker =
        ClientHandshaker::new(attestation_behavior).expect("could not create client handshaker");
    let client_hello = handshaker
        .create_client_hello()
        .expect("Couldn't create client hello message");

    let owned_request_flatbuffer = {
        let mut builder = oak_idl::utils::OwnedFlatbufferBuilder::default();
        let session_id = &schema::SessionId::new(MOCK_SESSION_ID);
        let body = builder.create_vector::<u8>(&client_hello);
        let flatbuffer = schema::UserRequest::create(
            &mut builder,
            &schema::UserRequestArgs {
                session_id: Some(session_id),
                body: Some(body),
            },
        );
        builder
            .finish(flatbuffer)
            .map_err(|err| {
                oak_idl::Status::new_with_message(oak_idl::StatusCode::Internal, err.to_string())
            })
            .unwrap()
    };
    let result = client.handle_user_request(owned_request_flatbuffer.into_vec());

    assert_eq!(
        result.unwrap_err(),
        oak_idl::Status::new(oak_idl::StatusCode::FailedPrecondition)
    );
}
