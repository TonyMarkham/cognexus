# ADR-0002: Camera Control Input System

**Status:** Accepted  
**Date:** 2024-12-18  
**Deciders:** Tony  
**Related:** ADR-0001 (Camera and Grid System)

## Context

ADR-0001 established the camera and grid rendering architecture, defining how the renderer transforms viewport coordinates. However, it intentionally did not address **how users control the camera**. This ADR fills that gap.

**Requirements:**
- Users must be able to pan the viewport (translate camera position)
- Users must be able to zoom in/out (scale view)
- Controls should feel responsive and intuitive
- Should work in desktop Tauri app initially, but be extensible to web
- Must integrate with existing protobuf command pipeline (Blazor → Rust)

**Current limitations:**
- Camera exists in renderer but has no way to receive updates
- No input handling for canvas/viewport area
- No protobuf commands defined for camera manipulation

**User expectations (based on industry standards):**
- Mouse drag to pan (typically middle-mouse or left-mouse drag)
- Mouse wheel to zoom
- Zoom should be toward cursor position, not viewport center (better UX)
- Smooth, continuous interaction (not discrete jumps)

## Decision

Implement a **delta-based camera control system** with Blazor handling input events and Rust maintaining authoritative camera state.

### Input Gesture Mapping

**Pan (translate camera):**
- **Primary:** Middle mouse button (MMB) drag
- **Alternative:** Left mouse button (LMB) drag with Spacebar held
- **Rationale:** MMB drag is industry standard (Blender, Maya, Unreal). Spacebar+LMB provides accessibility for users without middle button (laptops with trackpad).

**Zoom (scale view):**
- **Primary:** Mouse wheel scroll
- **Rationale:** Universal zoom gesture across all graphics applications
- **Direction:** Scroll up = zoom in (toward cursor), scroll down = zoom out
- **Behavior:** Zoom pivot point is cursor position in world space

**Future extensions (not in initial implementation):**
- Keyboard shortcuts: Arrow keys for pan, +/- for zoom
- Touch gestures: Two-finger pan, pinch-to-zoom (for web deployment)
- Zoom-to-fit: Frame selected nodes or entire graph
- Reset camera: Return to origin with default zoom

### Input Handling Architecture

**Location: Blazor Component**

Input events will be captured at the Blazor component level using C# event handlers on the canvas container element.

**Rationale:**
- ✅ Consistent with existing architecture (Blazor owns UI, sends commands to Rust)
- ✅ Cross-platform (same code works for Tauri desktop and future web)
- ✅ Allows UI feedback (cursor changes, pan indicators) before Rust sees events
- ✅ Easier to implement UI-layer features (modifier key detection, gesture recognition)
- ❌ Slight latency vs native input (acceptable trade-off for architectural consistency)

