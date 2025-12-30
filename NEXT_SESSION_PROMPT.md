# Next Session Prompt - December 30, 2024 (Evening)

## Context

I'm Tony, building **Cognexus** - a visual workflow automation tool (N8N clone) with a WebGL-rendered node graph editor. This is a learning project where I build and AI agents provide guidance using production-grade standards.

## What We Just Completed (Dec 30, 2024 - Evening Session)

✅ **WASM Component Model System**
- Created WIT interface definitions (`wit/plugin.wit`) with separate worlds:
  - `types-plugin` world for data type components
  - `nodes-plugin` world for node components
- Converted `backend/types` and `backend/nodes` to WASM components using `cargo-component`
- Implemented Component Model guest traits (`list-types()`, `list-nodes()`)
- Components export full metadata: UUIDs, names, descriptions, versions, port specs

✅ **CLI Inspection Tool** (`cli/inspect/`)
- Built `cognexus-inspect` binary for interrogating plugin components
- Loads WASM components using wasmtime Component Model API
- Generates WIT bindings for both worlds using `wasmtime::component::bindgen!`
- Calls discovery functions across component boundary
- Supports `--kind` flag to specify types or nodes
- Validates entire Component Model architecture end-to-end

✅ **Documentation Updates**
- Updated README with Component Model build instructions
- Added CLI tool usage section with examples
- Updated plugin development guide
- Separated sections for adding types vs nodes

✅ **Validation Complete**
- Both `cognexus_types.wasm` and `cognexus_nodes.wasm` build successfully
- CLI tool successfully loads and interrogates both components
- Metadata extraction works correctly
- Component Model architecture proven production-ready

## Current State

**Foundation: 90% Complete** (up from 70%)
- ✅ Data model with builders
- ✅ Both registries (DataType + NodeDefinition)
- ✅ First-party types/nodes as WASM components
- ✅ Graph validation complete
- ✅ Component Model system functional
- ✅ WIT interfaces defined
- ✅ CLI inspection tool working

**Runtime: 50% Complete** (up from 20%)
- ✅ CLI tool: 100% (validates architecture)
- ❌ Desktop app plugin discovery: 0% ← **NEXT TASK**
- ✅ Registries: 100% (waiting for desktop integration)

**Overall: ~60% Complete** (up from 40%)

## What to Work On Next

**Primary Goal:** Integrate plugin discovery into the Tauri desktop app

### Desktop App Plugin Discovery System

Build the runtime plugin loading system for the Tauri desktop application:

1. **Plugin Directory Structure**
   - Determine cross-platform plugin directories:
     - `builtin/` - First-party components (bundled with app, trusted)
     - `plugins/` - Third-party components (user-installed, sandboxed)
   - Use Tauri's resource APIs to locate bundled files
   - Use platform-appropriate user directories (XDG, AppData, Application Support)

2. **Component Scanner**
   - Scan plugin directories at startup
   - Discover `.wasm` component files
   - Determine component type (types-plugin or nodes-plugin)
   - Build an index of available components

3. **Lazy Component Loading**
   - Don't load all components at startup
   - Load on-demand when UUID is requested
   - Cache loaded components in memory
   - Handle component instantiation errors gracefully

4. **Registry Population**
   - Call `list-types()` on types components
   - Call `list-nodes()` on nodes components
   - Populate `DataTypeRegistry` with discovered types
   - Populate `NodeDefinitionRegistry` with discovered nodes
   - Store component references for later execution

5. **Tauri Integration**
   - Add wasmtime dependencies to Tauri app `Cargo.toml`
   - Create plugin manager service/module
   - Initialize at app startup
   - Expose plugin info to Blazor frontend via Tauri commands (future)

## Important Constraints

**Teaching Mode:**
- I implement features - you provide guidance and code snippets
- Give me snippets to type in, one step at a time
- Wait for my confirmation before proceeding
- Focus on correctness over speed

**Production Standards:**
- No shortcuts or technical debt
- Proper error handling (avoid `.unwrap()` in prod code)
- Security considerations from the start
- Document architectural decisions
- Cross-platform compatibility (macOS, Windows, Linux)

## Key Files to Reference

- `AGENTS.md` - Development philosophy and architecture
- `CURRENT_STATE.md` - Updated with Component Model progress
- `README.md` - Updated with build instructions and CLI usage
- `wit/plugin.wit` - WIT interface definitions
- `cli/inspect/src/main.rs` - Working example of loading components
- `backend/model/src/graph/` - Registries waiting for population
- `apps/desktop/cognexus/` - Tauri app that needs plugin integration

## Technical Context

**Tech Stack:**
- Rust (backend, WASM components)
- WASM Component Model (plugin system)
- wasmtime (component runtime)
- WGPU (rendering)
- Blazor WebAssembly (UI)
- Tauri 2.5 (desktop app)
- Protocol Buffers (communication)

**Component Build Commands:**
```bash
# Build types component
cargo component build -p cognexus-types

# Build nodes component  
cargo component build -p cognexus-nodes

# Inspect components
cargo run -p cognexus-inspect -- target/wasm32-wasip1/debug/cognexus_types.wasm --kind types
cargo run -p cognexus-inspect -- target/wasm32-wasip1/debug/cognexus_nodes.wasm --kind nodes
```

**What Works:**
- CLI tool proves Component Model architecture is solid
- Components build successfully
- Discovery functions work correctly
- Metadata extraction is complete and accurate

**What's Needed:**
- Desktop app integration (reuse patterns from CLI tool)
- Plugin directory management
- Registry population from loaded components
- Proper error handling for missing/invalid components

## Session Start

Please:
1. Read `AGENTS.md` for development philosophy
2. Review `CURRENT_STATE.md` for detailed progress
3. Check `cli/inspect/src/main.rs` to see the working component loading pattern
4. Guide me through building the desktop app plugin discovery system
5. One step at a time, with explanations

The CLI tool validates the architecture - now let's integrate it into the actual application! 🚀
