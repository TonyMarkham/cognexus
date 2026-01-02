# Next Session: Registry Implementation & Desktop Integration

## Quick Context

**What We Completed (Session 1 - Jan 1, 2026):**
- ✅ Created production-grade `backend/plugin-manager` crate
- ✅ Migrated and refactored plugin discovery code (scanner, loader, state, error handling)
- ✅ Defined Protobuf schema for plugin metadata (`proto/registry.proto`)
- ✅ Implemented WIT → Protobuf translation layer

**Current State:**
- Plugin manager can discover `.wasm` plugins from filesystem
- Can extract metadata (nodes, types, ports) from plugins
- Can translate WIT structures to Protobuf messages
- **BUT:** Discoveries are printed to logs and then discarded (no storage)

---

## Your Mission: Session 2

Implement the **Registry** and wire it into the **Desktop App**.

### Step 6: Implement Registry

**Goal:** Create a data structure to store discovered plugin metadata in memory.

**Create:** `backend/plugin-manager/src/registry.rs`

**Requirements:**
1. Store discovered nodes and types (keyed by ID)
2. Thread-safe for Tauri state (use `Arc<RwLock<HashMap<...>>>`)
3. Methods:
   - `new()` - Create empty registry
   - `register_node(NodeDefinition)` - Add discovered node
   - `register_type(TypeDefinition)` - Add discovered type
   - `get_node(&str) -> Option<NodeDefinition>` - Query by ID
   - `get_type(&str) -> Option<TypeDefinition>` - Query by ID
   - `list_nodes() -> Vec<NodeDefinition>` - Get all nodes
   - `list_types() -> Vec<TypeDefinition>` - Get all types

**Don't forget:**
- Production-grade error handling (follow existing patterns)
- Use protobuf types: `proto::{NodeDefinition, TypeDefinition}`
- Export from `lib.rs`

---

### Step 7: Update Desktop App

**Goal:** Replace the old plugin_manager module with the new crate.

**Tasks:**
1. Add dependency to `apps/desktop/cognexus/Cargo.toml`:
   ```toml
   cognexus-plugin-manager = { workspace = true }
   ```
   
2. Add to workspace `Cargo.toml`:
   ```toml
   cognexus-plugin-manager = { path = "backend/plugin-manager" }
   ```

3. In `apps/desktop/cognexus/src/main.rs`:
   - Remove `mod plugin_manager;`
   - Import: `use cognexus_plugin_manager::{PluginManager, Registry};`
   - Initialize registry: `let registry = Registry::new();`
   - After `discover_plugins()`, populate registry with discovered data
   - Store registry in Tauri state

4. Delete old code: `apps/desktop/cognexus/src/plugin_manager/`

5. Test: `cargo tauri dev` should discover plugins and populate registry

---

### Step 8: Add Tauri Commands

**Goal:** Expose registry data to frontend via Tauri IPC.

**In `apps/desktop/cognexus/src/main.rs`, add:**

```rust
#[tauri::command]
fn list_available_nodes(
    registry: tauri::State<Registry>
) -> Vec<proto::NodeDefinition> {
    registry.list_nodes()
}

#[tauri::command]
fn list_available_types(
    registry: tauri::State<Registry>
) -> Vec<proto::TypeDefinition> {
    registry.list_types()
}

#[tauri::command]
fn get_node_definition(
    id: String,
    registry: tauri::State<Registry>
) -> Option<proto::NodeDefinition> {
    registry.get_node(&id)
}
```

Register commands in `.invoke_handler()`.

**Test:** Use Tauri devtools to call these commands and verify responses.

---

## Important Reminders

1. **Read files before making claims** - Don't assume, verify
2. **Production-grade only** - Proper error handling, no shortcuts
3. **Constants over magic strings** - Define once, use everywhere
4. **Small chunks** - Propose code in bite-sized pieces for review
5. **Fresh eyes reviews** - Read the actual files, not your cache

---

## Key Files to Reference

- `backend/plugin-manager/src/lib.rs` - Public API, constants
- `backend/plugin-manager/src/error.rs` - Error handling patterns
- `backend/plugin-manager/src/translator.rs` - WIT → Proto conversion
- `proto/registry.proto` - Message definitions
- `SESSION_PLAN.md` - Full roadmap

---

## Success Criteria for Session 2

- [ ] Registry stores discovered plugins in memory
- [ ] Desktop app uses new `cognexus-plugin-manager` crate
- [ ] Old plugin_manager code deleted
- [ ] Tauri commands expose registry data
- [ ] Can query nodes/types from frontend (verify in devtools)

---

**Start with:** "Let's implement the Registry (Step 6). I'll guide you through creating `backend/plugin-manager/src/registry.rs` with production-grade patterns."
