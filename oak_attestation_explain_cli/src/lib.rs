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
use alloc::{fmt::Write, format, string::String};

use oak_attestation_explain::{HumanReadableExplanation, HumanReadableTitle};
use oak_attestation_verification_test_utils::reference_values_from_evidence;
use oak_proto_rust::oak::attestation::v1::{
    extracted_evidence::EvidenceValues, Evidence, OakRestrictedKernelData,
};

fn title(title: &str) -> String {
    format!(
        "



# {}
",
        title
    )
}

fn segment_title(title: &str, description: &str) -> String {
    format!(
        "

## {}
{}
",
        title, description
    )
}

pub fn explain_evidence(evidence: Evidence) -> Result<String, anyhow::Error> {
    let mut output = String::new();
    let extracted_evidence =
        oak_attestation_verification::verifier::extract_evidence(&evidence).unwrap();

    writeln!(output, "{}", title("Evidence:")).map_err(anyhow::Error::msg)?;

    match extracted_evidence.evidence_values.clone().take() {
        Some(EvidenceValues::OakRestrictedKernel(restricted_kernel_evidence)) => {
            match restricted_kernel_evidence {
                OakRestrictedKernelData {
                    root_layer: Some(root_layer),
                    kernel_layer: Some(kernel_layer),
                    application_layer: Some(application_layer),
                } => {
                    writeln!(
                        output,
                        "{}",
                        segment_title(
                            &root_layer.title().unwrap(),
                            &root_layer.description().unwrap()
                        )
                    )
                    .map_err(anyhow::Error::msg)?;
                    writeln!(
                        output,
                        "{}",
                        segment_title(
                            &kernel_layer.title().unwrap(),
                            &kernel_layer.description().unwrap(),
                        )
                    )
                    .map_err(anyhow::Error::msg)?;
                    writeln!(
                        output,
                        "{}",
                        segment_title(
                            &application_layer.title().unwrap(),
                            &application_layer.description().unwrap(),
                        )
                    )
                    .map_err(anyhow::Error::msg)?;
                    writeln!(output).map_err(anyhow::Error::msg)?;
                }
                _ => panic!("evidence values unexpectedly unset"),
            }
        }
        _ => panic!("not restricted kernel evidence"),
    };

    let reference_values = reference_values_from_evidence(extracted_evidence);

    writeln!(output, "{}", title("Reference values that describe this evidence:"))
        .map_err(anyhow::Error::msg)?;

    writeln!(
        output,
        "{}",
        reference_values.description().expect("could not get reference values description")
    )
    .map_err(anyhow::Error::msg)?;
    Ok(output)
}
