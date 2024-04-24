//
// Copyright 2024 The Project Oak Authors
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

#![no_std]

extern crate alloc;

#[cfg(test)]
extern crate std;

use alloc::{format, string::String};

use anyhow::{Context, Result};
use oak_proto_rust::oak::{
    attestation::v1::{
        root_layer_data::Report, ApplicationLayerData, KernelLayerData, OakContainersData,
        OakRestrictedKernelData, RootLayerData, SystemLayerData,
    },
    RawDigest,
};
use sha2::{Digest, Sha256};
use zerocopy::{FromBytes, FromZeroes};

use crate::alloc::string::ToString;

fn get_tee_name(tee_report: &Report) -> &'static str {
    match tee_report {
        Report::SevSnp(_) => "AMD SEV-SNP",
        Report::Tdx(_) => "Intel TDX",
        Report::Fake(_) => "UNSAFE FAKE",
    }
}

/// Provides human readable explanations for attestation data.
pub trait HumanReadableTitle {
    /// Concise title, eg. "Oak Containers Stack in a AMD SEV-SNP TEE".
    fn title(&self) -> Result<String, anyhow::Error>;
}

pub trait HumanReadableExplanation {
    /// Provides human readable explanations for a layer of an attestation data
    /// stack, including directions to map the attestation data to source code.
    fn description(&self) -> Result<String, anyhow::Error>;
}

impl HumanReadableTitle for OakRestrictedKernelData {
    fn title(&self) -> Result<String, anyhow::Error> {
        let tee_report = self
            .root_layer
            .as_ref()
            .context("unexpectedly unset proto field")?
            .report
            .as_ref()
            .context("unexpectedly unset proto field")?;
        let tee_name = get_tee_name(tee_report);
        Ok(format!("Oak Restricted Kernel Stack in a {} TEE", tee_name))
    }
}

impl HumanReadableTitle for OakContainersData {
    fn title(&self) -> Result<String, anyhow::Error> {
        let tee_report = self
            .root_layer
            .clone()
            .context("unexpectedly unset proto field")?
            .report
            .clone()
            .context("unexpectedly unset proto field")?;
        let tee_name = get_tee_name(&tee_report);
        Ok(format!("Oak Conatiners Stack in a {} TEE", tee_name))
    }
}

impl HumanReadableTitle for RootLayerData {
    fn title(&self) -> Result<String, anyhow::Error> {
        Ok("Root Layer".to_string())
    }
}

impl HumanReadableExplanation for RootLayerData {
    fn description(&self) -> Result<String, anyhow::Error> {
        match self.report.as_ref().context("unexpectedly unset proto field")? {
            Report::SevSnp(report) => {
                let initial_memory_sha256_digest =
                    SNPInitialMemoryMeasurement::try_from(report.initial_measurement.as_slice())?;
                Ok(format!(
                    "Initial memory digest: {}

{}

One or more SLSA provenances mapping this layer's attestation digest to source code should be available on rekor. They can be obtained with the following search:
{}",
                    initial_memory_sha256_digest.display_hash(),
                    initial_memory_sha256_digest.display_hash_explaination(),
                    initial_memory_sha256_digest.provenance_link()
                ))
            }
            _ => Err(anyhow::Error::msg(
                "human readable description not yet implemented for this type of TEE",
            )),
        }
    }
}

impl HumanReadableTitle for KernelLayerData {
    fn title(&self) -> Result<String, anyhow::Error> {
        Ok("Kernel Layer".to_string())
    }
}

impl HumanReadableExplanation for KernelLayerData {
    fn description(&self) -> Result<String, anyhow::Error> {
        let kernel_image_digest: ArtifactDigestSha2_256 =
            self.kernel_image.as_ref().context("unexpectedly unset proto field").and_then(
                |digest| ArtifactDigestSha2_256::try_from(digest).map_err(anyhow::Error::from),
            )?;
        let init_ram_fs_digest: ArtifactDigestSha2_256 =
            self.init_ram_fs.as_ref().context("unexpectedly unset proto field").and_then(
                |digest| ArtifactDigestSha2_256::try_from(digest).map_err(anyhow::Error::from),
            )?;

        let digests = format!(
            "Kernel Image: {}
Kernel Setup Data: {}
Kernel Command Line: {}
Initial RAM Disk: {}",
            kernel_image_digest.display_hash(),
            ArtifactDigestSha2_256::try_from(
                self.kernel_setup_data.as_ref().context("unexpectedly unset proto field")?
            )?
            .display_hash(),
            self.kernel_raw_cmd_line.as_ref().context("unexpectedly unset proto field")?,
            init_ram_fs_digest.display_hash()
        );

        Ok(format!(
            "{}

One or more SLSA provenances mapping this layer's attestation digests to source code should be available on rekor. They can be obtained with the following search:
{}",
            digests,
            kernel_image_digest.provenance_link()
        ))
    }
}

impl HumanReadableTitle for SystemLayerData {
    fn title(&self) -> Result<String, anyhow::Error> {
        Ok("System Layer".to_string())
    }
}

