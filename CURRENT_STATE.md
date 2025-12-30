# Current State - Cognexus Development

**Last Updated:** December 30, 2024

---

## 🎉 What We've Accomplished

### ✅ Complete Node-Graph Data Model
**Backend Model (`backend/model/`):**
- Graph, Node, Port, Edge data structures with full encapsulation
- Builder patterns for all entities (with optional IDs for deserialization)
- `DataType` trait split into `DataTypeInfo` (metadata) and `DataType` (serialization) with associated error types
- `DataTypeRegistry` for UUID→type lookup without error type conflicts ✅ **NEW**
- `NodeDefinition` trait split into `NodeDefinitionInfo` (metadata) and `NodeDefinition` (execution)
- `NodeDefinitionRegistry` for UUID→definition lookup without error type conflicts
- Graph operations: `add_node()` and `add_edge()` with full validation via registries ✅ **UPDATED**
- Port system: definitions create Port instances with unique UUIDs per definition ✅ **REFACTORED**
- Support for both dynamic creation and deserialization via optional IDs

### ✅ Plugin SDK (Traits)
**For plugin developers:**
- `DataTypeInfo` + `DataType` traits - define custom data types with serialization ✅ **UPDATED**
- `NodeDefinitionInfo` + `NodeDefinition` traits - define custom node types with execution
- Associated error types - each plugin defines its own errors for proper error hygiene
- Version tracking with `semver` - compatibility checking built into traits
- WASM-compatible interfaces - all trait methods work across WASM boundary
- Registry-safe trait splitting prevents object-safety issues with associated types

### ✅ First-Party Implementations
**Types (`backend/types/`):**
- `SignalType` - flow control signal with no data payload
- `TypeError` - type-specific error handling with location tracking
- Compiled to WASM ✅ (`cognexus_types.wasm`)
- Configured as both library and WASM module (`crate-type = ["lib", "cdylib"]`)

**Nodes (`backend/nodes/`):**
- `StartNode` - workflow initiator (0 inputs, 1 Signal output)
- `EndNode` - workflow terminator (1 Signal input, 0 outputs)
- `NodeError` - node-specific error handling with location tracking
- Compiled to WASM ✅ (`cognexus_nodes.wasm`)
- Configured as WASM module (`crate-type = ["cdylib"]`)

### ✅ WASM Build System
- Types and nodes successfully compile to `.wasm` files
- UUID configured with `js` feature for WASM/JavaScript compatibility
- Renderer compiles to WASM for Blazor integration
- All build steps documented in comprehensive README.md
- wasm-bindgen integration for JavaScript interop

### ✅ Architecture Decisions Locked In
- **WASM-first:** All nodes (first-party and plugins) compile to WASM for true dogfooding
- **Associated error types:** Each crate defines its own errors for proper encapsulation
- **UUID-based references:** Nodes, ports, edges identified by UUIDs for flexibility
- **Validation at boundaries:** Graph validates node existence when adding edges
- **Consistent builders:** All entities use builder pattern with optional IDs
- **Trait splitting:** Registry-safe traits (`NodeDefinitionInfo`) separate from execution traits

### ✅ Documentation
- Comprehensive README.md with build instructions
- AGENTS.md with architectural decisions and development philosophy
- This CURRENT_STATE.md tracking progress

---

## 🚧 What's Left To Do

### Critical Path (Must Have):

#### 1. **WASM Runtime & Loader** ⭐ **NEXT BIG TASK** ⭐
**Status:** Not started (0%)  
**Blocking:** Everything else

**A. CLI Tool for WASM Interrogation** (`cognexus inspect`)
- [ ] Create `cognexus-cli` crate with inspect command
- [ ] Load WASM module using wasmtime/wasmer
- [ ] Call registration function to discover types/nodes
- [ ] Extract metadata by calling trait methods:
  - `type_id()`, `name()`, `description()` for data types
  - `definition_id()`, `name()`, `input_port_specs()`, `output_port_specs()` for nodes
- [ ] Output discovered metadata (JSON or human-readable)
- [ ] Optionally generate signed metadata cache files
  - Include WASM hash for tamper detection
  - Binary format (not text-based for security)

**Why needed first:** Plugin authors use this to discover first-party type/node UUIDs. Third-party plugins can also be interrogated the same way. The WASM itself is the single source of truth.

