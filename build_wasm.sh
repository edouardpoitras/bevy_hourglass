#!/bin/bash
set -e

# Print status message
echo "Building bevy_hourglass for WASM..."

# Ensure wasm-bindgen CLI is installed
WASM_BINDGEN_PATH=""
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
    
    # Check if wasm-bindgen is now in PATH
    if command -v wasm-bindgen &> /dev/null; then
        echo "wasm-bindgen-cli installed successfully and found in PATH."
        WASM_BINDGEN_PATH="wasm-bindgen"
    else
        # If not in PATH, use the absolute path to the binary
        CARGO_BIN_PATH="$HOME/.cargo/bin/wasm-bindgen"
        if [ -x "$CARGO_BIN_PATH" ]; then
            echo "wasm-bindgen-cli installed but not in PATH. Using absolute path."
            WASM_BINDGEN_PATH="$CARGO_BIN_PATH"
        else
            echo "Error: wasm-bindgen-cli installation failed or binary not found at expected location."
            echo "Please ensure ~/.cargo/bin is in your PATH or install wasm-bindgen-cli manually."
            exit 1
        fi
    fi
else
    WASM_BINDGEN_PATH="wasm-bindgen"
    echo "wasm-bindgen-cli already installed."
fi

# Make sure the wasm32 target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Adding wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Create build directory if it doesn't exist
mkdir -p wasm

# Build the example for wasm32 target
echo "Building example for wasm32 target..."
cargo build --example simple --target wasm32-unknown-unknown --release

# Generate JavaScript bindings
echo "Generating JavaScript bindings with wasm-bindgen..."
$WASM_BINDGEN_PATH --out-dir wasm --target web target/wasm32-unknown-unknown/release/examples/simple.wasm

echo "Build completed successfully!"
echo "The WASM bundle is now available in the ./wasm directory"
echo ""
echo "To run the example, serve the wasm directory with a local HTTP server, for example:"
echo "  cd wasm && python -m http.server 8080"
echo "Then open http://localhost:8080 in your browser"
