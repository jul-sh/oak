mod utils;

use oak_remote_attestation::handshaker::{AttestationBehavior, ClientHandshaker};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// TODO(#1867): Add remote attestation support.
const TEE_MEASUREMENT: &[u8] = br"Test TEE measurement";

#[wasm_bindgen]
pub struct Handshaker {
    inner: ClientHandshaker,
}

#[wasm_bindgen]
impl Handshaker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Handshaker {
        let handshaker = ClientHandshaker::new(
            AttestationBehavior::create_peer_attestation(TEE_MEASUREMENT),
            Box::new(|server_identity| {
                if !server_identity.additional_info.is_empty() {
                    Ok(())
                } else {
                    anyhow::bail!("No additional info provided.")
                }
            }),
        );
        Handshaker { inner: handshaker }
    }

    pub fn create_client_hello(&mut self) -> Vec<u8> {
        self.inner
            .create_client_hello()
            .expect("Couldn't create client hello message")
    }

    pub fn next_step(&mut self, next_step: Vec<u8>) -> Option<Vec<u8>> {
        self.inner
            .next_step(&next_step)
            .expect("Couldn't create client hello message")
    }
}
