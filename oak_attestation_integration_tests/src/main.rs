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
//! Binary that's used to facilitate tests. It create binary-encoded evidence,
//! endorsements, and reference values using the code current attestation code.
//!
//! The resulting outputs are meant to be stored. They can be used to validate
//! that newer versions of the verification library continue to be able to
//! verify older versions of these artifacts. See b/370445356.

use chrono::Utc;
use oak_attestation_verification_test_utils::create_oak_containers_standalone_endorsed_evidence_with_matching_reference_values;
use oak_proto_rust::oak::{attestation::v1::ReferenceValues, session::v1::EndorsedEvidence};
use prost::Message;
use tokio::{fs::File, io::AsyncWriteExt};

// Constants that define the measurements for evidence.
const SETUP_DATA_DIGEST: &[u8] = &[1u8; 32];
const KERNEL_MEASUREMENT: &[u8] = &[2u8; 32];
const RAM_DISK_DIGEST: &[u8] = &[3u8; 32];
const MEMORY_MAP_DIGEST: &[u8] = &[4u8; 32];
const ACPI_DIGEST: &[u8] = &[5u8; 32];
const KERNEL_CMDLINE: &str = "custom kernel command line";
const STAGE1_SYSTEM_IMAGE: &[u8] = &[6u8; 32];
const APPLICATION_IMAGE: &[u8] = &[7u8; 32];
const APPLICATION_CONFIG: &[u8] = &[8u8; 32];

use chrono::{DateTime, Utc};

/// Represents a snapshot of endorsed evidence and reference values at a
/// specific version and time. This struct is used to manage and organize test
/// data for different versions of the attestation system.
struct Snapshot {
    version: u16,
    timestamp: DateTime<Utc>,
}
impl Snapshot {
    const TESTDATA_DIR: &'static str = "./testdata/snapshots";
    const ENDORSED_EVIDENCE_FILE: &'static str = "endorsed_evidence.binarypb";
    const REFERENCE_VALUES_FILE: &'static str = "reference_values.binarypb";

    fn new(version: u16) -> Self {
        Self { version, timestamp: Utc::now() }
    }

    /// Returns the directory name for this snapshot.
    fn dirname(&self) -> String {
        format!(
            "{}/{:05}-{}",
            Self::TESTDATA_DIR,
            self.version,
            self.timestamp.format("%Y-%m-%dT%H:%M:%S%:z")
        )
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn endorsed_evidence_path(&self) -> String {
        format!("{}/{}", self.dirname(), Self::ENDORSED_EVIDENCE_FILE)
    }

    pub fn reference_values_path(&self) -> String {
        format!("{}/{}", self.dirname(), Self::REFERENCE_VALUES_FILE)
    }

    /// Retrieves the most recent snapshot by finding the highest version number
    /// in the testdata directory.
    pub async fn most_recent() -> anyhow::Result<Self> {
        let entries = tokio::fs::read_dir(Self::TESTDATA_DIR)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read testdata directory: {}", e))?;
        let versions = entries
            .map(|entry| async move {
                let entry =
                    entry.map_err(|e| anyhow::anyhow!("Failed to read directory entry: {}", e))?;
                let file_name = entry.file_name();
                let version = file_name
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in filename"))?
                    .split('-')
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Invalid directory name format"))?
                    .parse::<u16>()
                    .map_err(|e| anyhow::anyhow!("Failed to parse version number: {}", e))?;
                Ok(version)
            })
            .collect::<Vec<_>>()
            .await;

        let max_version = versions.into_iter().filter_map(Result::ok).max().unwrap_or(0);

        Ok(Self::new(max_version))
    }

    /// Creates a new Snapshot with a version number one higher than the most
    /// recent snapshot. This function is used to generate the next
    /// sequential snapshot for testing purposes.
    pub async fn next() -> anyhow::Result<Self> {
        let most_recent_snapshot = Self::most_recent().await?;
        Ok(Self::new(most_recent_snapshot.version() + 1))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (endorsed_evidence, reference_values) = {
        let stage0_digests = oak_proto_rust::oak::attestation::v1::Stage0Measurements {
            setup_data_digest: SETUP_DATA_DIGEST.to_vec(),
            kernel_measurement: KERNEL_MEASUREMENT.to_vec(),
            ram_disk_digest: RAM_DISK_DIGEST.to_vec(),
            memory_map_digest: MEMORY_MAP_DIGEST.to_vec(),
            acpi_digest: ACPI_DIGEST.to_vec(),
            kernel_cmdline: KERNEL_CMDLINE.to_string(),
        };
        create_oak_containers_standalone_endorsed_evidence_with_matching_reference_values(
            stage0_digests,
            STAGE1_SYSTEM_IMAGE,
            APPLICATION_IMAGE,
            APPLICATION_CONFIG.to_vec(),
        )
        .await
    };

    let next_snapshot = Snapshot::next().await?;
    let dir_path = next_snapshot.dirname();
    tokio::fs::create_dir_all(&dir_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create directory {}: {}", dir_path, e))?;

    let endorsed_evidence_path = next_snapshot.endorsed_evidence_path();
    let reference_values_path = next_snapshot.reference_values_path();

    let (endorsed_evidence_result, reference_values_result) = tokio::join!(
        async {
            let mut endorsed_evidence_file = File::create(endorsed_evidence_path)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to create endorsed evidence file: {}", e))?;
            endorsed_evidence_file
                .write_all(&endorsed_evidence.encode_to_vec())
                .await
                .map_err(|e| anyhow::anyhow!("Failed to write endorsed evidence: {}", e))?;
            Ok(())
        },
        async {
            let mut reference_values_file = File::create(reference_values_path)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to create reference values file: {}", e))?;
            reference_values_file
                .write_all(&reference_values.encode_to_vec())
                .await
                .map_err(|e| anyhow::anyhow!("Failed to write reference values: {}", e))?;
            Ok(())
        }
    );
    endorsed_evidence_result?;
    reference_values_result?;

    Ok(())
}
