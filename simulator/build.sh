#!/bin/bash
set -e

echo "Building WASM module..."
wasm-pack build --target web

echo "Copying WASM artifacts to www directory..."
mkdir -p www/pkg
cp -r pkg/* www/pkg/

echo "Build complete! Run a local server with:"
echo "  ccargo run --bin serve --features serve"
echo "Then open http://localhost:8000"
