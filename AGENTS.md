# AGENTS.md

## Project Overview

Cognexus is a visual workflow automation tool (N8N clone) with a WebGL-rendered node graph editor. The core innovation is using Rust + WGPU for high-performance node graph rendering, with a Blazor WebAssembly frontend for UI controls.

**Key Goals:**
- Learn Rust, WGPU, and Tauri 2.5 through building
- Create a cross-platform node graph editor
- Deploy as desktop app first, web application later

## Architecture

```
┌──────────────────────────────┐
│         Front End(s)          │
│  Blazor / Web UI / Native UI  │
│  - Panels                     │
│  - Buttons                    │
│  - Inspectors                 │
│  - Styling / UX               │
└───────────────┬──────────────┘
                │ Commands & Events (Protobuf)
┌───────────────▼──────────────┐
│      Render Engine Core       │
│           (Rust)              │
│  - Scene graph / node graph   │
│  - Geometry generation        │
│  - Shader/material system     │
│  - Camera & interaction       │
│  - Picking / hit-testing      │
└───────────────┬──────────────┘
                │ GPU abstraction
┌───────────────▼──────────────┐
│          wgpu backend         │
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
- **Future**: Web deployment (Blazor + Rust both as WASM)

## Important Design Decisions

1. **This is NOT a client-server architecture** - No HTTP API layer. Blazor and Rust communicate via Protobuf messages over Tauri IPC (desktop) or JS interop (web).

2. **WGPU eliminates the JavaScript rendering layer** - The render engine is pure Rust, outputting directly to WebGL/WebGPU.

3. **Single binary for desktop** - Tauri bundles everything into one executable.

4. **Learning project** - The human developer (Tony) builds this. AI agents provide guidance, answer questions, and review code - but should NOT implement features unprompted.

## Directory Structure

```
cognexus/
├── AGENTS.md
├── LICENSE
├── README.md
├── Cargo.toml              # Workspace root
├── proto/                  # Protobuf message definitions
│   ├── commands.proto      # UI → Rust commands
│   └── events.proto        # Rust → UI events
├── backend/
│   ├── model/              # Node graph data structures
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── renderer/           # WGPU rendering engine
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── frontend/
│   └── cognexus/           # Blazor WebAssembly UI
│       ├── CognexusBlazor.csproj
│       ├── Program.cs
│       ├── Pages/
│       ├── Components/
│       └── wwwroot/
└── apps/
    └── desktop/
        └── cognexus/       # Tauri desktop app
            ├── Cargo.toml
            ├── tauri.conf.json
            ├── build.rs
            └── src/
                └── main.rs
```

## Setup Commands

**Prerequisites:**
- Rust toolchain (`rustup`)
- .NET 9.0 SDK
- Node.js (for Tauri CLI)
- Tauri CLI: `cargo install tauri-cli@^2.0.0`

**Initial setup:**
```bash
# Install Rust dependencies
cargo build

# Install .NET dependencies
cd frontend/cognexus && dotnet restore
```

## Build Commands

**Desktop (Tauri):**
```bash
cargo tauri dev          # Development mode
cargo tauri build        # Production build
```

**Web (future):**
```bash
cd frontend/cognexus
dotnet publish -c Release
# Serve wwwroot/ with any static file server
```

## Code Style

**Rust:**
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow standard Rust naming conventions (snake_case for functions, PascalCase for types)

**C# / Blazor:**
- Follow standard C# conventions (PascalCase for public members)
- Use `dotnet format` for formatting

**Protobuf:**
- Use snake_case for field names
- Clear, descriptive message names

## Testing Instructions

(To be added as tests are written)

## Communication Protocol

All communication between Blazor and Rust uses Protocol Buffers:

**Desktop mode:**
- Blazor serializes protobuf → sends via Tauri `invoke()` → Rust deserializes
- Rust serializes protobuf → sends via Tauri events → Blazor deserializes

**Web mode (future):**
- Same protobuf schemas, different transport (JS interop instead of Tauri IPC)

## Working with AI Agents

**Agent role:**
- Provide step-by-step guidance for production-grade development
- Answer questions about Rust, WGPU, Blazor, Tauri
- Review code and suggest improvements
- Explain concepts and best practices
- Teach professional, industry-standard patterns from the start
- **If you don't know something, be honest and research before answering - never guess**

**Production-grade standards (NOT negotiable):**
- Proper Rust workspace dependency management (centralized in root Cargo.toml with `workspace = true`)
- Clean architecture and separation of concerns
- Security considerations from day one
- Proper error handling (avoid liberal use of `.unwrap()`, use proper `Result` types)
- Configuration management that scales
- No shortcuts, no "good enough for now" code
- No technical debt accumulation
- Document architectural decisions and reasoning
- **Focus on correctness, not speed - doing things right is more important than doing things fast**

**Agent should NOT:**
- Build entire features without being asked
- Make assumptions about requirements
- Jump ahead without confirming the approach
- Generate large amounts of code unprompted
- Suggest toy examples or quick hacks
- Teach beginner patterns that need to be unlearned later
- Compromise on code quality for speed

**This is a learning project focused on professional development practices. The human builds, the agent guides with production-grade standards.**
