# Copyright 2020 The Bazel Authors. All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

load("@rules_rust//rust:defs.bzl", "rust_shared_library")
load("@rules_rust//wasm_bindgen:defs.bzl", "rust_wasm_bindgen")

package(
    default_visibility = ["//visibility:public"],
    licenses = ["notice"],
)

rust_shared_library(
    name = "oak_attestation_explain_wasm",
    srcs = glob(["src/*.rs"]),
    platform = "//:wasm32-unknown-unknown",
    deps = [
        "//oak_attestation_explain",
        "//oak_attestation_verification",
        "//oak_proto_rust",
        "@oak_crates_index//:prost",
        "@oak_crates_index//:wasm-bindgen",
    ],
)

exports_files(["index.html"])

rust_wasm_bindgen(
    name = "oak_attestation_explain_lib_wasm_bindgen",
    bindgen_flags = ["--browser"],
    target = "web",
    wasm_file = ":oak_attestation_explain_wasm",
)