**Example usage:**
```bash
# Discover first-party types
$ cognexus inspect cognexus_types.wasm
Found data types:
  - Signal (989bcbb2-b1a1-4f3f-be15-22ada278aedc)
    Description: A flow control signal with no data payload

# Discover nodes and their ports
$ cognexus inspect cognexus_nodes.wasm
Found node definitions:
  - Start (40ebe0be-d2db-4eed-80f3-91267352ee42)
    Output ports: signal → Signal type
```

**B. Desktop App WASM Loader** (`apps/desktop/cognexus/`)
- [ ] Add `wasmtime` or `wasmer` dependency to Cargo.toml
- [ ] Implement lazy-loading discovery system
  - Scan metadata cache files at startup (cheap)
  - Map type/node UUIDs → WASM file paths
  - Only load WASM when type/node is actually requested
- [ ] Create module loader
  - Load WASM bytes from files
  - Verify WASM hash matches metadata (security)
  - Instantiate WASM modules with appropriate imports
  - Handle module initialization
- [ ] Build registration system
  - Each WASM exports `register_plugin()` function
  - Function instantiates types/nodes and registers them
  - Registries populated on-demand when UUIDs requested
- [ ] Handle serialization across WASM boundary
  - Convert Rust types to bytes
  - Pass bytes to WASM for execution
  - Receive bytes from WASM
  - Convert bytes back to Rust types
- [ ] Separate first-party from third-party plugins
  - `builtin/` directory for first-party WASM (trusted, bundled with app)
  - `plugins/` directory for third-party WASM (sandboxed, user-installed)

**Why critical:** Without this, the WASM modules we built (`cognexus_types.wasm`, `cognexus_nodes.wasm`) are unused. This is THE piece that makes the plugin system real and validates our dogfooding approach.

**Design decisions made:**
- **Lazy loading:** Modules loaded on-demand, not at startup (performance)
- **WASM is source of truth:** No hand-written manifests to prevent tampering
- **Metadata caching:** Signed binary cache for fast discovery without loading WASM
- **CLI-based discovery:** Plugin authors use `cognexus inspect` to find UUIDs
- **Registration function pattern:** Each WASM exports `register_plugin()` that registers all its types/nodes
- **Trait methods as interface:** UUIDs extracted by calling `type_id()` and other trait methods

#### 2. **DataTypeRegistry** ✅ **COMPLETE**
**Status:** Complete (100%)  

- [x] Create registry parallel to `NodeDefinitionRegistry`
- [x] Store `DataTypeInfo` trait objects by UUID
- [x] Implement registration with duplicate detection
- [x] Implement lookup by UUID with error handling
- [x] Export from model crate
- [ ] Add version validation (similar to nodes) - deferred

**Why needed:** Nodes reference data types by UUID. We need a way to look them up at runtime for validation and execution.

#### 3. **Port Validation in Graph** ✅ **COMPLETE**
**Status:** Complete (100%)  
**Depends on:** ~~DataTypeRegistry~~ (done), WASM runtime

- [x] When adding edges, validate ports exist on nodes
  - Query node definition from registry
  - Check port specs for matching port IDs
- [x] `Graph::add_node()` validates definition exists in NodeDefinitionRegistry
- [x] `Graph::add_edge()` validates ports exist on node definitions
- [x] Add comprehensive error messages
- [ ] Validate port data types are compatible - deferred
  - Check source port type matches target port type
  - Consider type coercion rules (future)

**Current state:** Full validation implemented. Both methods now require registry references.

#### 4. **Graph Query Methods**
**Status:** Basic getters only (25%)

- [ ] `find_node_by_id(&self, id: Uuid) -> Option<&Node>`
- [ ] `find_node_by_name(&self, name: &str) -> Option<&Node>`
- [ ] `find_edge_by_id(&self, id: Uuid) -> Option<&Edge>`
- [ ] `get_node_ports(&self, node_id: Uuid, registry: &NodeDefinitionRegistry) -> Result<(Vec<Port>, Vec<Port>), ModelError>`
- [ ] `validate_edge(&self, edge_id: Uuid) -> Result<(), ModelError>`
- [ ] `find_edges_connected_to_node(&self, node_id: Uuid) -> Vec<&Edge>`
- [ ] `find_edges_connected_to_port(&self, node_id: Uuid, port_id: Uuid) -> Vec<&Edge>`

**Why needed:** Users need to query and inspect the graph structure.

---

### Important (Should Have):

