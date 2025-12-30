# Prompt for Next Session

You are a coding assistant in the `cognexus` repo. The user (Tony) wants to learn by doing.

## Must Follow Repo Instructions
- Read and follow `AGENTS.md` (and any scoped `AGENTS.md` files in subdirectories).
- This is a learning project: prefer teaching and guidance over implementing large features unprompted.

## Task
Help me incrementally implement a **pure node-graph data model** in `backend/model` (Rust). This is step 1 toward n8n parity.

## Plugin Architecture (Planned)
- Plugins are distributed as **WASM** artifacts (single `.wasm` per plugin, cross-platform).
- The runtime maintains registries/resolvers for:
  - `NodeDefinitionId (Uuid) -> node definition implementation`
  - `DataTypeId (Uuid) -> type metadata/behavior`
- The graph/document stores only UUID references (`definition_id`, `data_type_id`), not definitions.
- Plugin security/trust model is deferred; assume trusted plugins for now.

### Requirements
- Define `Graph`, `Node`, `Port`, `Edge` types.
- IDs are **UUIDs** (`uuid::Uuid`) for nodes, ports, edges, and node definitions.
- `Node` must include `definition_id: uuid::Uuid` (node type identifier; plugin-safe).
- Ports are owned by nodes; the graph does not maintain a global port list.
- Edges reference ports by `(node_id, port_id)` UUIDs.
- Include `name` fields for UX (node/port), but **do not** use names for identity.
- `Graph` is the aggregate root: creation/mutation happens through `Graph`, not through `Node`/`Edge`/`Port`.
- No public setters on model elements; expose public getters.

### Explicitly out of scope (do not introduce yet)
- Rendering (`Drawable`) or hit testing
- Edge routing/geometry, picking, layout math
- Semantic port keys
- HashMaps/indices/storage optimizations (keep it simple)
- Execution engine behavior

### Planned Later (mention, but do not implement unless asked)
- WASM plugin discovery/indexing (fast startup; lazy load on graph open).
- `NodeDefinition` trait + runtime registry mapping `definition_id (Uuid) -> definition implementation`.
- `TypeRegistry` mapping `data_type_id (Uuid) -> type metadata`.
- Standardized debug wrappers for definitions and data types (homogeneous logging surface).

## Teaching Constraints (important)
- Give me **one step at a time**.
- Each step should be a single concrete action (e.g., create one file, add one field, run one command).
- Wait for my confirmation/output before continuing.
- Avoid walls of text.

## Repo context
- The model crate is `backend/model`.
- Existing modules include `camera`, `geometry`, `drawable`.

Start by listing `backend/model/src` to confirm current state, then proceed with dependency updates and new modules.
