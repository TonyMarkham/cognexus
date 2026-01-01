# ADR-0005: Flexible Plugin Architecture (Shell/Core Separation)

**Status:** Accepted  
**Date:** 2024-12-31  
**Deciders:** Tony  

## Context

Cognexus runs in two distinct environments with different constraints:

1.  **Desktop (Tauri):**
    - **Shell:** Native Rust (Tauri process).
    - **Frontend:** WASM (WebView). Runs Renderer & UI.
    - **Backend:** Native Rust (Tauri Host). Runs Plugin Loader & Master Graph State.
    - **Constraint:** Frontend WASM cannot load plugins directly (no filesystem, no JIT).

2.  **Web/VPS (Axum):**
    - **Shell:** Native Rust (Axum server).
    - **Frontend:** Remote Browser (WASM).
    - **Backend:** Native Rust (Server Process).
    - **Constraint:** Must share architecture with Desktop.

**The Requirements:**
1.  **Unified Discovery:** Plugins are discovered from the filesystem in the Native Backend.
2.  **Clean Separation:** Plugins use WIT (Standard). Internal App uses Protobuf.
3.  **Fast Rendering:** The Frontend Renderer draws nodes using Metadata (Protobuf), never calling into WASM plugins directly.

## Decision

### 1. Architecture: Shell vs. Plugin Manager

We will separate the application logic into distinct crates based on responsibility and compilation target.

#### The Plugin Manager (`backend/plugin-manager` crate)
*   **Target:** Native (x86/ARM). **Does NOT compile to WASM.**
*   **Dependencies:** `wasmtime`, `tokio`, `std::fs`, `cognexus-model`, `proto`.
*   **Responsibilities:**
    *   Scans filesystem for `.wasm` plugins.
    *   Loads plugins using `wasmtime`.
    *   Extracts WIT Metadata (Names, Ports, Visuals).
    *   Translates WIT Metadata -> Protobuf Messages (for Frontend).
    *   Populates the **Registry** (Node Definitions), *not* the User Graph.

#### The WASM Frontend
*   **Target:** WASM (`wasm32-unknown-unknown`).
*   **Dependencies:** `wgpu`, `prost` (Protobuf), `cognexus-model`.
*   **Responsibilities:**
    *   Receives `NodeDefinition` Protobufs from Backend.
    *   Populates "Add Node" UI.
    *   Renders nodes using visual data from Protobuf.
    *   **Crucial:** It never touches the plugin files directly.

#### The App State (Owned by Shell)
*   **Location:** `apps/desktop` (Tauri) or `apps/server` (Axum).
*   **Content:**
    *   **Master Graph:** Instance of `cognexus_model::Graph`.
    *   **Registry:** Instance of `plugin_manager::Registry`.

### 2. Data Flow Strategy

We bridge the gap between Plugin Authors (WIT) and Internal App (Protobuf) in the Backend.

**Step 1: Plugin Author (WIT)**
Plugin authors define nodes using the standard WIT interface.
```wit
record node-info {
    name: string,
    inputs: list<port>,
    visual: visual-style, // Added in future update
}
```

**Step 2: Backend Translation (Native)**
The `PluginLoader` reads the WIT data and maps it to internal Protobufs.
```rust
// In backend/plugin-manager
let wit_info = plugin.list_nodes()?;
let proto_def = NodeDefinition {
    name: wit_info.name,
    shape: convert_shape(wit_info.visual.shape),
    // ...
};
```

**Step 3: Frontend Consumption (WASM)**
The Frontend receives the Protobuf and updates the UI/Renderer.
```rust
// In frontend
let def = decode_protobuf(bytes);
renderer.register_node_type(def);
```

### 3. Execution (Out of Scope for this ADR)
Execution will happen entirely in the **Native Backend**. The Backend holds the live `wasmtime::Instance` and calls `execute()`. This path does not involve the Frontend or Protobuf, ensuring high performance.

## Technical Strategy

1.  **Crate Structure:**
    *   `apps/desktop`: The Shell. Imports `backend/plugin-manager` and `cognexus-model`.
    *   `backend/plugin-manager`: The Native Logic. Owns `wasmtime`.
    *   `cognexus-model`: The Shared Data Types (Graph, Node, Edge).
    *   `proto`: The Data Exchange Types (NodeDefinition, VisualData).

2.  **Dependency Management:**
    *   `backend/plugin-manager` is **Native Only**.
    *   `cognexus-model` and `cognexus-renderer` must remain pure to support usage in both environments.

## Consequences

### Positive

1.  **Solves Compilation Issues:** Isolates `wasmtime` to the Native Backend, preventing WASM compilation errors in the Frontend.
2.  **Naming Clarity:** `plugin-manager` clearly indicates its scope (Plugins), distinct from `model` (Data) or `renderer` (Visuals).
3.  **High Performance:** Renderer uses pure Data (Protobuf). Executor uses pure Code (Wasmtime). No crossing boundaries unnecessarily.
4.  **Standard API:** Plugin authors see a clean WIT interface, unaware of the internal Protobuf plumbing.

### Negative

1.  **Duplication of Schema:** We must maintain mapping logic between WIT Structs and Protobuf Messages.
2.  **State Sync:** Requires robust IPC syncing to ensure Frontend UI matches Backend State (solved by Protobuf + Events).

## Implementation Plan

1.  **Create Crate:** Initialize `backend/plugin-manager` (Native Crate).
2.  **Refactor:** Move logic from `apps/desktop/plugin_manager` to this new crate.
3.  **Define Protobufs:** Update `proto` crate to include `NodeDefinition` and `VisualData`.
4.  **Implement Translator:** Write the logic to convert WIT structs -> Protobufs.
5.  **Update Tauri:** Desktop app initializes `plugin-manager`, gets Protobufs, sends to Frontend.
6.  **Update Frontend:** Frontend decodes Protobufs and registers them with Renderer.

## References

- **WASM Component Model:** https://component-model.bytecodealliance.org/
- **Protobuf:** Used for efficient, type-safe IPC.
