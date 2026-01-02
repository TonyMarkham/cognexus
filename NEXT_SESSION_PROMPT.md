# Next Session: Frontend Integration & Visual Node Editor

## Quick Context

**What We Completed (Session 2 - Jan 1, 2026):**
- ✅ Implemented production-grade Registry (thread-safe, proper error handling)
- ✅ Added production-grade logging system (dual output, colors, timestamps)
- ✅ Integrated plugin-manager crate into desktop app
- ✅ Created Tauri commands exposing registry data to frontend
- ✅ End-to-end tested: 2 nodes + 1 type discovered and queryable

**Current State:**
- Backend can discover plugins and store them in registry
- Frontend can query available nodes/types via Tauri commands
- **BUT:** No UI to browse or visualize the discovered plugins
- **AND:** No visual node graph editor yet

---

## Your Mission: Session 3

Build the **Frontend Integration** to display discovered plugins and begin the visual node editor.

### Option A: Plugin Browser UI (Recommended Starting Point)

**Goal:** Create a Blazor component that displays discovered nodes and types.

**Tasks:**
1. Create a new Blazor component: `frontend/cognexus/Components/PluginBrowser.razor`
2. Call Tauri commands from C# using JSInterop
3. Display nodes in a list/grid with:
   - Node name
   - Description
   - Version
   - Input/output ports
4. Display types in a separate section

**Technical Details:**
- Use `IJSRuntime` to call `window.__TAURI__.core.invoke()`
- Deserialize JSON responses to C# DTOs (mirror protobuf structure)
- Handle errors gracefully
- Consider using Blazor's state management

---

### Option B: Visual Node Editor Foundation

**Goal:** Start building the canvas-based node editor.

**Tasks:**
1. Create canvas area in Blazor
2. Integrate with WGPU renderer (already exists in `backend/renderer`)
3. Display nodes as visual boxes on canvas
4. Basic interaction: pan, zoom
5. Click to select nodes

**Technical Details:**
- Renderer already exists but needs integration
- May need to pass canvas element to Tauri/WGPU
- Consider using existing `Camera2D` from `backend/model`

---

### Option C: Execution Engine (Advanced)

**Goal:** Build the workflow execution system.

**Tasks:**
- Design execution model (topological sort, async/await, etc.)
- Implement node instance management
- Handle data flow between nodes
- Error handling during execution

**This is complex - recommend doing A or B first.**

---

## Recommendation

**Start with Option A (Plugin Browser UI)** because:
1. Quick win - you'll see your plugins in the UI immediately
2. Validates the entire backend → frontend data flow
3. Provides foundation for drag-and-drop into node editor later
4. Low complexity, high value

Then move to Option B (Visual Editor) in a follow-up session.

---

## Key Files to Reference

**Backend (Already Complete):**
- `backend/plugin-manager/src/registry.rs` - Registry implementation
- `apps/desktop/cognexus/src/main.rs` - Tauri commands

**Frontend (Need to Create/Modify):**
- `frontend/cognexus/Pages/Home.razor` - Main page (might add browser here)
- Create: `frontend/cognexus/Components/PluginBrowser.razor`
- Create: `frontend/cognexus/Models/` - C# DTOs for nodes/types

**Tauri Commands Available:**
```javascript
// JavaScript/JSInterop
await invoke('list_available_nodes')  // Returns: Array<NodeDefinition>
await invoke('list_available_types')  // Returns: Array<TypeDefinition>
await invoke('get_node_definition', { id: 'node-id' })  // Returns: NodeDefinition | null
```

---

## Success Criteria for Session 3

**If doing Option A (Plugin Browser):**
- [ ] Blazor component displays discovered nodes
- [ ] Blazor component displays discovered types
- [ ] Data flows from Rust → Tauri → Blazor successfully
- [ ] Error handling in place
- [ ] UI is readable and organized

**If doing Option B (Visual Editor):**
- [ ] Canvas renders in Blazor
- [ ] WGPU renderer integration working
- [ ] Can display at least one node visually
- [ ] Basic camera controls (pan/zoom)

---

## Important Reminders

1. **Production-grade patterns** - No shortcuts, proper error handling
2. **Small incremental steps** - Break work into reviewable chunks
3. **Test as you go** - Verify each piece works before moving on
4. **Ask questions** - If requirements are unclear, ask before coding

---

## What to Do First

1. **Read this entire file** to understand the options
2. **Decide which option** you want to tackle (A recommended)
3. **Ask any clarifying questions** about the approach
4. **Start with smallest possible increment** (e.g., just calling one command and logging result)

---

**Start with:** "I want to build [Option A/B/C]. Let's start by [specific first step]."
