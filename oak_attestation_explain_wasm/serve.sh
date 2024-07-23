#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Navigating to the Bazel workspace root.
WORKSPACE_ROOT=$(bazel info workspace)
cd "$WORKSPACE_ROOT"

build_and_copy() {
    echo "INFO: Building the project. $(date +%H:%M:%S)"
    temp_log=$(mktemp)
    if ! bazel build //oak_attestation_explain_wasm:oak_attestation_explain_wasm > "$temp_log" 2>&1; then
        echo "ERROR: Build failed. Logs:"
        cat "$temp_log"
        exit 1
    fi
    cp oak_attestation_explain_wasm/index.html bazel-bin/oak_attestation_explain_wasm/
    echo "INFO: Build and copy complete. $(date +%H:%M:%S)"
}

build_and_copy

echo "INFO: Starting a HTTP server in the output directory"

# Watch for changes in source files and rebuild if needed.
previous_modification_times=()
while true; do
    current_modification_times=()
    for file in oak_attestation_explain_wasm/**/*; do
        current_modification_times+=($(stat -c %Y "$file"))
    done

    if [[ "$previous_modification_times" != "${current_modification_times[*]}" ]]; then
        echo "INFO: Changes detected, rebuilding... $(date +%H:%M:%S)"
        build_and_copy
        previous_modification_times=("${current_modification_times[@]}")
    fi

    sleep 1
done &

python3 -m http.server --directory bazel-bin/oak_attestation_explain_wasm 0
