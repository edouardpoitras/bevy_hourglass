# Bevy Hourglass WASM Demo

This directory contains the WebAssembly build of the Bevy Hourglass demo.

## Live Demo

Visit the live demo at: https://edouardpoitras.github.io/bevy_hourglass/

## Local Development

To build and run the WASM demo locally:

1. **Prerequisites:**
   - Rust with `wasm32-unknown-unknown` target
   - `wasm-bindgen-cli` (automatically installed by build script)

2. **Build the WASM bundle:**
   ```bash
   # From the repository root
   ./build_wasm.sh