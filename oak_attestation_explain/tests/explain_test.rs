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
#![feature(assert_matches)]

use std::fs;

use oak_attestation_explain::{HumanReadableExplanation, HumanReadableTitle};
use oak_proto_rust::oak::attestation::v1::{
    extracted_evidence::EvidenceValues, ExtractedEvidence, OakRestrictedKernelData,
};
use prost::Message;

// TODO: b/334900893 - Generate extracted evidence programatically.
const RK_EXTRACTED_EVIDENCE_PATH: &str = "testdata/rk_extracted_evidence.binarypb";

#[test]
fn produces_expected_explaination() {
    let mut extracted_evidence = {
        let serialized =
            fs::read(RK_EXTRACTED_EVIDENCE_PATH).expect("could not read extracted evidence");
        ExtractedEvidence::decode(serialized.as_slice()).expect("could not decode evidence")
    };

    eprintln!("{:?}", extracted_evidence.evidence_values);
    match extracted_evidence.evidence_values.take() {
        Some(EvidenceValues::OakRestrictedKernel(restricted_kernel_evidence)) => {
            assert_eq!(
                restricted_kernel_evidence.title().unwrap(),
                format!("Oak Restricted Kernel Stack in a {} TEE", "AMD SEV-SNP")
            );
            match restricted_kernel_evidence {
                OakRestrictedKernelData {
                    root_layer: Some(root_layer),
                    kernel_layer: Some(kernel_layer),
                    application_layer: Some(application_layer),
                } => {
                    assert_eq!(
                        root_layer.description().unwrap(),
                        r#"Initial memory digest: sha2-256:519bb2bd42afa2dd8cb3ca88aed6a8aea8905ee371f5e64b4aae03c7cec99a22

ⓘ The firmware attestation digest is the sha2-256 hash of the sha2-386 hash of the initial memory state taken by the AMD SoC. The original sha2-386 hash of the initial memory is: sha2-384:5a5cd76580dd3f0e9cc69ddfe7a6120919c02c3e376317bb3cc6de40a66e60683d380d966664d83fcd124f83f878d2ec.

One or more SLSA provenances mapping this layer's attestation digest to source code should be available on rekor. They can be obtained with the following search:
https://search.sigstore.dev/?hash=519bb2bd42afa2dd8cb3ca88aed6a8aea8905ee371f5e64b4aae03c7cec99a22"#
                    );
                    assert_eq!(
                        kernel_layer.description().unwrap(),
                        r#"Kernel Image: sha2-256:bb149e581ed858d4269acf844ca9ceb00162f2e2aa2e2061072462a05e0c8743
Kernel Setup Data: sha2-256:4cd020820da663063f4185ca14a7e803cd7c9ca1483c64e836db840604b6fac1
Kernel Command Line: console=ttyS0
Initial RAM Disk: sha2-256:0000000000000000000000000000000000000000000000000000000000000000

One or more SLSA provenances mapping this layer's attestation digests to source code should be available on rekor. They can be obtained with the following search:
https://search.sigstore.dev/?hash=bb149e581ed858d4269acf844ca9ceb00162f2e2aa2e2061072462a05e0c8743"#
                    );
                    assert_eq!(
                        application_layer.description().unwrap(),
                        r#"Binary digest: sha2-256:b5cae5b9b92104f7ebc08b7cd7dc9f2fb191ebd5db7041421f2f885b777d5040
Binary provenance: https://search.sigstore.dev/?hash=b5cae5b9b92104f7ebc08b7cd7dc9f2fb191ebd5db7041421f2f885b777d5040"#
                    );
                }
                _ => panic!("evidence values unexpectedly unset"),
            }
        }
        _ => panic!("not restricted kernel evidence"),
    }
}
