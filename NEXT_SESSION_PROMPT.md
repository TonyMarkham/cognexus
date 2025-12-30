# Next Session Prompt - December 30, 2024

## Context

I'm Tony, building **Cognexus** - a visual workflow automation tool (N8N clone) with a WebGL-rendered node graph editor. This is a learning project where I build and AI agents provide guidance using production-grade standards.

## What We Just Completed (Dec 30, 2024)

✅ **DataTypeRegistry System**
- Split `DataType` trait into `DataTypeInfo` (metadata) and `DataType` (serialization)
- Implemented `DataTypeRegistry` following the `NodeDefinitionRegistry` pattern
- Both registries now complete and ready for WASM loader

✅ **Graph Validation System**
- `Graph::add_node()` now validates definition exists in `NodeDefinitionRegistry`
- `Graph::add_edge()` now validates ports exist on node definitions
- Full validation through registry dependency injection

✅ **Port System Refactor**
- Node definitions create actual `Port` instances (not just specs)
- Each port has a UUID from the definition (template)
- Node instance ID + port definition ID = unique connection point

✅ **WASM Loader Architecture Designed**
- Lazy loading: modules loaded on-demand, not at startup
- CLI tool (`cognexus inspect`) for discovering type/node UUIDs from WASM
- WASM as single source of truth (no hand-written manifests)
- Registration function pattern for plugin discovery
- See `CURRENT_STATE.md` section 1 for full design

## Current State

**Foundation: 70% Complete**
- ✅ Data model with builders
- ✅ Both registries (DataType + NodeDefinition)
- ✅ First-party types/nodes (Signal, Start, End) as WASM
- ✅ Graph validation complete

**Runtime: 20% Complete**
- ❌ WASM loader: 0% ← **NEXT BIG TASK**
- ✅ Registries: 100% (waiting for loader)

**Overall: ~40% Complete**

## What to Work On Next

**Primary Goal:** Build the WASM runtime and loader system

**Two parallel tracks:**

### Track A: CLI Tool (`cognexus inspect`)
Build a CLI tool that can:
1. Load a WASM module (using wasmtime or wasmer)
2. Call the registration function to discover types/nodes
3. Extract metadata by calling trait methods (`type_id()`, `name()`, etc.)
4. Output discovered UUIDs and metadata

**Why first:** Plugin authors (including third-party) need this to discover first-party type/node UUIDs. Testing this validates the discovery pattern works before building the full loader.

### Track B: Desktop App WASM Loader
Integrate WASM loading into the Tauri desktop app (`apps/desktop/cognexus/`):
1. Determine cross-platform plugin directory structure:
   - `builtin/` - First-party WASM (bundled with app, trusted)
   - `plugins/` - Third-party WASM (user-installed, sandboxed)
   - Use Tauri's resource APIs to locate bundled files cross-platform
   - Use platform-appropriate user directories for plugins (XDG on Linux, AppData on Windows, Application Support on macOS)
2. Scan for WASM files at startup (lightweight metadata scan)
3. Implement lazy loading (load WASM on-demand when UUID requested)
4. Registration function calls populate registries
5. Handle first-party (trusted) vs third-party (sandboxed) separately

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

## Key Files to Reference

- `AGENTS.md` - Development philosophy and architecture
- `CURRENT_STATE.md` - Detailed progress tracking and next steps
- `backend/model/src/graph/` - Data model and registries
- `backend/types/` - SignalType (example DataType implementation)
- `backend/nodes/` - StartNode, EndNode (example NodeDefinition implementations)

## Technical Context

**Tech Stack:**
- Rust (backend, WASM modules)
- WGPU (rendering)
- Blazor WebAssembly (UI)
- Tauri 2.5 (desktop app)
- Protocol Buffers (communication)

**WASM Compilation:**
```bash
# Build types
cargo build --target wasm32-unknown-unknown -p cognexus-types

# Build nodes  
cargo build --target wasm32-unknown-unknown -p cognexus-nodes

# Check model
cargo check -p cognexus-model
```

## Session Start

Please:
1. Read `AGENTS.md` for development philosophy
2. Review `CURRENT_STATE.md` section 1 (WASM Runtime & Loader) for the full design
3. Help me decide: Should we start with Track A (CLI tool) or Track B (desktop loader)?
4. Guide me through implementation one step at a time

Let's build the WASM loader system! 🚀
