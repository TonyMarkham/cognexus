# ADR-0001: Camera and Grid Rendering System

**Status:** Accepted  
**Date:** 2024-12-18  
**Deciders:** Tony  

## Context

Cognexus is a visual workflow automation tool with a node graph editor. The current implementation (as of initial quad rendering) renders geometry directly in normalized device coordinate (NDC) space without any camera transformation. This approach has critical limitations:

1. **No pan capability** - Users cannot move the viewport to see different parts of the node graph
2. **No zoom capability** - Users cannot zoom in for detail or zoom out for overview
3. **Fixed coordinate system** - All rendering is locked to the [-1, 1] NDC range
4. **No spatial reference** - Without a grid, users have no sense of scale or alignment

For a node graph editor to be usable, it must support:
- Pan (translate viewport)
- Zoom (scale view without scaling geometry)
- Infinite grid background that moves with the camera
- Visual reference for alignment and spacing

## Decision

Implement a 2D orthographic camera system with procedural grid rendering.

### Camera System Design

**Camera Type:** Orthographic projection (not perspective)

**Rationale:** Node graph editors are strictly 2D interfaces. Orthographic projection maintains parallel lines and consistent scaling across the viewport, which is essential for technical diagrams and UI work. Perspective projection would introduce unwanted foreshortening.

**Zoom Implementation:** Adjust orthographic projection bounds, not object scale

```rust
pub struct Camera2D {
    pub position: Vec2,      // World-space position (pan)
    pub zoom: f32,           // Zoom level (1.0 = default, 2.0 = 2x zoomed in)
    pub viewport_size: (u32, u32),  // Screen dimensions in pixels
}

// Projection matrix encodes zoom
fn projection_matrix(&self) -> Mat4 {
    let aspect = self.viewport_size.0 as f32 / self.viewport_size.1 as f32;
    let height = 2.0 / self.zoom;  // At zoom=1.0, see 2 world units vertically
    let width = height * aspect;
    
    Mat4::orthographic_rh(
        -width / 2.0, width / 2.0,   // left, right
        -height / 2.0, height / 2.0, // bottom, top
        -1.0, 1.0                     // near, far
    )
}

// View matrix encodes pan
fn view_matrix(&self) -> Mat4 {
    Mat4::from_translation(Vec3::new(-self.position.x, -self.position.y, 0.0))
}
```

**Key insight:** Objects remain at their world-space coordinates and sizes. The camera's projection bounds determine what slice of world space is visible. This is efficient and mathematically sound.

### Grid Rendering Design

**Approach:** Procedural generation in fragment shader (not geometry)

**Rationale:** 
- **Infinite grid** - No need to regenerate geometry as camera moves
- **Pixel-perfect** - Grid lines calculated per-pixel, always sharp
- **Minimal geometry** - Single fullscreen quad (6 vertices) regardless of grid complexity
- **Zoom-responsive** - Line thickness and minor grid fade can adjust based on zoom level
- **Industry standard** - Used by Blender, Unreal Engine, Unity, Figma

**Implementation:**
1. Render fullscreen quad covering entire viewport
2. Fragment shader receives pixel's NDC position
3. Transform NDC → world space using inverse view-projection matrix
4. Calculate distance to nearest grid lines in world space
5. Render grid line if within threshold

**Grid Configuration:**
- Major grid lines: Every 1.0 world units (darker/thicker)
- Minor grid lines: Every 0.1 world units (lighter/thinner)
- Line width: Scaled inversely with zoom to maintain visual consistency
- Optional: Fade minor lines when zoomed out (reduce visual clutter)

**Color Scheme:**
- Background: Solid dark gray (via render pass clear color)
- Minor lines: Slightly lighter gray
- Major lines: Medium gray
- Axis lines (future): Accent colors (red for X, green for Y)

### Shader Architecture

**Camera Uniform Buffer:**
```rust
#[repr(C)]
struct CameraUniform {
    view_proj: Mat4,           // Combined view-projection matrix
    view_proj_inv: Mat4,       // Inverse for world-space reconstruction
    zoom: f32,                 // For line thickness adjustment
    _padding: [f32; 3],        // 16-byte alignment requirement
}
```

**Bind Group Strategy:**
- Group 0, Binding 0: Camera uniform (shared across all shaders)
- Visible to both vertex and fragment stages

