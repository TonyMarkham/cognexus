# Cognexus

A visual workflow automation tool with a WebGL-rendered node graph editor. Built with Rust + WGPU for high-performance rendering and a Blazor WebAssembly frontend for UI controls.

## Architecture

```
┌──────────────────────────────┐
│         Front End            │
│  Blazor WebAssembly UI       │
│  - Panels, Buttons           │
│  - Inspectors, Controls      │
└───────────────┬──────────────┘
                │ Commands & Events (Protobuf)
┌───────────────▼──────────────┐
│      Render Engine Core       │
│           (Rust)              │
│  - Node graph data model      │
│  - Geometry generation        │
│  - Camera & interaction       │
└───────────────┬──────────────┘
                │ GPU abstraction
┌───────────────▼──────────────┐
│          WGPU backend         │
│  - Native (Vulkan/Metal/DX)   │
│  - Web (WebGPU)               │
└──────────────────────────────┘
```

**Tech Stack:**
- **Frontend**: Blazor WebAssembly (UI controls, panels, inspectors)
- **Backend**: Rust (node graph data model, rendering engine)
- **Rendering**: WGPU (cross-platform GPU abstraction)
- **Desktop**: Tauri 2.5 (embeds Blazor in native webview)
- **Communication**: Protocol Buffers (Blazor ↔ Rust)
- **Plugins**: WASM-based node definitions (first-party and third-party)

## Prerequisites

