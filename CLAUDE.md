# Claude Development Guidelines for Bevy Hourglass

This file contains instructions and preferences for Claude when working on the Bevy Hourglass project.

## Essential Requirements

### Code Quality
- **ALWAYS run `cargo fmt` before completing any work** to ensure consistent code formatting
- **ALWAYS run `cargo clippy` before completing any work** to catch common mistakes and improve code quality
- Run `cargo check` to verify code compiles without errors
- Run `cargo test` if tests exist to ensure functionality remains intact

### Project Structure
This is a Bevy plugin project with the following structure:
- `src/lib.rs` - Main library entry point
- `src/plugin.rs` - Bevy plugin implementation
- `src/components.rs` - ECS components
- `src/systems.rs` - Bevy systems
- `src/resources.rs` - ECS resources
- `src/events.rs` - Custom events
- `src/sprite_hourglass.rs` - Sprite-based hourglass implementation
- `src/mesh_hourglass.rs` - Mesh-based hourglass implementation
- `examples/` - Example applications demonstrating usage

### Bevy Compatibility
- Current Bevy version: **0.16.0**
- Follow Bevy's ECS patterns and conventions
- Use Bevy's new APIs and patterns (avoid deprecated APIs)
- Maintain compatibility with the current Bevy version

### Development Workflow
1. Make code changes following Rust and Bevy best practices
2. Run `cargo fmt` to format code
3. Run `cargo clippy` to check for issues
4. Run `cargo check` to verify compilation
5. Test examples to ensure functionality works as expected
6. Consider WASM compatibility when making changes

### Code Style Guidelines
- Follow standard Rust naming conventions
- Use descriptive variable and function names
- Add documentation comments for public APIs
- Keep functions focused and reasonably sized
- Prefer explicit types when it improves clarity

### Testing
- Run existing tests with `cargo test`
- Examples serve as integration tests - ensure they still work after changes
- Test WASM compatibility when possible using the build script

### WASM Support
- This project supports WebAssembly compilation
- Use the `build_wasm.sh` script for WASM builds
- Be mindful of WASM-specific dependencies and features
- Test WASM builds when making significant changes

### Dependencies
- Minimize external dependencies
- When adding dependencies, consider their impact on compile time and binary size
- Ensure new dependencies are compatible with WASM target
- Update Cargo.toml appropriately and follow semantic versioning

### Examples
- Keep examples simple and focused
- Ensure examples demonstrate key features clearly
- Test examples after making changes to verify they still work
- Examples include: `simplest.rs`, `simple_with_ui.rs`, `2d_mesh_hourglass.rs`

## Common Commands
- Format code: `cargo fmt`
- Check for issues: `cargo clippy`
- Build: `cargo build`
- Test: `cargo test`
- Run example: `cargo run --example [example_name]`
- Build for WASM: `./build_wasm.sh`

Remember: This is a plugin for game development with Bevy, so prioritize performance, usability, and clear APIs.