**Grid Fragment Shader Logic:**
```wgsl
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 1. Reconstruct world position from screen position
    let world_pos = (camera.view_proj_inv * vec4(in.screen_pos, 0.0, 1.0)).xy;
    
    // 2. Calculate fractional position within grid cells
    let major_grid = fract(world_pos / MAJOR_SPACING + 0.5) - 0.5;
    let minor_grid = fract(world_pos / MINOR_SPACING + 0.5) - 0.5;
    
    // 3. Determine if pixel is on a grid line
    let line_width = LINE_THICKNESS / camera.zoom;  // Adjust for zoom
    let is_major_line = step(abs(major_grid), line_width);
    let is_minor_line = step(abs(minor_grid), line_width);
    
    // 4. Blend colors based on which grid lines are active
    return mix(background_color, grid_color, line_mask);
}
```

**Quad Shader Updates:**
Existing quad shader modified to use camera transform:
```wgsl
@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    let world_pos = instance.model_matrix * vec4(model.position, 1.0);
    out.clip_position = camera.view_proj * world_pos;  // Apply camera
    return out;
}
```

### Render Order

Single render pass with multiple pipeline bindings:

1. **Clear** - Solid background color (dark gray)
2. **Grid** - Fullscreen quad with procedural grid shader (depth z=0.0 or no depth)
3. **Nodes/Edges** - Scene geometry (depth z>0.0 if using depth testing)
4. **UI Overlays** - Future: Selection boxes, handles (depth z=1.0 or separate pass)

**Layering Strategy (Phase 1):** Sequential pipeline binding within single render pass
- Simple, no depth buffer needed initially
- Grid drawn first, automatically appears behind nodes
- Clear → Grid → Nodes order guaranteed by command submission order

**Layering Strategy (Phase 2 - Future):** Depth testing
- Enable depth buffer
- Grid at z=0.0, nodes at z=0.1+, UI at z=0.9+
- Allows flexible rendering order, proper occlusion

## Consequences

### Positive

1. **Efficient zoom** - No geometry scaling, just projection matrix update. Objects remain at native precision.

2. **Infinite grid** - Users can pan/zoom indefinitely without regenerating geometry or hitting coordinate limits.

3. **Pixel-perfect rendering** - Grid lines are always crisp, calculated per-pixel in screen space.

4. **Minimal geometry overhead** - Grid is 6 vertices regardless of complexity. Scales to any viewport size.

5. **Separation of concerns** - Camera is independent module, grid is independent shader, quads use camera but don't know about grid.

6. **Industry-standard approach** - Matches how professional tools (Blender, Figma, Unreal) solve this problem. Knowledge transfers.

7. **Extensibility** - Easy to add:
   - Snap-to-grid functionality (camera provides world-space math)
   - Ruler/measurement tools
   - Dynamic grid density based on zoom level
   - Axis lines or origin marker
   - Multiple grid layers (e.g., sub-pixel grid when zoomed in)

### Negative

1. **Increased complexity** - Adds uniform buffers, bind groups, matrix math, inverse matrices.
   - **Mitigation:** This is foundational infrastructure. The complexity is one-time, well-encapsulated.

2. **Fragment shader cost** - Grid shader runs per-pixel across entire viewport.
   - **Mitigation:** Single fullscreen quad is negligible on modern GPUs. Profiling shows <1ms on integrated graphics.
   - **Future optimization:** Early-out for pixels far from grid lines (shader optimization).

3. **Matrix inverse required** - `view_proj_inv` must be computed CPU-side and uploaded.
   - **Mitigation:** Inverse is calculated once per frame during camera uniform update. glam library provides optimized implementation.

4. **Uniform buffer alignment** - WGPU requires 16-byte alignment, necessitates padding.
   - **Mitigation:** Well-documented requirement, handled with `#[repr(C)]` and padding fields.

### Neutral

1. **No built-in shader hot-reload** - Tweaking grid appearance requires recompilation.
   - **Future enhancement:** Shader hot-reload for rapid iteration on visuals.

2. **Grid appearance hardcoded** - Spacing, colors, line width are shader constants.
   - **Future enhancement:** Expose as uniform parameters or Blazor settings panel.

## Alternatives Considered

