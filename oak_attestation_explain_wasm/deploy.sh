#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

readonly WORKSPACE_ROOT=$(bazel info workspace)
readonly OUTPUT_DIR=oak_attestation_explain_wasm/pkg

# Navigating to the Bazel workspace root
cd "$WORKSPACE_ROOT"

admin_session --reason="b/359541656" -- /google/bin/releases/static-content/mpms/scs_client_tools/scs_client_tools/scs push -r "${OUTPUT_DIR}" /external_content/gstatic/__testing__/oak/wasm_client/

