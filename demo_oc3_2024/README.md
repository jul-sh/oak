# Evidence Verification Demo

This demo reproducably builds the entire software stack powering the deployment
of an Oak Enclave Application (firmware, kernel, application) from source code.
The build artifacts are measured.

The resulting measurements are then used in the proccess of verifying DICE
attestation evidence returned by a prior deployment of this Oak Enclave
Application in a genuine AMD SEV-SNP TEE.

Matching the measurements of the local builds with those found in the
attestation evidence makes it possible to trace the deployment all the way back
to source code.

This process permits clients to meaningfully reason about the behavior of remote
workloads.

## Prerequisites

- Docker must be installed

## Running the demo

```sh
# First, pull the relevant builder docker image.
./scripts/docker_pull

# Second, start the docker container and enter the build shell.
./scripts/docker_run bash -c "nix develop"

# Lastly, run the demo script.
./demo_oc3_2024/get_measurements
```
