# Cognexus: Step-By-Step Node Graph Data Model (No Rendering)

## Goal
Teach Tony how to incrementally add a **pure data model** for a node graph in `backend/model`, defining:

- `Graph` (nodes + edges)
- `Node` (definition/type + ports)
- `Port` (UUID identity, direction, name)
- `Edge` (UUID identity, connects ports by UUID)

**Explicitly out of scope (for now):**
- rendering abstractions (`Drawable`)
- hit testing / picking
- “semantic port keys”
- storage optimizations (hash maps, indices, etc.)

## Must Follow Repo Instructions
- Read and follow `AGENTS.md` (and any scoped `AGENTS.md` files in subdirectories).
- This is a learning project: prefer teaching and guidance over implementing large features unprompted.

## Plugin Architecture (Planned)
- Plugins are distributed as **WASM** artifacts (single `.wasm` per plugin, cross-platform).
- The runtime maintains registries/resolvers for:
  - `NodeDefinitionId (Uuid) -> node definition implementation`
  - `DataTypeId (Uuid) -> type metadata/behavior`
- The graph/document stores only UUID references (`definition_id`, `data_type_id`), not definitions.
- Plugin security/trust model is deferred; assume trusted plugins for now.

## Design Principles
- **UUIDs are identity** (stable for serialization + IPC + execution + rendering references).
- **Node definitions are identified by UUID** (`definition_id: Uuid`) to support plugins without name collisions.
- Names are **mutable UX labels**, never used for linkage.
- Ports are **owned by nodes**. The graph does **not** maintain a global port list.
- Edges reference ports via `(node_id, port_id)` UUIDs.
- `Graph` is the aggregate root: creation/mutation happens through `Graph`, not through `Node`/`Edge`/`Port`.

## Minimal Types (v1)

### Node Definition (Important)
- In v1, `Node` stores `definition_id: uuid::Uuid` (a node type identifier).
- Display names are separate, mutable UX fields; they are never used for identity.

### Node Definition / Types (Planned Later)
- Introduce a WASM plugin system (single `.wasm` artifact per plugin).
- Introduce a `NodeDefinition` trait and a runtime registry mapping `definition_id (Uuid) -> definition implementation`.
- Introduce a `TypeRegistry` mapping `data_type_id (Uuid) -> type metadata`.
- For homogeneous debugging, use standardized debug wrapper types (owned by the host crate) rather than trusting plugin formatting.

- `GraphId = uuid::Uuid` (workflow/document identity)
- `NodeId = uuid::Uuid`
- `PortId = uuid::Uuid`
- `EdgeId = uuid::Uuid`
- `NodeDefinitionId = uuid::Uuid`

- `Graph { id: GraphId, nodes: Vec<Node>, edges: Vec<Edge> }`
- `Node { id: NodeId, definition_id: NodeDefinitionId, name: String, ports: Vec<Port> }`
- `Port { id: PortId, direction: PortDirection, name: String }`
- `Edge { id: EdgeId, from: PortEndpoint, to: PortEndpoint }`
- `PortEndpoint { node_id: NodeId, port_id: PortId }`
- `enum PortDirection { Input, Output }`

## Encapsulation Rules (v1)
- `Node`/`Port`/`Edge` have private fields and public getters.
- No public setters.
- Construction is `pub(crate)` and only performed by `Graph` methods.

## Acceptance Criteria
- Adds the node graph data model under `backend/model/src/node_graph/`.
- Updates `backend/model/src/lib.rs` to export `pub mod node_graph;`.
- Adds `uuid` as a workspace dependency and uses it from `cognexus-model`.
- `cargo check -p cognexus-model` succeeds.
- Does not introduce rendering/hit-testing/execution behavior.

**No additional validation logic required in v1.** (Can add later.)

## File Layout to Add
Create a new module directory:

- `backend/model/src/node_graph/`
  - `mod.rs`
  - `graph.rs`
  - `node.rs`
  - `port.rs`
  - `edge.rs`

Update:
- `backend/model/src/lib.rs` to include `pub mod node_graph;`

## Workspace Dependency
Add UUID dependency (workspace-managed):

- Root `Cargo.toml` under `[workspace.dependencies]`:
  - `uuid = { version = "1.x", features = ["v4"] }`

- `backend/model/Cargo.toml` add:
  - `uuid = { workspace = true }`

## Teaching Style Requirement
Proceed **one step at a time**.

Each step should be:
- One concrete action (create one file, add one type, run one command).
- Wait for Tony to confirm or paste output before continuing.
- No walls of text.

## Step Sequence
1) Verify current `backend/model/src/` contents (list directory).
2) Add uuid dependency (workspace + model crate).
3) Create `node_graph/mod.rs` with module declarations.
4) Create `node_graph/port.rs` (UUID + `PortDirection` + `Port` + getters).
5) Create `node_graph/node.rs` (UUID + `definition_id` + `name` + ports + getters; constructors `pub(crate)`).
6) Create `node_graph/edge.rs` (UUID + endpoints + getters; constructors `pub(crate)`).
7) Create `node_graph/graph.rs` (UUID graph id; `Graph::new()`; `Graph` methods create nodes/ports/edges).
8) Add `pub mod node_graph;` to `backend/model/src/lib.rs`.
9) Tony runs: `cargo check -p cognexus-model` and shares output.