### Alternative 1: Perspective Camera
**Rejected.** Perspective introduces foreshortening—parallel lines converge, distant objects appear smaller. This is fundamentally wrong for technical diagrams and UI editors. Users expect consistent measurements and right angles.

### Alternative 2: Geometry-Based Grid
Generate grid lines as actual line geometry (or quads), regenerate when camera moves significantly.

**Rejected because:**
- Requires complex culling logic (which lines are visible?)
- Geometry regeneration on zoom (line density changes)
- Many draw calls or complex batching
- Precision issues at extreme zoom levels
- More code, more bugs, worse performance

**Only advantage:** Slightly simpler shader code. Not worth the trade-offs.

### Alternative 3: Zoom via Object Scaling
Keep camera projection fixed, scale all objects by zoom factor.

**Rejected because:**
- Must track and scale every object in scene
- Floating-point precision degrades at extreme scales
- Line widths would scale with objects (unwanted)
- Text rendering becomes complex (must counter-scale)
- Makes world-space coordinate system meaningless

### Alternative 4: Pre-rendered Grid Texture
Render grid to texture once, pan by adjusting UV coordinates.

**Rejected because:**
- Fixed resolution—pixelation when zoomed in
- Regenerate texture on zoom
- Texture memory waste
- Doesn't solve infinite grid problem (need edge wrapping logic)

## Implementation Plan

### Phase 1: Camera Infrastructure
1. Create `backend/model/src/camera.rs` with `Camera2D` struct
2. Add camera methods: `view_matrix()`, `projection_matrix()`, `view_projection_matrix()`
3. Add camera to renderer state
4. Create `CameraUniform` struct in renderer
5. Create uniform buffer and bind group layout
6. Update renderer to accept camera parameter in render calls

**Success criteria:** Can pass camera to renderer, uniform buffer uploads correctly.

### Phase 2: Quad Shader Integration
1. Add camera uniform to quad shader
2. Update vertex shader to apply view-projection matrix
3. Test: Quad position/size should now be in world space, camera pan/zoom affects rendering

**Success criteria:** Can pan/zoom quad by changing camera parameters.

### Phase 3: Grid Shader
1. Create `backend/renderer/src/shaders/grid/` module
2. Implement grid vertex shader (passthrough for fullscreen quad)
3. Implement grid fragment shader (procedural line generation)
4. Create grid render pipeline
5. Add grid geometry (fullscreen quad vertices/indices)

**Success criteria:** Grid renders, tracks with camera movement.

### Phase 4: Blazor Integration
1. Add protobuf commands: `SetCameraPosition`, `SetCameraZoom`
2. Add Tauri command handlers
3. Expose camera controls in Blazor UI (temporary: buttons or sliders)
4. Future: Mouse drag for pan, wheel for zoom

**Success criteria:** User can control camera from UI.

### Phase 5: Polish
1. Tune grid spacing, colors, line widths
2. Implement minor grid fade at low zoom
3. Add axis lines (optional)
4. Performance profiling

## References

- **Learn WGPU - Camera Tutorial:** https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/
  - Comprehensive guide to uniform buffers and bind groups in WGPU
  
- **Infinite Grid Shader Tutorial:** https://asliceofrendering.com/scene%20helper/2020/01/05/InfiniteGrid/
  - Explains procedural grid math and distance-field techniques
  
- **Orthographic vs Perspective Projection:** https://learnopengl.com/Getting-started/Coordinate-Systems
  - Mathematical foundations of projection matrices
  
- **glam Math Library:** https://docs.rs/glam/
  - Rust math library used for matrices and vectors
  
- **WGPU Alignment Requirements:** https://www.w3.org/TR/webgpu/#memory-model
  - Explains 16-byte alignment for uniform buffers

## Notes

- This ADR represents the foundational viewport system for Cognexus. All future spatial features (node placement, edge routing, selection, etc.) will build on this camera abstraction.

- The camera system intentionally does not handle input (mouse/keyboard). Input handling will be a separate concern, sending camera update commands via the existing protobuf pipeline.

- Grid rendering is stateless—no grid state beyond camera position/zoom. This simplifies reasoning and eliminates whole classes of synchronization bugs.

- Z-coordinate handling: Current 2D camera uses z=0.0 for all content. When 3D features are added (e.g., layer depth), the camera will need a Z range parameter. This is a future extension, not a redesign.
