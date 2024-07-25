#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Define workspace root, crate directory, and output directory.
readonly WORKSPACE_ROOT=$(bazel info workspace)
readonly CRATE_DIR=oak_containers_examples/hello_world/web_client

# Navigate to the Bazel workspace root.
cd "$WORKSPACE_ROOT"

echo "INFO: Serving the project using trunk"
(cd "${CRATE_DIR}" && trunk serve)
