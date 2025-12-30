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
- **cargo-component**: `cargo install cargo-component`
- **WASM targets**: 
  - `rustup target add wasm32-unknown-unknown` (for renderer)
  - `rustup target add wasm32-wasip1` (for plugins)

## Project Structure

```
cognexus/
├── wit/                # WebAssembly Interface Type definitions
│   └── plugin.wit      # Plugin interface for types and nodes
├── backend/
│   ├── model/          # Node graph data structures & traits
│   ├── types/          # First-party data types (WASM component)
│   ├── nodes/          # First-party nodes (WASM component)
│   ├── renderer/       # WGPU rendering engine (WASM for browser)
│   ├── proto/          # Protobuf message definitions
│   └── common/         # Shared utilities
├── frontend/
│   └── cognexus/       # Blazor WebAssembly UI
├── cli/
│   └── inspect/        # CLI tool for inspecting plugin components
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

### Building Plugin WASM Components

First-party types and nodes are compiled as WASM Component Model components using `cargo-component`:

```bash
# Build types component
cargo component build -p cognexus-types

# Build nodes component
cargo component build -p cognexus-nodes
```

Output files:
- `target/wasm32-wasip1/debug/cognexus_types.wasm`
- `target/wasm32-wasip1/debug/cognexus_nodes.wasm`

These are **WASM Components** (not raw WASM modules), implementing the WIT interfaces defined in `wit/plugin.wit`.

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

## Inspecting Plugin Components

The `cognexus-inspect` CLI tool allows you to interrogate WASM plugin components and discover their metadata.

### Usage

```bash
# Inspect a types plugin
cargo run -p cognexus-inspect -- <path-to-wasm> --kind types

# Inspect a nodes plugin
cargo run -p cognexus-inspect -- <path-to-wasm> --kind nodes
```

### Examples

**Inspecting types:**
```bash
cargo run -p cognexus-inspect -- target/wasm32-wasip1/debug/cognexus_types.wasm --kind types
```

Output:
```
Found 1 data type(s):
  - Signal (989bcbb2-b1a1-4f3f-be15-22ada278aedc)
    Description: A flow control signal with no data payload
    Version: 0.1.0
```

**Inspecting nodes:**
```bash
cargo run -p cognexus-inspect -- target/wasm32-wasip1/debug/cognexus_nodes.wasm --kind nodes
```

Output:
```
Found 2 node(s):
  - Start (40ebe0be-d2db-4eed-80f3-91267352ee42)
    Description: Initiates workflow execution
    Version: 0.1.0
    Input ports: 0
    Output ports: 1
  - End (e7a20e26-27ce-4d49-9759-50db835d46e6)
    Description: Terminates workflow execution
    Version: 0.1.0
    Input ports: 1
    Output ports: 0
```

### What the CLI Does

The inspect tool:
1. Loads the WASM component using wasmtime's Component Model support
2. Instantiates the component with WASI
3. Calls the appropriate discovery function (`list-types` or `list-nodes`)
4. Displays metadata including UUIDs, names, descriptions, versions, and port information

This demonstrates the plugin discovery mechanism that will be used by the desktop app to load plugins at runtime.

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

### Adding New Data Types
```bash
# 1. Edit backend/types/src/
# 2. Rebuild types component
cargo component build -p cognexus-types
# 3. Inspect the component to verify changes
cargo run -p cognexus-inspect -- target/wasm32-wasip1/debug/cognexus_types.wasm --kind types
# 4. Type components will be loaded at runtime (TODO: implement loader)
```

### Adding New Node Types
```bash
# 1. Edit backend/nodes/src/
# 2. Rebuild node component
cargo component build -p cognexus-nodes
# 3. Inspect the component to verify changes
cargo run -p cognexus-inspect -- target/wasm32-wasip1/debug/cognexus_nodes.wasm --kind nodes
# 4. Node components will be loaded at runtime (TODO: implement loader)
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

2. Add WIT component metadata to your `Cargo.toml`:
```toml
[package.metadata.component]
package = "cognexus:plugin"
target = { path = "../../wit", world = "nodes-plugin" }
```

3. Implement the Component Model guest trait in your crate's `lib.rs`:
```rust
mod bindings;
use bindings::exports::cognexus::plugin::nodes::Guest;

struct Component;

impl Guest for Component {
    fn list_nodes() -> Vec<NodeInfo> {
        // Return metadata for your nodes
    }
}

bindings::export!(Component with_types_in bindings);
```

4. Compile to WASM Component:
```bash
cargo component build -p your-node-crate
```

5. Inspect to verify:
```bash
cargo run -p cognexus-inspect -- target/wasm32-wasip1/debug/your_node_crate.wasm --kind nodes
```

6. The runtime will load your component at runtime (loader implementation in progress).

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
