# Bevy Hourglass

[![Bevy Hourglass](https://github.com/edouardpoitras/bevy_hourglass/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/edouardpoitras/bevy_hourglass/actions/workflows/rust.yml)
[![Latest version](https://img.shields.io/crates/v/bevy_hourglass.svg)](https://crates.io/crates/bevy_hourglass)
[![Documentation](https://docs.rs/bevy_hourglass/badge.svg)](https://docs.rs/bevy_hourglass)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache](https://img.shields.io/badge/license-Apache-blue.svg)

A flexible hourglass plugin for Bevy applications.

**Website:** [https://edouardpoitras.github.io/bevy_hourglass/](https://edouardpoitras.github.io/bevy_hourglass/)

![animation](examples/auto_flip_mayhem.gif)

## Features

- Customizable visual hourglass timer with detailed mesh geometry
- Multiple hourglass styles: straight-sided, curved bulbs, various neck styles
- Flexible builder pattern for easy configuration
- Auto-flip functionality for continuous animation
- Configurable flip animations with custom durations
- Events for state changes (flipping, emptying)
- WebAssembly (WASM) support

## Examples

### Basic 2D Mesh Hourglass

Run the basic 2D mesh hourglass example:

```bash
cargo run --example 2d_mesh_hourglass
```

### Auto-Flip Mayhem

See multiple hourglasses with random configurations and auto-flipping:

```bash
cargo run --example auto_flip_mayhem
```

### Interactive Flip Demo

Control hourglass flipping with keyboard input:

```bash
cargo run --example flip_demo
```

### Curve Styles Demo

Explore different hourglass shapes and styles:

```bash
cargo run --example curve_styles_demo
```

### WebAssembly Examples

This project includes WebAssembly support, allowing you to run hourglasses in a web browser.

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
- Build examples for the WASM target (currently includes `2d_mesh_hourglass` and `auto_flip_mayhem`)
- Generate JavaScript bindings

#### Running the WASM Examples

After building, you can serve the WASM files with a local HTTP server:

Using Python's built-in HTTP server:

```bash
cd wasm && python -m http.server 8080
```

Then open http://localhost:8080 in your web browser to access the available examples.

## Usage

```rust
use bevy::prelude::*;
use bevy_hourglass::{
    BulbStyle, HourglassMeshBodyConfig, HourglassMeshBuilder, HourglassMeshPlatesConfig,
    HourglassMeshSandConfig, HourglassPlugin, NeckStyle,
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

    // Create a 2D mesh hourglass with detailed geometry using the new builder pattern
    HourglassMeshBuilder::new(Transform::from_xyz(0.0, 0.0, 0.0))
        .with_body(HourglassMeshBodyConfig {
            total_height: 200.0,
            bulb_style: BulbStyle::Circular {
                curvature: 1.0,
                width_factor: 0.75,
                curve_resolution: 20,
            },
            neck_style: NeckStyle::Curved {
                curvature: 0.2,
                width: 12.0,
                height: 8.0,
                curve_resolution: 5,
            },
            color: Color::srgba(0.85, 0.95, 1.0, 0.2),
        })
        .with_plates(HourglassMeshPlatesConfig {
            width: 165.0,
            height: 10.0,
            color: Color::srgb(0.6, 0.4, 0.2),
        })
        .with_sand(HourglassMeshSandConfig {
            color: Color::srgb(0.9, 0.8, 0.6),
            fill_percent: 1.0,  // Start with full top bulb
            wall_offset: 8.0,   // Distance from glass walls
        })
        .with_timing(Duration::from_secs(30)) // 30-second timer for automatic animation
        .with_auto_flip(true)                 // Enable auto-flipping when empty
        .with_flip_duration(0.5)              // 0.5 second flip animation
        .build(&mut commands, &mut meshes, &mut materials);
}
```

## Bevy Compatibility

|bevy|bevy_hourglass|
|---|---|
|0.16|0.2|

## License

MIT OR Apache-2.0
