# Bevy Hourglass

[![Bevy Hourglass](https://github.com/edouardpoitras/bevy_hourglass/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/edouardpoitras/bevy_hourglass/actions/workflows/rust.yml)
[![Latest version](https://img.shields.io/crates/v/bevy_hourglass.svg)](https://crates.io/crates/bevy_hourglass)
[![Documentation](https://docs.rs/bevy_hourglass/badge.svg)](https://docs.rs/bevy_hourglass)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache](https://img.shields.io/badge/license-Apache-blue.svg)

A flexible hourglass plugin for Bevy applications.

## Features

- Customizable visual hourglass timer
- Sprite-based rendering
- Configurable duration
- Events for state changes (flipping, emptying)
- WebAssembly (WASM) support

## Examples

### Simple Example

Run the simple example with:

```bash
cargo run --example simple
```

### WebAssembly Example

This project includes WebAssembly support for the simple example, allowing you to run the hourglass in a web browser.

#### Building for WASM

1. Make the build script executable (if not already):
   ```bash
   chmod +x build_wasm.sh
   ```

2. Run the build script:
   ```bash
   ./build_wasm.sh
   ```

This script will:
- Install the necessary tools (wasm-bindgen-cli) if not already installed
- Add the wasm32-unknown-unknown target if needed
- Build the example for the WASM target
- Generate JavaScript bindings

#### Running the WASM Example

After building, you can serve the WASM files with a local HTTP server:

Using Python's built-in HTTP server:

```bash
cd wasm && python -m http.server 8080
```

Then open http://localhost:8080 in your web browser.

## Usage

Add the dependency to your Cargo.toml:

```toml
[dependencies]
bevy_hourglass = "0.1.0"
```

In your Bevy application:

```rust
use bevy::prelude::*;
use bevy_hourglass::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HourglassPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a camera
    commands.spawn(Camera2d::default());
    
    // Spawn a hourglass
    spawn_hourglass(
        &mut commands,
        Duration::from_secs(60),  // 60 second timer
        Vec2::ZERO,              // Position at center
        Vec2::new(100.0, 200.0), // Size
        Color::srgb(0.8, 0.8, 0.8), // Container color
        Color::srgb(0.9, 0.7, 0.2)  // Sand color
    );
}
```

## Bevy Compatibility

|bevy|bevy_hourglass|
|---|---|
|0.16|0.1|

## License

MIT OR Apache-2.0
