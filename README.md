# Bevy Hourglass

[![Bevy Hourglass](https://github.com/edouardpoitras/bevy_hourglass/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/edouardpoitras/bevy_hourglass/actions/workflows/rust.yml)
[![Latest version](https://img.shields.io/crates/v/bevy_hourglass.svg)](https://crates.io/crates/bevy_hourglass)
[![Documentation](https://docs.rs/bevy_hourglass/badge.svg)](https://docs.rs/bevy_hourglass)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache](https://img.shields.io/badge/license-Apache-blue.svg)

A flexible hourglass plugin for Bevy applications.

**Website:** [https://github.com/edouardpoitras/bevy_hourglass](https://github.com/edouardpoitras/bevy_hourglass)

## Features

- Customizable visual hourglass timer
- Sprite-based rendering
- 2D mesh-based hourglass with detailed geometry
- Configurable duration
- Events for state changes (flipping, emptying)
- WebAssembly (WASM) support

## Examples

### 2D Mesh Hourglass Example

Run the 2D mesh hourglass example with:

```bash
cargo run --example 2d_mesh_hourglass
```

### WebAssembly Example

This project includes WebAssembly support for the 2D mesh hourglass example, allowing you to run the hourglass in a web browser.

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
- Build the 2D mesh hourglass example for the WASM target
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
use bevy_hourglass::{
    HourglassMeshBodyConfig, HourglassMeshBuilder, HourglassMeshPlatesConfig,
    HourglassMeshSandConfig, HourglassPlugin,
};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HourglassPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());

    // Create a 2D mesh hourglass with detailed geometry
    HourglassMeshBuilder::new(Transform::from_xyz(0.0, 0.0, 0.0))
        .with_body(HourglassMeshBodyConfig {
            total_height: 200.0,
            bulb_radius: 100.0,
            bulb_width_factor: 0.75,
            bulb_height_factor: 1.0,
            bulb_curve_resolution: 20,
            neck_width: 12.0,
            neck_height: 7.0,
            neck_curve_resolution: 5,
            color: Color::srgba(0.85, 0.95, 1.0, 0.2),
        })
        .with_plates(HourglassMeshPlatesConfig {
            width: 165.0,
            height: 10.0,
            color: Color::srgb(0.6, 0.4, 0.2),
        })
        .with_sand(HourglassMeshSandConfig {
            color: Color::srgb(0.9, 0.8, 0.6),
            fill_percent: 1.0,       // Start with full top bulb
            scale_factor: 0.95,      // Sand is 95% of glass size
            neck_scale_factor: 0.35, // Sand is 35% of neck size
        })
        .with_timing(Duration::from_secs(30)) // 30-second timer for automatic animation
        .build(&mut commands, &mut meshes, &mut materials);
}
```

## Bevy Compatibility

|bevy|bevy_hourglass|
|---|---|
|0.16|0.1|

## License

MIT OR Apache-2.0
