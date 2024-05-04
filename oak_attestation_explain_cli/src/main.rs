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

#![feature(let_chains)]

use std::{fs, path::PathBuf};

use clap::Parser;
use oak_attestation_explain::{HumanReadableExplanation, HumanReadableTitle};
use oak_proto_rust::oak::attestation::v1::{
    extracted_evidence::EvidenceValues, Evidence, OakRestrictedKernelData,
};
use prost::Message;

#[derive(Parser, Debug)]
#[group(skip)]
pub struct Params {
    /// Path to the evidence to inspect.
    #[arg(long, value_parser = path_exists, default_value = "oak_attestation_verification/testdata/rk_evidence.binarypb")]
    pub evidence: PathBuf,
}

fn path_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !fs::metadata(s).map_err(|err| err.to_string())?.is_file() {
        Err(String::from("path does not represent a file"))
    } else {
        Ok(path)
    }
}

fn is_object_of_strings(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(object) => {
            object.iter().all(|(_key, value)| matches!(value, serde_json::Value::String(_)))
        }
        _ => false,
    }
}

fn is_array_of_strings(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Array(array) => {
            array.iter().all(|value| matches!(value, serde_json::Value::String(_)))
        }
        _ => false,
    }
}

fn simplify_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            // Strings and regexes are nested under a value key. Remove
            // for human readability.
            if map.len() == 1
                && let Some(inner_value) = map.get_mut("value")
                && is_array_of_strings(inner_value)
            {
                *value = inner_value.clone();
                return simplify_value(value);
            }
            // Some reference values nest digests under a key. Remove
            // for human readability.
            if map.len() == 1
                && let Some(digests) = map.get_mut("digests")
                && matches!(digests, serde_json::Value::Object(_))
            {
                *value = digests.clone();
                return simplify_value(value);
            }

            // Reformat digests to be hex encoded.
            if let Some(digests) = map.get_mut("digests")
                && let serde_json::Value::Array(digests_vec) = digests
                && digests_vec.iter().all(is_object_of_strings)
            {
                digests_vec.iter_mut().for_each(|digest| {
                    let digest =
                        digest.as_object_mut().expect("validated as object in prior conditional");

                    digest.iter_mut().for_each(|(_key, hash_value)| {
                        let base64_encoded_hash =
                            hash_value.as_str().expect("validated as string in prior conditional");
                        let hash = base64::decode(base64_encoded_hash)
                            .expect("invalid base64 digest hash");
                        *hash_value = serde_json::Value::String(hex::encode(hash));
                    });
                });
            }

            // Recursively simplify nested objects
            for (_, v) in map.iter_mut() {
                simplify_value(v);
            }
        }
        serde_json::Value::Array(arr) => {
            // Recursively simplify values within the array
            for v in arr.iter_mut() {
                simplify_value(v);
            }
        }
        _ => {} // No simplification needed for other types
    }
}

fn main() {
    let mut extracted_evidence = {
        let Params { evidence } = Params::parse();
        let evidence = {
            let serialized = fs::read(evidence).expect("could not read evidence");
            Evidence::decode(serialized.as_slice()).expect("could not decode evidence")
        };

        oak_attestation_verification::verifier::extract_evidence(&evidence).unwrap()
    };

    let reference_values =
        oak_attestation_verification_test_utils::reference_values_from_evidence(extracted_evidence);

    let mut v = serde_json::to_value(reference_values).unwrap();

    print!("{}", serde_json::to_string_pretty(&v).expect("could not print reference values"));

    simplify_value(&mut v);
    print!("{}", serde_json::to_string_pretty(&v).expect("could not print reference values"));
}
