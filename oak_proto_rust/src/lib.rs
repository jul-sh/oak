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

// This module provides a uniform way to import crypto protos regardless of
// building with cargo or bazel.

// Prost generated code cannot build on its own: it needs to be
// included into a manually crafted module structure. With crypto_rust_prost,
// this is not needed.

// TODO: b/333064338 - Remove this crate once we've stopped using cargo.

#![no_std]
#![feature(never_type)]

// Inlined from tonic::include_proto in order to cut dependency on tonic.
macro_rules! include_proto {
    ($package: tt) => {
        include!(concat!(env!("OUT_DIR"), concat!("/", $package, ".rs")));
    };
}

pub mod oak {
    include_proto!("oak");

    pub mod attestation {
        pub mod v1 {
            #![allow(clippy::large_enum_variant)]
            include_proto!("oak.attestation.v1");

            // A few messages use prost's the one_of directive. The rust struct
            // resulting rust struct structure is a little hard to read. Since we
            // want to print the debug message for these messages as part of human
            // readable attestation explaination, we implement a simpler debug
            // output ourselves. This debug implementation direclty prints the
            // relevant value.
            use core::fmt::{self, Debug};

            impl Debug for BinaryReferenceValue {
                fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match &self.r#type {
                        Some(r#type) => match r#type {
                            binary_reference_value::Type::Skip(skip_verification) => {
                                Debug::fmt(&skip_verification, fmt)
                            }
                            binary_reference_value::Type::Endorsement(endorsements) => {
                                Debug::fmt(&endorsements, fmt)
                            }
                            binary_reference_value::Type::Digests(digests) => {
                                write!(fmt, "{:?}", digests)
                            }
                        },
                        None => write!(fmt, "None"),
                    }
                }
            }

            impl Debug for KernelBinaryReferenceValue {
                fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match &self.r#type {
                        Some(r#type) => match r#type {
                            kernel_binary_reference_value::Type::Skip(skip_verification) => {
                                Debug::fmt(&skip_verification, fmt)
                            }
                            kernel_binary_reference_value::Type::Endorsement(endorsement) => {
                                Debug::fmt(&endorsement, fmt)
                            }
                            kernel_binary_reference_value::Type::Digests(digests) => {
                                Debug::fmt(&digests, fmt)
                            }
                        },
                        None => write!(fmt, "None"),
                    }
                }
            }

            impl Debug for TextReferenceValue {
                fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match &self.r#type {
                        Some(r#type) => match r#type {
                            text_reference_value::Type::Skip(skip_verification) => {
                                Debug::fmt(&skip_verification, fmt)
                            }
                            text_reference_value::Type::Endorsement(endorsements) => {
                                Debug::fmt(&endorsements, fmt)
                            }
                            text_reference_value::Type::Regex(regex) => Debug::fmt(&regex, fmt),
                            text_reference_value::Type::StringLiterals(string_literal) => {
                                Debug::fmt(&string_literal, fmt)
                            }
                        },
                        None => write!(fmt, "None"),
                    }
                }
            }

            impl Debug for ReferenceValues {
                fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match &self.r#type {
                        Some(r#type) => match r#type {
                            reference_values::Type::OakRestrictedKernel(skip_verification) => {
                                Debug::fmt(&skip_verification, fmt)
                            }
                            reference_values::Type::OakContainers(endorsement) => {
                                Debug::fmt(&endorsement, fmt)
                            }
                            reference_values::Type::Cb(digests) => Debug::fmt(&digests, fmt),
                        },
                        None => write!(fmt, "None"),
                    }
                }
            }

            #[cfg(test)]
            mod tests {
                extern crate alloc;
                use alloc::{format, string::String};

                use super::*;

                #[test]
                fn expected_debug_messages() {
                    assert_eq!(
                        "StringLiterals { value: [\"foo\"] }",
                        format!(
                            "{:?}",
                            TextReferenceValue {
                                r#type: Some(text_reference_value::Type::StringLiterals(
                                    StringLiterals { value: alloc::vec![String::from("foo",)] }
                                )),
                            }
                        )
                    );
                    assert_eq!(
                        "SkipVerification",
                        format!(
                            "{:?}",
                            BinaryReferenceValue {
                                r#type: Some(binary_reference_value::Type::Skip(
                                    SkipVerification {}
                                )),
                            }
                        )
                    );
                }
            }
        }
    }

    pub mod crypto {
        pub mod v1 {
            #![allow(dead_code)]
            include_proto!("oak.crypto.v1");
        }
    }

    pub mod oak_functions {
        pub mod abi {
            include_proto!("oak.functions.abi");
        }
        pub mod lookup_data {
            include_proto!("oak.functions.lookup_data");
        }
        pub mod testing {
            use prost::Message;
            include_proto!("oak.functions.testing");
        }
    }
}