#### 5. **Serialization/Deserialization**
**Status:** Builders support it, but no format implementation (10%)

- [ ] Add `serde::Serialize` + `serde::Deserialize` to Graph/Node/Edge/Port
- [ ] Implement graph save to JSON format
- [ ] Implement graph save to YAML format (more human-readable)
- [ ] Implement graph load from file with UUID preservation
- [ ] Handle versioning in saved files
- [ ] Migration system for old save formats
- [ ] Validation on load (check all referenced definitions exist)

**Why needed:** Users need to save and load their workflows.

#### 6. **More Data Types**
**Status:** Only Signal implemented (10%)

**Primitive types:**
- [ ] `U32Type` - unsigned 32-bit integer
- [ ] `I32Type` - signed 32-bit integer  
- [ ] `F64Type` - 64-bit floating point
- [ ] `StringType` - UTF-8 string
- [ ] `BoolType` - boolean true/false

**Complex types:**
- [ ] `ArrayType<T>` - homogeneous array
- [ ] `ObjectType` - key-value map
- [ ] `NullType` - represents absence of value

**Why needed:** Workflows need to pass actual data, not just signals.

#### 7. **More Node Types**
**Status:** Only Start/End implemented (10%)

**Math nodes:**
- [ ] `AddNode` - add two numbers
- [ ] `SubtractNode` - subtract two numbers
- [ ] `MultiplyNode` - multiply two numbers
- [ ] `DivideNode` - divide two numbers
- [ ] `ModuloNode` - remainder after division

**Logic nodes:**
- [ ] `IfNode` - conditional branching
- [ ] `SwitchNode` - multi-way branching
- [ ] `CompareNode` - equality/inequality checks
- [ ] `AndNode`, `OrNode`, `NotNode` - boolean logic

**Data nodes:**
- [ ] `ConstantNode` - output a constant value
- [ ] `VariableNode` - read/write a variable
- [ ] `TransformNode` - map data transformation

**Why needed:** Build actual useful workflows.

#### 8. **Execution Engine**
**Status:** Not started (0%)  
**Depends on:** WASM runtime, more types, more nodes

- [ ] Topological sort for node evaluation order
  - Handle cycles (error or special handling?)
  - Handle disconnected subgraphs
- [ ] Value storage during execution
  - Port values
  - Variable state
- [ ] Node execution orchestration
  - Call node `execute()` in correct order
  - Pass input values
  - Store output values
- [ ] Error handling during execution
  - Node execution errors
  - Type errors
  - Propagation of errors through graph
- [ ] Async execution support
  - Nodes that take time (network, file I/O)
  - Parallel execution where possible
- [ ] Execution context
  - Global variables
  - Environment data
  - Debugging/tracing hooks

**Why needed:** Make workflows actually run and produce results.

---

### Nice To Have (Later):

#### 9. **Advanced Features**
- [ ] Undo/redo system for graph editing
- [ ] Graph validation utilities
  - Detect cycles
  - Find disconnected nodes
  - Validate all port connections
- [ ] Subgraphs/grouping for organization
- [ ] Dynamic port creation (variable number of inputs/outputs)
- [ ] Hot reload for plugin WASM modules (dev experience)
- [ ] Graph diffing (show changes between versions)
- [ ] Graph templates/snippets

#### 10. **Developer Experience**
- [ ] Plugin template/scaffolding CLI tool
  - `cognexus new plugin MyPlugin`
  - Generates boilerplate for node/type crates
- [ ] Testing utilities for plugin developers
  - Mock graph execution
  - Test harness for nodes
- [ ] Documentation generator from trait implementations
- [ ] Example plugins with best practices
  - HTTP request node
  - File I/O nodes
  - JSON parsing nodes

#### 11. **UI Integration**
**Current state:** Blazor UI exists but doesn't use graph model yet

- [ ] Blazor components for node graph editor
  - Node rendering on canvas
  - Edge rendering with curves
  - Pan and zoom (camera controls exist)
- [ ] Node palette with drag-and-drop
- [ ] Property inspectors for selected nodes
- [ ] Wire the Graph model to UI via Tauri commands
- [ ] Real-time graph updates during execution
- [ ] Debugging visualization (current executing node, values)

---

## 📊 Progress Estimate

### Foundation: 70% Complete ✅
- ✅ Data model: 100%
- ✅ Plugin SDK: 100%
- ⚠️ First-party types/nodes: 30% (Signal, Start, End only)
- ✅ Build system: 100%
- ✅ Documentation: 80%

