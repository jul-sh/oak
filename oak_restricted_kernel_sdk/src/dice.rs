//
// Copyright 2023 The Project Oak Authors
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

use crate::syscall::read;
use oak_dice::evidence::{RestrictedKernelDiceData, P256_PRIVATE_KEY_SIZE};
use oak_restricted_kernel_interface::DICE_DATA_FD;
use p256::ecdsa::SigningKey;
use zerocopy::{AsBytes, FromZeroes};

#[allow(dead_code)]
fn get_restricted_kernel_dice_data() -> anyhow::Result<RestrictedKernelDiceData> {
    let mut result = RestrictedKernelDiceData::new_zeroed();
    let buffer = result.as_bytes_mut();
    let len = read(DICE_DATA_FD, buffer).map_err(|err| anyhow::anyhow!("read failure: {err}"))?;
    if len != buffer.len() {
        anyhow::bail!("invalid dice data size");
    }
    Ok(result)
}

pub struct Signer {
    key: SigningKey,
}

impl Signer {
    pub fn sign(&self, message: &[u8]) -> oak_crypto::signer::Signature {
        <SigningKey as oak_crypto::signer::Signer>::sign(&self.key, message)
    }
}

impl TryFrom<&oak_dice::evidence::RestrictedKernelDiceData> for Signer {
    type Error = anyhow::Error;
    fn try_from(
        dice_data: &oak_dice::evidence::RestrictedKernelDiceData,
    ) -> Result<Self, Self::Error> {
        let key = SigningKey::from_slice(
            &dice_data.application_private_keys.signing_private_key[..P256_PRIVATE_KEY_SIZE],
        )
        .map_err(|error| anyhow::anyhow!("couldn't deserialize signing key: {}", error))?;
        Ok(Signer { key })
    }
}
