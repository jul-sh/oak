#!/bin/bash
#
# Temporary script to deploy the Oak client to gstatic.
# This will be replaced later with automated deployments.

set -e

readonly WORKSPACE_ROOT="$(bazel info workspace)"
readonly OUTPUT_DIR="oak_attestation_explain_wasm/pkg"
readonly GSTATIC_PATH="__testing__/oak/wasm_client"

# Navigate to the Bazel workspace root.
cd "${WORKSPACE_ROOT}"

just oak_attestation_explain_wasm

# Results will be available at https://www.gstatic.com/__testing__/oak/wasm_client
admin_session --reason="b/359541656" -- \
  /google/bin/releases/static-content/mpms/scs_client_tools/scs_client_tools/scs \
  push -r "${OUTPUT_DIR}" "/external_content/gstatic/${GSTATIC_PATH}"