### Runtime: 20% Complete ⚠️
- ❌ WASM loader: 0% ← **BLOCKING EVERYTHING**
- ✅ Registries: 100% (Both registries complete, ready for WASM loader)
- ❌ Execution engine: 0%
- ❌ Serialization: 10% (structures support it, no format implementation)

### Overall Progress: ~40% Complete

**Next milestone:** WASM runtime functional (would bring overall to ~55%)

---

## 🎯 Recommended Next Steps (Priority Order)

### Phase 1: Make Plugins Real (Weeks 1-2)
1. **Build WASM runtime & loader** ⭐ Most critical ⭐ **READY TO START**
   - Research wasmtime vs wasmer
   - Implement basic module loading
   - Test with first-party nodes/types
   - Populate registries from WASM modules
   - This unblocks everything else
   - **Registries are ready and waiting for the loader**

2. ~~**Create DataTypeRegistry**~~ ✅ **COMPLETE**
   - Mirror NodeDefinitionRegistry pattern
   - Wire into WASM loader

3. ~~**Complete port validation**~~ ✅ **COMPLETE**
   - Use registries to validate edges fully

### Phase 2: Expand Capabilities (Weeks 3-4)
4. **Add more data types**
   - U32, String, Bool at minimum
   - Enables real data flow

5. **Add more node types**
   - Math operations (Add, Multiply, etc.)
   - Basic logic (If, Compare)
   - Enables useful workflows

6. **Implement graph serialization**
   - JSON format
   - Save/load workflows

### Phase 3: Make It Work (Weeks 5-6)
7. **Build execution engine**
   - Topological sort
   - Value propagation
   - Actually run workflows

8. **Wire UI to graph model**
   - Display nodes/edges on canvas
   - Create/delete nodes
   - Connect edges

### Phase 4: Polish (Week 7+)
9. **Developer experience**
   - Plugin templates
   - Documentation
   - Examples

10. **Advanced features**
    - Undo/redo
    - Validation
    - Hot reload

---

## 🏗️ Architecture Status

### ✅ Validated Decisions
- WASM-first for plugins: **Proven** (builds successfully)
- Associated error types: **Working well** (clean separation)
- Builder patterns: **Consistent** (all entities use them)
- UUID-based references: **Flexible** (no lifetime issues)
- Trait splitting (Info/Definition): **Solved registry problem** (no error type conflicts)
- Port UUIDs from definitions: **Correct design** (node instance ID + port definition ID = unique connection)
- Registry dependency injection: **Clean separation** (Graph doesn't own registries, they're passed in)

### ⚠️ Pending Validation
- WASM runtime choice (wasmtime vs wasmer): **Research needed**
- Module discovery strategy: **Design needed**
- Execution model (sync vs async): **Depends on use cases**
- Type coercion rules: **Deferred**

### 🔄 Known Technical Debt
None yet - we've been building things properly from the start.

---

## 🐛 Known Issues

1. **Types/Nodes WASM not loaded:** Built but unused (by design, waiting for runtime)
2. ~~**Port validation incomplete:**~~ ✅ Fixed - full validation implemented
3. ~~**No type registry:**~~ ✅ Fixed - DataTypeRegistry complete
4. **Limited type library:** Only Signal type implemented
5. **Limited node library:** Only Start/End nodes implemented
6. **Registries not instantiated:** No code creates/populates registries yet (waiting for WASM loader)

---

## 📝 Notes for Future Sessions

### When Implementing WASM Runtime:
- Consider security implications (sandboxing)
- Think about error recovery (bad plugin shouldn't crash app)
- Plan for versioning (old plugins with new runtime)
- Consider performance (caching, lazy loading)

### When Building Execution Engine:
- Start simple (synchronous, single-threaded)
- Add complexity only when needed
- Think about debugging/tracing from the start
- Consider pause/resume for long-running workflows

### When Expanding Type System:
- Keep serialization format stable
- Version the type system itself
- Consider type compatibility rules (can Int32 → Float64?)
- Think about nullable vs non-nullable

---

## 🤝 Contributing

This is a learning project. The human developer (Tony) implements features with AI guidance. AI provides architecture advice, code review, and teaches professional patterns.

See [AGENTS.md](./AGENTS.md) for development philosophy and guidelines.

---

**Ready to continue?** Start with building the WASM runtime - it's the critical path item that unblocks everything else.
