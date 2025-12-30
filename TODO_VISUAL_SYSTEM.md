# TODO: Visual System Implementation

**Date:** 2024-12-29  
**Context:** See ADR-0004 for full architecture. This TODO focuses on MVP first, then full DSL.

---

## Phase 0: MVP - Prove the Pipeline (1-2 weeks)

**Goal:** Get StartNode and EndNode rendering with hardcoded visuals to validate the architecture.

### Step 1: Create visual-style crate (minimal)
- [ ] `mkdir backend/visual-style`
- [ ] Create `Cargo.toml` with basic dependencies
- [ ] Create `src/lib.rs` with stub types
- [ ] Add to workspace

### Step 2: Define minimal NodeVisualStyle trait
```rust
// Just enough to render StartNode/EndNode
pub trait NodeVisualStyle {
    fn color(&self) -> Color;
    fn shape_type(&self) -> ShapeType;
}

pub enum ShapeType {
    RoundedRectangle,
    Circle,
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
```

- [ ] Define trait in `visual-style/src/lib.rs`
- [ ] Export from crate

### Step 3: Implement visual styles for StartNode/EndNode
- [ ] Update `backend/nodes/src/start.rs` to implement `NodeVisualStyle`
  - Green rounded rectangle
- [ ] Update `backend/nodes/src/end.rs` to implement `NodeVisualStyle`
  - Red rounded rectangle
- [ ] Add `visual-style` dependency to nodes crate

### Step 4: Update renderer to use visual styles
- [ ] Extend `NodeInstance` struct with:
  - `shape_type: u32`
  - `corner_radius: f32`
- [ ] Update vertex buffer layout (add locations for new fields)
- [ ] Update WGSL shader to handle shape switching
- [ ] Add SDF function for rounded rectangle
- [ ] Wire node definitions → visual styles → renderer

### Step 5: Test rendering
- [ ] Create a test graph with Start + End nodes
- [ ] Render to screen
- [ ] Verify shapes, colors, transforms work
- [ ] Verify camera pan/zoom still works

### Step 6: Add edge rendering between nodes
- [ ] Calculate port positions (hardcoded: left/right edges)
- [ ] Render straight line between ports
- [ ] Verify connection appears correctly

**Deliverable:** Two colored rounded rectangles with a line between them. Proves the pipeline works.

---

## Phase 1: Full DSL System (2-3 weeks)

**Goal:** Implement the complete YAML-based shape system from ADR-0004.

### Step 1: Shape DSL Implementation
- [ ] Design final YAML schema (review ADR examples)
- [ ] Implement `ShapeDefinition` Rust types
- [ ] Implement `EdgeGeometry` enum (Line, Arc, CubicBezier)
- [ ] Add `serde` derives for YAML parsing
- [ ] Implement shape validation (manifold check)
- [ ] Write unit tests for shape parsing

### Step 2: Shape Library
- [ ] Create `shapes/` directory
- [ ] Implement 3 basic shapes:
  - [ ] `rectangle.yaml`
  - [ ] `rounded_rectangle.yaml`
  - [ ] `circle.yaml`
- [ ] Implement `ShapeLibrary::load_from_directory()`
- [ ] Test shape loading and validation

### Step 3: Node-Type DSL Implementation
- [ ] Design final YAML schema for node-types
- [ ] Implement `NodeVisualStyle` as parsed struct (not trait)
- [ ] Implement port placement calculation
- [ ] Implement edge reference resolution
- [ ] Add transform support (scale, skew)
- [ ] Write unit tests

### Step 4: Node-Type Definitions
- [ ] Create `node-types/` directory
- [ ] Create `start.yaml` (references rounded_rectangle)
- [ ] Create `end.yaml` (references rounded_rectangle)
- [ ] Implement `NodeVisualStyleRegistry::load_from_directory()`
- [ ] Test loading and port calculation

### Step 5: Port Position Calculation
- [ ] Implement `EdgeGeometry::evaluate_at(t)` for Line
- [ ] Implement `EdgeGeometry::evaluate_at(t)` for Arc
- [ ] Implement `EdgeGeometry::evaluate_at(t)` for CubicBezier
- [ ] Implement automatic distribution (even spacing)
- [ ] Unit tests for geometric calculations

### Step 6: Renderer Integration
- [ ] Wire `ShapeLibrary` into renderer
- [ ] Wire `NodeVisualStyleRegistry` into renderer
- [ ] Update instance buffer with transform data
- [ ] Update shaders for transforms (scale, skew)
- [ ] Test with YAML-defined shapes

### Step 7: Complex Shapes
- [ ] Implement `hexagon.yaml`
- [ ] Implement `diamond.yaml`
- [ ] Implement `stored_data.yaml` (arc + chevron)
- [ ] Implement `document.yaml` (bezier curve)
- [ ] Test complex edge calculations

### Step 8: Complete Shape Library
- [ ] All shapes from flowchart reference image
- [ ] Validation and testing
- [ ] Documentation for each shape

---

## Phase 2: Plugin Support (1 week)

### Step 1: Plugin Shape Loading
- [ ] Scan plugin directories for `shapes/` folders
- [ ] Merge plugin shapes into main library
- [ ] Handle conflicts (duplicate IDs)

### Step 2: Plugin Node-Type Loading
- [ ] Scan plugin directories for `node-types/` folders
- [ ] Validate shape references
- [ ] Register visual styles

### Step 3: Hot Reload (Dev Experience)
- [ ] Watch YAML files for changes
- [ ] Reload shapes on change
- [ ] Reload node-types on change
- [ ] Update renderer without restart

---

## Phase 3: Advanced Features (Future)

### Text Rendering
- [ ] Implement text anchor positioning
- [ ] Render node labels below shapes
- [ ] Render status/error text
- [ ] Font loading and caching

### Icon Support
- [ ] Icon loading system (SVG or PNG)
- [ ] Icon anchor positioning
- [ ] Icon rendering inside shapes

### Animation
- [ ] Glow effects (selection, hover)
- [ ] Execution feedback (pulses, flows)
- [ ] State transitions

### Tooling
- [ ] YAML validator CLI
- [ ] Shape preview tool
- [ ] Visual shape editor (WYSIWYG)

---

## Current Status

**Completed:**
- ✅ ADR-0004 written (architecture documented)
- ✅ Design decisions finalized
- ✅ YAML schema examples created

**Next Action:**
Start Phase 0, Step 1: Create visual-style crate

**Blocking Items:**
None - ready to start

---

## Notes

- **MVP first:** Don't jump to full DSL until pipeline is proven
- **Incremental:** Each phase should produce working software
- **Test early:** Validate geometric calculations with unit tests
- **Document as you go:** Update this TODO with learnings
