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

load("@bazel_skylib//rules:copy_file.bzl", "copy_file")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

rust_binary(
    name = "oak_attestation_explain_wasm",
    srcs = ["src/lib.rs"],
    crate_type = "cdylib",
    out_binary = True,
    platform = "//:wasm32-unknown-unknown",
    rustc_flags = ["-Cstrip=debuginfo"],
    visibility = ["//visibility:public"],
)

genrule(
    name = "copy_resources",
    srcs = [":oak_attestation_explain_wasm"],
    outs = ["oak_attestation_explain.wasm"],
    cmd = "cp $(SRCS) $(OUTS)",
)

genrule(
    name = "create_index_html",
    srcs = ["main.html"],
    outs = ["index.html"],
    cmd = "cp $(SRCS) $(OUTS)",
    visibility = ["//visibility:public"],
)
