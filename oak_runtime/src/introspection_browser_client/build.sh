#!/usr/bin/env bash

# Install npm modules
npm ci

# Generate JavaScript code from the introspection_events proto
mkdir -p proto
protoc --proto_path=../../../proto --js_out=import_style=commonjs,binary:proto ../../../proto/introspection_events.proto

# Build JavaScript bundle
npx webpack
