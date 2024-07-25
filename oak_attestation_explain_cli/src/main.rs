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

use std::{fs, path::PathBuf};

use clap::Parser;
use oak_proto_rust::oak::attestation::v1::Evidence;
use prost::Message;

#[derive(Parser, Debug)]
#[group(skip)]
pub struct Params {
    /// Path to the evidence to inspect.
    #[arg(long, value_parser = path_exists, default_value = "oak_attestation_explain/testdata/rk_evidence.binarypb")]
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

fn main() {
    let Params { evidence } = Params::parse();
    let evidence = {
        let serialized = fs::read(evidence).expect("could not read evidence");
        Evidence::decode(serialized.as_slice()).expect("could not decode evidence")
    };
    println!(
        "{}",
        oak_attestation_explain_cli::explain_evidence(evidence).expect("failed to write to stdout")
    );
}