impl HumanReadableExplanation for SystemLayerData {
    fn description(&self) -> Result<String, anyhow::Error> {
        let system_image_digest =
            self.system_image.as_ref().context("unexpectedly unset proto field").and_then(
                |digest| ArtifactDigestSha2_256::try_from(digest).map_err(anyhow::Error::from),
            )?;
        Ok(format!(
            "System image digest: {}
System image provenance: {}",
            system_image_digest.display_hash(),
            system_image_digest.provenance_link(),
        ))
    }
}

impl HumanReadableTitle for ApplicationLayerData {
    fn title(&self) -> Result<String, anyhow::Error> {
        Ok("Application Layer".to_string())
    }
}

impl HumanReadableExplanation for ApplicationLayerData {
    fn description(&self) -> Result<String, anyhow::Error> {
        let digests = {
            let binary_digest =
                self.binary.as_ref().context("unexpectedly unset proto field").and_then(
                    |digest| ArtifactDigestSha2_256::try_from(digest).map_err(anyhow::Error::from),
                )?;

            // Restricted Kernel Applications do not use a config, no digest is included in
            // the evidence.
            if let Some(config_digest) = self
                .config
                .as_ref()
                .context("unexpectedly unset proto field")
                .and_then(|digest| {
                    ArtifactDigestSha2_256::try_from(digest).map_err(anyhow::Error::from)
                })
                .ok()
            {
                format!(
                    "Binary digest: {}
Binary provenance: {}
Config digest: {}
Config provenance: {}",
                    binary_digest.display_hash(),
                    binary_digest.provenance_link(),
                    config_digest.display_hash(),
                    config_digest.provenance_link()
                )
            } else {
                format!(
                    "Binary digest: {}
Binary provenance: {}",
                    binary_digest.display_hash(),
                    binary_digest.provenance_link(),
                )
            }
        };

        Ok(digests)
    }
}

trait OakDigestDisplay {
    /// String of the hash in the following format: [alg]:[hash]. The algorithm
    /// descripter follows fieldnames of the proto-message: oak.RawDigest
    /// struct.
    fn display_hash(&self) -> String;

    fn provenance_link(&self) -> String;
}

/// Convience struct that maintains type safety determinig the length of the
/// underlying slice. Provides consistent methods for creating and printing the
/// data.
#[derive(FromBytes, FromZeroes)]
struct ArtifactDigestSha2_256(pub [u8; 32]);

impl TryFrom<&RawDigest> for ArtifactDigestSha2_256 {
    type Error = anyhow::Error;

    fn try_from(value: &RawDigest) -> Result<Self, Self::Error> {
        match value.sha2_256.as_slice() {
            [] => Err(anyhow::Error::msg("no sha2_256 hash found")),
            _ => value.sha2_256.as_slice().try_into(),
        }
    }
}

impl TryFrom<&[u8]> for ArtifactDigestSha2_256 {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        ArtifactDigestSha2_256::read_from(slice).context("unexpected length of measurement")
    }
}

impl OakDigestDisplay for ArtifactDigestSha2_256 {
    // # TODO: b/335458726 - Stop permitting unexported code once mod is completed.
    #[allow(dead_code)]
    fn provenance_link(&self) -> String {
        format!("https://search.sigstore.dev/?hash={}", hex::encode(self.0))
    }
    fn display_hash(&self) -> String {
        format!("sha2-256:{}", hex::encode(self.0.as_slice()))
    }
}

/// Convience struct that maintains type safety determinig the length of the
/// underlying slice. Provides consistent methods for creating and printing the
/// data.
#[derive(FromZeroes, FromBytes)]
struct SNPInitialMemoryMeasurement([u8; 48]);

impl SNPInitialMemoryMeasurement {
    fn sha2_256(&self) -> ArtifactDigestSha2_256 {
        Sha256::digest(self.0).as_slice().try_into().expect("sha256 digest to have expected length")
    }
    /// Print a hash in the following format: [alg]:[hash]. The algorithm
    /// descripter follows fieldnames of the proto-message: oak.RawDigest
    /// struct.
    fn display_hash_explaination(&self) -> String {
        format!(
            "ⓘ The firmware attestation digest is the sha2-256 hash of the sha2-386 hash of the initial memory state taken by the AMD SoC. The original sha2-386 hash of the initial memory is: sha2-384:{}.",
            { hex::encode(self.0.as_slice()) }
        )
    }
}

impl OakDigestDisplay for SNPInitialMemoryMeasurement {
    fn display_hash(&self) -> String {
        self.sha2_256().display_hash()
    }

    fn provenance_link(&self) -> String {
        format!("https://search.sigstore.dev/?hash={}", hex::encode(self.sha2_256().0))
    }
}

impl TryFrom<&[u8]> for SNPInitialMemoryMeasurement {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        SNPInitialMemoryMeasurement::read_from(slice).context("unexpected length of measurement")
    }
}