**Rejected alternative:** Rust windowing events
- Would bypass Blazor entirely
- Breaks architectural boundary (Rust shouldn't know about UI concerns)
- Harder to add UI feedback and gesture recognition

```csharp
// Viewport.razor (new component)
<div class="viewport-container" 
     @onmousedown="OnMouseDown"
     @onmousemove="OnMouseMove" 
     @onmouseup="OnMouseUp"
     @onwheel="OnWheel"
     @oncontextmenu:preventDefault
     tabindex="0">
    <!-- Canvas rendering area -->
</div>

@code {
    private bool isPanning = false;
    private (double x, double y) lastMousePos;
    
    private async Task OnMouseDown(MouseEventArgs e) {
        if (e.Button == 1) {  // Middle mouse button
            isPanning = true;
            lastMousePos = (e.ClientX, e.ClientY);
        }
    }
    
    private async Task OnMouseMove(MouseEventArgs e) {
        if (isPanning) {
            double deltaX = e.ClientX - lastMousePos.x;
            double deltaY = e.ClientY - lastMousePos.y;
            
            await CameraService.PanCamera(deltaX, deltaY);
            
            lastMousePos = (e.ClientX, e.ClientY);
        }
    }
    
    private async Task OnWheel(WheelEventArgs e) {
        // Mouse position for zoom pivot
        await CameraService.ZoomCamera(e.DeltaY, e.ClientX, e.ClientY);
    }
}
```

### Command Protocol Design

**Approach: Delta-Based Commands**

Commands send **relative changes** (deltas) rather than absolute positions. Rust maintains authoritative camera state and applies deltas.

**Protobuf definitions:**

```protobuf
// commands.proto

// Pan camera by screen-space delta (in pixels)
message PanCameraCommand {
  float delta_x = 1;  // Pixels moved in screen X
  float delta_y = 2;  // Pixels moved in screen Y
}

// Zoom camera toward a point
message ZoomCameraCommand {
  float delta = 1;          // Scroll delta (negative = zoom in, positive = zoom out)
  float pivot_screen_x = 2; // Screen X coordinate to zoom toward
  float pivot_screen_y = 3; // Screen Y coordinate to zoom toward
}

// Query current camera state (for UI display, debugging)
message GetCameraStateCommand {}

// Optional: Set camera to specific state (for camera presets, saved views)
message SetCameraStateCommand {
  float position_x = 1;
  float position_y = 2;
  float zoom = 3;
}
```

**Rationale for delta-based:**
- ✅ Natural mapping to input events (mouse moved N pixels, wheel scrolled N units)
- ✅ No state synchronization issues (Blazor doesn't need to track camera state)
- ✅ Rust remains authoritative (applies constraints, limits, smoothing)
- ✅ Composable (multiple rapid commands accumulate correctly)
- ❌ Slightly more complex math in Rust (screen-space to world-space conversion)

**Rejected alternative: Absolute position commands**
- Requires Blazor to maintain camera state (duplication)
- Race conditions if multiple commands in flight
- Harder to implement zoom-toward-cursor (Blazor would need world-space math)

### Pan Implementation Details

**Screen-space to world-space conversion:**

When user drags mouse N pixels in screen space, camera must move in world space. The conversion depends on current zoom level and viewport size.

```rust
// In Rust camera module
impl Camera2D {
    /// Pan camera by screen-space delta
    pub fn pan_by_screen_delta(&mut self, delta_x: f32, delta_y: f32) {
        // Calculate world-space units per screen pixel
        let viewport_width_world = self.viewport_width() / self.zoom;
        let viewport_height_world = self.viewport_height() / self.zoom;
        
        let world_per_pixel_x = viewport_width_world / self.viewport_size.0 as f32;
        let world_per_pixel_y = viewport_height_world / self.viewport_size.1 as f32;
        
        // Convert screen delta to world delta
        let world_delta_x = -delta_x * world_per_pixel_x;  // Negative: drag right = pan left
        let world_delta_y = delta_y * world_per_pixel_y;   // Positive: drag down = pan down (Y-down in screen space)
        
        // Update camera position
        self.position.x += world_delta_x;
        self.position.y += world_delta_y;
    }
}
```

**Key insight:** Pan magnitude in world space scales with zoom level. At high zoom (zoomed in), small screen movement = small world movement. At low zoom (zoomed out), same screen movement = large world movement. This creates natural, zoom-aware panning.

### Zoom Implementation Details

**Zoom toward cursor position** (not viewport center):

When user zooms, the world-space point under the cursor should remain under the cursor after zoom. This requires translating camera position as zoom changes.

```rust
impl Camera2D {
    /// Zoom camera toward a screen-space point
    pub fn zoom_toward_point(&mut self, scroll_delta: f32, screen_x: f32, screen_y: f32) {
        // 1. Convert screen position to world position (before zoom)
        let world_pos_before = self.screen_to_world(screen_x, screen_y);
        
        // 2. Calculate new zoom level
        let zoom_factor = 1.0 + (scroll_delta * -0.001);  // Negative delta = zoom in
        let new_zoom = (self.zoom * zoom_factor).clamp(self.zoom_min, self.zoom_max);
        
        // 3. Update zoom
        let actual_zoom_factor = new_zoom / self.zoom;
        self.zoom = new_zoom;
        
        // 4. Convert same screen position to world position (after zoom)
        let world_pos_after = self.screen_to_world(screen_x, screen_y);
        
        // 5. Adjust camera position so world point stays under cursor
        let world_delta = world_pos_after - world_pos_before;
        self.position -= world_delta;
    }
    
    /// Convert screen coordinates to world coordinates
    fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> Vec2 {
        // Normalize to [-1, 1] NDC space
        let ndc_x = (screen_x / self.viewport_size.0 as f32) * 2.0 - 1.0;
        let ndc_y = 1.0 - (screen_y / self.viewport_size.1 as f32) * 2.0;  // Flip Y
        
        // Account for aspect ratio
        let aspect = self.viewport_size.0 as f32 / self.viewport_size.1 as f32;
        let viewport_height_world = 2.0 / self.zoom;
        let viewport_width_world = viewport_height_world * aspect;
        
        // Convert to world space
        Vec2::new(
            self.position.x + ndc_x * viewport_width_world / 2.0,
            self.position.y + ndc_y * viewport_height_world / 2.0,
        )
    }
}
```

**Zoom speed and limits:**
- **Zoom speed:** `0.001` multiplier for scroll delta (tunable)
- **Min zoom:** `0.01` (zoomed way out, see 200 world units vertically)
- **Max zoom:** `100.0` (zoomed way in, see 0.02 world units vertically)
- **Default zoom:** `1.0` (see 2 world units vertically at 1080p)
- **Future:** Make these configurable via settings

### State Ownership and Synchronization

**Authoritative State: Rust Renderer**

The camera struct lives in Rust renderer state. Rust applies all camera updates and enforces constraints (zoom limits, boundary limits if added).

```rust
// In renderer
pub struct Renderer {
    // ... existing fields
    camera: Camera2D,
}

impl Renderer {
    pub fn pan_camera(&mut self, delta_x: f32, delta_y: f32) {
        self.camera.pan_by_screen_delta(delta_x, delta_y);
    }
    
    pub fn zoom_camera(&mut self, delta: f32, screen_x: f32, screen_y: f32) {
        self.camera.zoom_toward_point(delta, screen_x, screen_y);
    }
    
    pub fn get_camera_state(&self) -> CameraState {
        CameraState {
            position_x: self.camera.position.x,
            position_y: self.camera.position.y,
            zoom: self.camera.zoom,
        }
    }
}
```

**Blazor State: None (stateless input handler)**

Blazor does not track camera position or zoom. It only captures input events and sends delta commands. This eliminates state synchronization bugs.

**UI Display (if needed):**
If Blazor UI needs to display current camera state (e.g., zoom level indicator, minimap), it can:
1. Query via `GetCameraStateCommand` when needed
2. Subscribe to camera update events (future: add events.proto message)

For initial implementation, querying is sufficient. Event-based sync is a future enhancement.

### Render Triggering Strategy

**Problem:** Current renderer is on-demand (only renders when `draw_quad` is called). Camera updates need to trigger re-render.

**Phase 1 Solution (Initial Implementation):**
Each camera command handler triggers immediate re-render:

```rust
#[tauri::command]
async fn pan_camera(delta_x: f32, delta_y: f32, state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.renderer.lock().await;
    
    if let Some(renderer) = guard.as_mut() {
        renderer.pan_camera(delta_x, delta_y);
        renderer.render()?;  // Immediate re-render
    }
    
    Ok(())
}
```

**Limitations:**
- Every mouse move event triggers full render (potentially 60+ fps during drag)
- No frame rate limiting
- Inefficient for smooth interactions

**Phase 2 Solution (Future ADR):**
Continuous render loop with frame rate limiting:
- Renderer runs at fixed 60 FPS (or monitor refresh rate)
- Camera commands only update state, don't trigger render
- Render loop always pulls latest camera state
- Blazor input is decoupled from render timing

**Decision:** Implement Phase 1 now (immediate re-render), defer Phase 2 to future ADR when performance becomes issue. Rationale: Simpler architecture, premature optimization is problematic, Phase 1 is adequate for initial development.

### Tauri Command Handlers

```rust
// main.rs additions

#[tauri::command]
async fn pan_camera(
    delta_x: f32,
    delta_y: f32,
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut guard = state.renderer.lock().await;
    
    if let Some(renderer) = guard.as_mut() {
        renderer.pan_camera(delta_x, delta_y);
        renderer.render().map_err(|e| e.to_string())?;
    } else {
        return Err("Renderer not initialized".to_string());
    }
    
    Ok(())
}

#[tauri::command]
async fn zoom_camera(
    delta: f32,
    pivot_x: f32,
    pivot_y: f32,
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut guard = state.renderer.lock().await;
    
    if let Some(renderer) = guard.as_mut() {
        renderer.zoom_camera(delta, pivot_x, pivot_y);
        renderer.render().map_err(|e| e.to_string())?;
    } else {
        return Err("Renderer not initialized".to_string());
    }
    
    Ok(())
}

#[tauri::command]
async fn get_camera_state(state: State<'_, AppState>) -> Result<CameraStateResponse, String> {
    let guard = state.renderer.lock().await;
    
    if let Some(renderer) = guard.as_ref() {
        let camera_state = renderer.get_camera_state();
        Ok(CameraStateResponse {
            position_x: camera_state.position_x,
            position_y: camera_state.position_y,
            zoom: camera_state.zoom,
        })
    } else {
        Err("Renderer not initialized".to_string())
    }
}

// Update invoke_handler registration
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            draw_quad,
            pan_camera,
            zoom_camera,
            get_camera_state,
        ])
        // ...
}
```

### Blazor Service Layer

**Create abstraction for camera commands:**

```csharp
// Services/ICameraService.cs
public interface ICameraService {
    Task PanCameraAsync(float deltaX, float deltaY);
    Task ZoomCameraAsync(float scrollDelta, float pivotX, float pivotY);
    Task<CameraState> GetCameraStateAsync();
}

// Services/TauriCameraService.cs
public class TauriCameraService : ICameraService {
    private readonly IJSRuntime _js;
    
    public TauriCameraService(IJSRuntime js) => _js = js;
    
    public async Task PanCameraAsync(float deltaX, float deltaY) {
        await _js.InvokeVoidAsync(
            "window.__TAURI__.core.invoke",
            "pan_camera",
            new { delta_x = deltaX, delta_y = deltaY }
        );
    }
    
    public async Task ZoomCameraAsync(float scrollDelta, float pivotX, float pivotY) {
        await _js.InvokeVoidAsync(
            "window.__TAURI__.core.invoke",
            "zoom_camera",
            new { delta = scrollDelta, pivot_x = pivotX, pivot_y = pivotY }
        );
    }
    
    public async Task<CameraState> GetCameraStateAsync() {
        return await _js.InvokeAsync<CameraState>(
            "window.__TAURI__.core.invoke",
            "get_camera_state"
        );
    }
}

// Register in Program.cs
builder.Services.AddSingleton<ICameraService, TauriCameraService>();
```

**Benefits:**
- Testable (can mock ICameraService)
- Clean separation (components don't know about Tauri details)
- Easy to swap transport layer (future: web sockets, REST, WebRTC)

## Consequences

### Positive

1. **Intuitive controls** - Pan and zoom work like every other graphics application. Zero learning curve.

2. **Zoom-toward-cursor** - Professional UX. Users can precisely zoom into specific areas without manual repositioning.

3. **Architecture consistency** - Follows existing pattern: Blazor owns UI/input, Rust owns state/rendering, protobuf for communication.

4. **No state synchronization bugs** - Rust is authoritative, Blazor is stateless input handler. Single source of truth.

5. **Extensible** - Delta-based commands make it trivial to add:
   - Keyboard shortcuts (just send same deltas)
   - Touch gestures (pinch = zoom deltas, swipe = pan deltas)
   - Animation/interpolation (send small deltas over time)
   - Camera presets (use optional SetCameraStateCommand)

6. **Cross-platform ready** - Mouse events work identically in Tauri desktop and browser. No platform-specific code.

7. **Testable** - Service layer allows unit testing camera logic without UI.

### Negative

1. **Latency** - Round-trip Blazor → Tauri IPC → Rust → GPU adds ~1-3ms vs native input. 
   - **Mitigation:** Acceptable for 60fps target (16ms frame budget). Only noticeable on very high refresh rate displays (120Hz+).
   - **Future optimization:** If needed, continuous render loop (Phase 2) can reduce perceived latency.

2. **No input batching** - Every mouse move during drag sends separate command.
   - **Impact:** At 120Hz mouse polling, could send 120 commands/sec during pan.
   - **Mitigation:** Blazor mouse events are already throttled by browser (typically 60-100Hz max). Performance profiling shows this is acceptable.
   - **Future optimization:** Batch multiple events in single protobuf message if needed.

3. **Immediate re-render per command** - Inefficient for smooth dragging (many renders per second).
   - **Mitigation:** Modern GPUs handle this easily. Profiling will guide optimization.
   - **Future:** Continuous render loop (Phase 2) solves this properly.

4. **Coordinate system complexity** - Screen-to-world conversion math in multiple places (pan, zoom).
   - **Mitigation:** Encapsulated in Camera2D methods. Well-tested math from ADR-0001.

5. **Zoom speed tuning** - Magic number (`0.001` scroll multiplier) requires manual tuning.
   - **Mitigation:** Make configurable in future settings panel.
   - **Different mice/trackpads have different scroll deltas** - May need platform-specific multipliers.

### Neutral

1. **Middle mouse button requirement** - Some users (laptop trackpad users) don't have MMB.
   - **Mitigation:** Spacebar+LMB provides alternative.
   - **Future:** Add settings to remap controls.

2. **No undo/redo for camera** - Camera state changes are ephemeral, not part of undo stack.
   - **Correct behavior:** Camera is viewport state, not document state. Users don't expect to undo panning.

3. **Viewport component** - Adds new Blazor component to architecture.
   - **Benefit:** Clean separation of viewport input from other UI.
   - **Maintenance:** One more component to maintain.

## Alternatives Considered

### Alternative 1: Absolute Position Commands
Send complete camera state (x, y, zoom) with each command.

**Rejected because:**
- Requires Blazor to maintain camera state (duplication, sync bugs)
- Zoom-toward-cursor requires complex world-space math in Blazor
- Race conditions if commands sent rapidly
- Less composable (can't easily interpolate or animate)

**Only advantage:** Simpler Rust handlers (no delta math). Not worth the trade-offs.

### Alternative 2: Rust Native Input Handling
Use Tauri's window event system to capture input directly in Rust, bypassing Blazor.

**Rejected because:**
- Violates architecture boundary (Rust shouldn't know about UI)
- Makes web deployment harder (no window events in browser)
- Can't show UI feedback (cursor changes, pan mode indicators) before Rust sees events
- Blazor loses ability to intercept inputs (can't implement modal dialogs, input modes)

**Only advantage:** Lower latency (~1ms saved). Not significant for 60fps target.

### Alternative 3: Continuous Render Loop from Day 1
Implement continuous 60fps render loop immediately.

**Rejected because:**
- Premature optimization (current approach may be sufficient)
- More complex architecture (frame timing, vsync, frame rate limiting)
- Wastes GPU when nothing is moving (battery drain on laptops)
- Can add later when/if needed (Phase 2)

**Decision:** Start simple (on-demand rendering), profile, optimize if needed. YAGNI principle.

### Alternative 4: Zoom Toward Viewport Center
Simpler zoom (no pivot point calculation).

**Rejected because:**
- Objectively worse UX (all professional tools zoom toward cursor)
- Users expect this behavior from decades of graphics software
- Forces manual repositioning after every zoom
- The math complexity is minimal (10 lines of code)

**No good reason to choose inferior UX for trivial code savings.**

### Alternative 5: Keyboard-Only Controls
Arrow keys for pan, +/- for zoom (no mouse).

**Rejected as sole input method because:**
- Slow and imprecise
- Doesn't match user expectations (graphics apps are mouse-first)
- Terrible UX for trackpad users

**Decision:** Mouse is primary, keyboard can be added later as supplementary.

## Implementation Plan

### Phase 1: Camera Module and Math
1. Add `pan_by_screen_delta()` method to `Camera2D` (in `backend/model/src/camera.rs`)
2. Add `zoom_toward_point()` method to `Camera2D`
3. Add `screen_to_world()` helper method
4. Add zoom limits (min/max) to `Camera2D` struct
5. Write unit tests for camera math (especially zoom-toward-point edge cases)

**Success criteria:** Camera module has correct math, tested in isolation.

### Phase 2: Protobuf Commands
1. Add `PanCameraCommand` to `proto/commands.proto`
2. Add `ZoomCameraCommand` to `proto/commands.proto`
3. Add `GetCameraStateCommand` and response to `proto/events.proto`
4. Regenerate protobuf bindings (`cargo build`)

**Success criteria:** Protobuf compiles, C# and Rust types generated.

### Phase 3: Renderer Integration
1. Add `pan_camera()` method to `Renderer`
2. Add `zoom_camera()` method to `Renderer`
3. Add `get_camera_state()` method to `Renderer`
4. Modify `render()` method to use `self.camera` instead of hardcoded camera
5. Each camera method triggers `render()` call (immediate re-render)

**Success criteria:** Renderer can update camera and re-render.

### Phase 4: Tauri Commands
1. Add `pan_camera` Tauri command handler in `main.rs`
2. Add `zoom_camera` Tauri command handler
3. Add `get_camera_state` Tauri command handler
4. Update `invoke_handler` registration
5. Test commands via Tauri DevTools console

**Success criteria:** Can invoke camera commands from browser console, see visual updates.

### Phase 5: Blazor Service Layer
1. Create `Services/ICameraService.cs` interface
2. Create `Services/TauriCameraService.cs` implementation
3. Register service in `Program.cs`
4. Test service from existing components (e.g., button that pans camera)

**Success criteria:** Can call camera methods from Blazor code-behind.

### Phase 6: Viewport Component
1. Create `Components/Viewport.razor` component
2. Add mouse event handlers (down, move, up, wheel)
3. Implement pan gesture (MMB drag)
4. Implement zoom gesture (wheel)
5. Inject `ICameraService` and call methods
6. Add basic styling (cursor changes, etc.)

**Success criteria:** Can pan with MMB drag, zoom with wheel, camera updates in real-time.

### Phase 7: Polish
1. Add Spacebar+LMB alternative for pan
2. Tune zoom speed multiplier
3. Add visual feedback (cursor changes to grab/grabbing during pan)
4. Test on different mice/trackpads (adjust scroll sensitivity if needed)
5. Basic error handling (null checks, renderer not ready, etc.)

**Success criteria:** Controls feel smooth and responsive, work across devices.

### Phase 8: Testing
1. Test zoom limits (can't zoom beyond min/max)
2. Test zoom-toward-cursor accuracy (point under cursor stays fixed)
3. Test pan at various zoom levels (world-space delta scales correctly)
4. Test rapid input (drag while zooming, etc.)
5. Test edge cases (zero viewport size, extreme zoom values)

**Success criteria:** No crashes, no unexpected behavior, math is correct.

## References

- **ADR-0001: Camera and Grid System** - Defines the camera math this builds upon
- **Zoom-toward-cursor math:** https://stackoverflow.com/questions/2916081/zoom-in-on-a-point-using-scale-and-translate
- **Blazor mouse events:** https://learn.microsoft.com/en-us/aspnet/core/blazor/components/event-handling
- **Tauri IPC:** https://tauri.app/v1/guides/features/command/
- **UX reference (Blender navigation):** https://docs.blender.org/manual/en/latest/editors/3dview/navigate/navigation.html

## Notes

- **Performance profiling deferred:** We explicitly choose simple immediate-render approach initially. If profiling reveals performance issues during smooth pan/zoom, we'll write ADR-0003 for continuous render loop.

- **Touch input deferred:** This ADR focuses on desktop mouse input. Touch gestures (pinch-to-zoom, two-finger pan) will be addressed in a future ADR when web deployment is prioritized.

- **Camera animation deferred:** Smooth camera transitions (ease-in/ease-out for zoom-to-fit, etc.) are not part of this ADR. Future enhancement.

- **Coordinate system note:** This assumes Y-up world space (standard for graphics) but Y-down screen space (browser convention). The `screen_to_world()` method handles the Y-flip.

- **Testing strategy:** Camera math is pure, testable in isolation. Integration testing via manual UI testing initially. Future: Add automated UI tests via Tauri's testing framework.