- **Rust** toolchain (latest stable via [rustup](https://rustup.rs/))
- **.NET 9.0 SDK** or later ([download](https://dotnet.microsoft.com/download))
- **Node.js** (for Tauri CLI)
- **Tauri CLI**: `cargo install tauri-cli@^2.0.0`
- **wasm-bindgen-cli**: `cargo install wasm-bindgen-cli`
- **WASM target**: `rustup target add wasm32-unknown-unknown`

## Project Structure

```
cognexus/
├── backend/
│   ├── model/          # Node graph data structures & traits
│   ├── types/          # First-party data types (compiled to WASM)
│   ├── nodes/          # First-party nodes (compiled to WASM)
│   ├── renderer/       # WGPU rendering engine
│   ├── proto/          # Protobuf message definitions
│   └── common/         # Shared utilities
├── frontend/
│   └── cognexus/       # Blazor WebAssembly UI
└── apps/
    └── desktop/
        └── cognexus/   # Tauri desktop app
```

## Initial Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/cognexus.git
cd cognexus

# Install Rust dependencies
cargo build

# Install .NET dependencies
cd frontend/cognexus
dotnet restore
cd ../..
```

## Build Instructions

### Full Build (Desktop App)

To build and run the desktop application:

```bash
# Step 1: Build the renderer as WASM
cargo build --target wasm32-unknown-unknown -p cognexus-renderer

# Step 2: Generate JavaScript bindings for the renderer
wasm-bindgen target/wasm32-unknown-unknown/debug/cognexus_renderer.wasm \
  --out-dir frontend/cognexus/wwwroot/wasm \
  --target web

# Step 3: Publish Blazor frontend (copies to Tauri frontend directory)
cd frontend/cognexus
dotnet publish -c Release
cd ../..

# Step 4: Run the desktop app
cargo tauri dev
```

### Building Plugin WASM Modules

First-party types and nodes are compiled as separate WASM modules:

```bash
# Build types WASM
cargo build --target wasm32-unknown-unknown -p cognexus-types

# Build nodes WASM
cargo build --target wasm32-unknown-unknown -p cognexus-nodes
```

Output files:
- `target/wasm32-unknown-unknown/debug/cognexus_types.wasm`
- `target/wasm32-unknown-unknown/debug/cognexus_nodes.wasm`

### Production Build

```bash
# Build renderer with optimizations
cargo build --target wasm32-unknown-unknown --release -p cognexus-renderer

# Generate JS bindings
wasm-bindgen target/wasm32-unknown-unknown/release/cognexus_renderer.wasm \
  --out-dir frontend/cognexus/wwwroot/wasm \
  --target web

# Publish Blazor
cd frontend/cognexus
dotnet publish -c Release
cd ../..

# Build production desktop app
cargo tauri build
```

Production binary locations:
- **macOS**: `./target/release/bundle/macos/Cognexus.app`
- **Windows**: `./target/release/bundle/msi/`
- **Linux**: `./target/release/bundle/appimage/`

## Running the Desktop App

### Development Mode
```bash
cargo tauri dev
```
Hot-reloads on Rust and Blazor changes.

### Production Binary
```bash
# macOS
./target/release/bundle/macos/Cognexus.app/Contents/MacOS/cognexus-desktop

# Windows
./target/release/bundle/msi/Cognexus.msi

# Linux
./target/release/bundle/appimage/cognexus.AppImage
```

## Development Workflow

### Making Changes to the Renderer
```bash
# 1. Edit Rust code in backend/renderer/
# 2. Rebuild WASM and bindings
cargo build --target wasm32-unknown-unknown -p cognexus-renderer
wasm-bindgen target/wasm32-unknown-unknown/debug/cognexus_renderer.wasm \
  --out-dir frontend/cognexus/wwwroot/wasm \
  --target web
# 3. Republish Blazor
cd frontend/cognexus && dotnet publish -c Release && cd ../..
# 4. Restart Tauri
cargo tauri dev
```

### Making Changes to the Node Graph Model
```bash
# Changes to backend/model/ are automatically picked up by cargo tauri dev
# Just save your changes and Tauri will rebuild
```

### Adding New Node Types
```bash
# 1. Edit backend/nodes/src/
# 2. Rebuild node WASM
cargo build --target wasm32-unknown-unknown -p cognexus-nodes
# 3. Node modules are loaded at runtime (TODO: implement loader)
```

## Plugin Development

See [AGENTS.md](./AGENTS.md) for detailed architecture documentation.

### Creating a Custom Node Type

1. Implement the `NodeDefinition` and `NodeDefinitionInfo` traits:
```rust
use cognexus_model::graph::{NodeDefinition, NodeDefinitionInfo};

pub struct MyCustomNode;

impl NodeDefinitionInfo for MyCustomNode {
    fn definition_id(&self) -> Uuid { /* ... */ }
    fn name(&self) -> &str { "My Node" }
    fn input_port_specs(&self) -> Vec<(&str, Uuid)> { /* ... */ }
    fn output_port_specs(&self) -> Vec<(&str, Uuid)> { /* ... */ }
    // ...
}

impl NodeDefinition for MyCustomNode {
    type Error = MyNodeError;
    fn execute(&self, inputs: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        // Your node logic here
    }
}
```

2. Compile to WASM:
```bash
cargo build --target wasm32-unknown-unknown -p your-node-crate
```

3. The runtime will load your WASM module (loader implementation pending).

## Testing

```bash
# Run all tests
cargo test

# Test specific crate
cargo test -p cognexus-model
cargo test -p cognexus-renderer

# Test with output
cargo test -- --nocapture
```

## Code Style

### Rust
```bash
# Format code
cargo fmt

# Lint code
cargo clippy
```

### C# / Blazor
```bash
# Format code
cd frontend/cognexus
dotnet format
```

## Troubleshooting

### WASM build fails with uuid errors
Add the `js` feature to uuid in `Cargo.toml`:
```toml
uuid = { version = "1.19.0", features = ["v4", "serde", "js"] }
```

### Blazor not loading renderer
Make sure you've run all three steps:
1. Build renderer WASM
2. Generate JS bindings with wasm-bindgen
3. Publish Blazor frontend

### Desktop app shows blank screen
Check that the Blazor frontend was published:
```bash
ls apps/desktop/cognexus/frontend/
```
Should contain index.html and related files.

## Contributing

This is a learning project. See [AGENTS.md](./AGENTS.md) for architectural decisions and development guidelines.

## License

See [LICENSE](./LICENSE) for details.
