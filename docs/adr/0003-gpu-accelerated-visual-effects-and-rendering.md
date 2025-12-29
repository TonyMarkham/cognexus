# ADR-0003: GPU-Accelerated Visual Effects and Rendering Architecture

**Status:** Accepted  
**Date:** 2024-12-19  
**Deciders:** Tony  

## Context

Cognexus is positioning itself as a high-performance alternative to existing workflow automation tools (n8n, Zapier, Make). These competitors use Canvas 2D or SVG rendering, which creates performance bottlenecks at scale and limits visual feedback capabilities. As a desktop-first application with WGPU rendering, we have access to GPU acceleration that competitors cannot leverage.

Key requirements driving this decision:

1. **Performance at Scale** - Must handle 500+ nodes with smooth 60fps panning/zooming
2. **Visual Execution Feedback** - Users should SEE workflows executing in real-time with visual effects
3. **Variable Node Shapes** - Different node types need different shapes (rectangles, circles, hexagons) without geometry overhead
4. **Smooth Connections** - Bézier curves between nodes must be anti-aliased and responsive
5. **Cost Awareness** - AI agent nodes consuming tokens should have visual feedback
6. **Desktop Advantage** - Leverage GPU capabilities that web Canvas 2D cannot match

The fundamental question: How do we render a node graph that is both performant AND visually engaging, using our GPU-accelerated architecture as a competitive advantage?

## Decision

Implement a comprehensive GPU-accelerated visual effects system using fragment shader-based rendering and instanced particle systems.

### 1. Fragment Shader SDF (Signed Distance Field) Rendering

**All shapes rendered using SDFs in fragment shaders, NOT geometry tessellation.**

```rust
// Node shape defined by type, not vertex count
pub struct Node {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub shape_type: ShapeType,
    pub corner_radius: f32,
    pub glow_intensity: f32,
}

pub enum ShapeType {
    RoundedRectangle,
    Circle,
    Hexagon,
    Diamond,
}
```

**Fragment shader example:**
```wgsl
@fragment
fn fs_node(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = in.uv * in.size - in.size * 0.5;
    
    // Calculate distance to shape using SDF
    var dist: f32;
    if (in.shape_type == 0u) {  // Rounded rectangle
        dist = sdf_rounded_box(pos, in.size * 0.5, in.corner_radius);
    } else if (in.shape_type == 1u) {  // Circle
        dist = length(pos) - in.size.x * 0.5;
    }
    
    // Anti-aliased edge
    let alpha = smoothstep(0.0, 1.0, -dist);
    
    // Optional glow effect for selected/active nodes
    let glow_dist = dist + 10.0 * in.glow_intensity;
    let glow_alpha = smoothstep(10.0, 0.0, glow_dist) * 0.6;
    
    let total_alpha = max(alpha, glow_alpha);
    return vec4<f32>(in.color.rgb, total_alpha);
}
```

**Rationale:** 
- ANY 2D shape possible with same 6-vertex quad
- Variable corner radius per node (no geometry regeneration)
- Pixel-perfect at any zoom level
- Anti-aliasing built into shader
- Glow effects are essentially free (just math)

### 2. Adaptive Connection Rendering

**Connections use adaptive rendering based on distance and alignment.**

```rust
const CURVE_START_DISTANCE: f32 = 60.0;   // Start adding curve
const CURVE_FULL_DISTANCE: f32 = 120.0;   // Full curve amount
const AXIS_ALIGNMENT_THRESHOLD: f32 = 10.0; // Consider straight if aligned

enum ConnectionGeometry {
    Straight { start: Vec2, end: Vec2 },
    Bezier { 
        start: Vec2, 
        end: Vec2, 
        control1: Vec2, 
        control2: Vec2 
    },
}

fn calculate_connection_geometry(from: Vec2, to: Vec2) -> ConnectionGeometry {
    let delta = to - from;
    let distance = delta.length();
    
    // Rule 1: Short connections - straight
    if distance < CURVE_START_DISTANCE {
        return ConnectionGeometry::Straight { start: from, end: to };
    }
    
    // Rule 2: Axis-aligned - straight (even if long)
    if delta.y.abs() < AXIS_ALIGNMENT_THRESHOLD || 
       delta.x.abs() < AXIS_ALIGNMENT_THRESHOLD {
        return ConnectionGeometry::Straight { start: from, end: to };
    }
    
    // Rule 3: Long diagonal - use smooth S-curve Bézier
    let curve_amount = calculate_curve_amount(distance);
    let offset = (distance * 0.4).min(100.0) * curve_amount;
    
    ConnectionGeometry::Bezier {
        start: from,
        end: to,
        control1: from + Vec2::new(offset, 0.0),
        control2: to - Vec2::new(offset, 0.0),
    }
}
```

**Rationale:**
- Short connections don't need curves (cleaner, fewer vertices)
- Horizontal/vertical connections stay straight (intentional alignment)
- Diagonal connections get smooth S-curves (organic flow)
- Smooth transition prevents "pop" between straight/curved
- 2-3x fewer vertices than all-Bézier approach

**Alternatives Considered:**
- **Rectilinear (orthogonal) routing** - Rejected: Looks robotic, doesn't fit modern workflow aesthetics
- **Manual routing with editable knots** - Deferred to Phase 2: Too complex for MVP, users won't ask for it initially
- **All connections always curved** - Rejected: Unnecessary curves on short/aligned connections look worse

### 3. Z-Ordering Strategy

**Render layers from back to front:**

```
Z = 0.0  : Nodes (background layer)
Z = 0.1  : Connections (middle layer) 
Z = 0.2  : Ports (front layer, always clickable)
Z = 0.25 : Particle effects (above all geometry)
```

**Rationale:**
- Connections visible on top of node bodies (traceable)
- Connections slide under ports (clean endpoints)
- Ports always accessible for interaction
- Matches industry standard (Unreal Blueprint, Blender nodes)

**Alternative Considered:**
- **Connections behind nodes** - Rejected: Connections disappear behind large nodes, making them hard to trace

### 4. GPU Particle System Architecture

**Instanced particle rendering for visual effects.**

```rust
struct ParticleSystem {
    particles: Vec<Particle>,
    effect_type: EffectType,
    spawn_time: f32,
}

enum EffectType {
    DataFlow { connection: ConnectionId },
    NodeActivation { node: NodeId },
    TokenBurn { node: NodeId, tokens_per_sec: f32 },
    Success { position: Vec2 },
    Error { position: Vec2 },
}

struct Particle {
    position: Vec2,
    velocity: Vec2,
    lifetime: f32,
    particle_id: u32,  // For shader-based randomness
}
```

**Visual Effects:**

1. **Data Flow Visualization** - Particles stream along connections during execution
2. **Node State Feedback** - Glows, sparks, vortexes indicate processing state
3. **Token Consumption** - Orbiting rings visualize AI token burn rate
4. **Execution Events** - Fireworks on success, explosions on error
5. **Cost Awareness** - Ring color (gold→orange→red) indicates expense

**Example: Token Burn Visualization**

```wgsl
// Sonic-style rings orbit agent nodes, spiral inward as consumed
@fragment
fn fs_token_ring(in: VertexOutput) -> @location(0) vec4<f32> {
    // Phase 1: Orbit node (1.5s)
    // Phase 2: Spiral inward (0.5s) 
    // Phase 3: Consumed (disappear)
    
    let orbit_progress = min(in.lifetime / 1.5, 1.0);
    let angle = orbit_progress * 6.28 * 2.0;  // 2 full rotations
    
    // Gold ring with shine
    let gold = vec3<f32>(1.0, 0.8, 0.0);
    let shine = sin(in.time * 10.0) * 0.3 + 0.7;
    
    return vec4<f32>(gold * shine, in.alpha);
}
```

**Performance:**
- 1000 particles = 1 draw call (instanced)
- GPU parallel processing
- ~0.5ms GPU time at 1000 particles
- Canvas 2D equivalent: 20ms+ CPU time

### 5. Node Collision and Placement System

**AABB collision detection with minimum spacing enforcement.**

```rust
const MIN_NODE_SPACING: f32 = 20.0;      // Unconnected nodes
const CONNECTED_NODE_SPACING: f32 = 60.0; // Nodes with connections
const MIN_CONNECTION_LENGTH: f32 = 50.0;  // Minimum visible connection

fn check_collision_with_spacing(
    dragging_node: &Node,
    other_nodes: &[Node],
) -> bool {
    let dragging_aabb = expand_aabb(&dragging_node.aabb(), MIN_NODE_SPACING / 2.0);
    
    for other in other_nodes {
        let other_aabb = expand_aabb(&other.aabb(), MIN_NODE_SPACING / 2.0);
        
        if aabb_intersects(&dragging_aabb, &other_aabb) {
            return true;
        }
    }
    false
}
```

**Ghost Preview on Collision:**
- Dragging node turns red when collision detected
- Semi-transparent "ghost" shows nearest valid position
- On release: Node snaps to ghost position (always valid)

**Rationale:**
- Ensures all connections are visible (minimum 50px length)
- Ports have room for interaction (no overlapping hit boxes)
- Professional appearance (consistent spacing)

## Consequences

### Positive

**Performance Advantages:**
- 500+ nodes at 60fps (competitors lag at 200 nodes)
- Particle effects essentially free (GPU parallel processing)
- No performance degradation with visual effects enabled
- Sub-millisecond per-node rendering

**Visual Quality:**
- Anti-aliased shapes at any zoom level
- Smooth curves without aliasing artifacts
- Professional-grade visual effects (glows, particles, animations)
- Pixel-perfect rendering

**Competitive Differentiation:**
- ONLY workflow tool with real-time execution visualization
- Desktop GPU capabilities competitors cannot match
- "Gamified" execution feedback makes workflows engaging
- Visual cost awareness (token consumption) is unique

**Development Velocity:**
- Add new shapes without geometry code (just shader SDF functions)
- Particle effects reusable across features
- Visual debugging built-in (see execution flow)

### Negative

**GPU Dependency:**
- Requires GPU with WGPU support (WebGPU/Metal/Vulkan/DX12)
- Won't run on very old hardware (<2015)
- Fallback rendering path not planned

**Shader Expertise Required:**
- Team must understand WGSL shader language
- Debugging shaders harder than CPU code
- Visual effects require graphics programming knowledge

**Platform Variability:**
- Different GPUs may render effects slightly differently
- Driver bugs can affect rendering
- Mobile deployment requires careful performance testing

**Mitigation:**
- Document shader patterns extensively
- Create reusable SDF function library
- Target modern hardware (5 years old or newer)
- Provide "reduced effects" mode for lower-end systems

### Neutral

**Trade-offs:**
- Complexity in shaders vs complexity in geometry management (chose shaders)
- Visual richness vs minimalist aesthetic (chose richness for differentiation)
- Development time on effects vs features (effects ARE the feature)

## Alternatives Considered

### 1. Canvas 2D Rendering (Like n8n)

**Approach:** Use HTML5 Canvas 2D API for all rendering.

**Pros:**
- Simpler to implement
- Widely understood API
- No GPU dependency

**Cons:**
- Performance ceiling (~200 nodes before lag)
- No particle effects at scale
- Jaggy edges, poor anti-aliasing
- Cannot differentiate from competitors

**Verdict:** Rejected - gives up our main competitive advantage

### 2. Texture-Based Shape Rendering

**Approach:** Pre-render shapes to PNG textures, render as textured quads.

**Pros:**
- Can use design tools (Figma) to create shapes
- Familiar workflow

**Cons:**
- Fixed shapes (no runtime variable corner radius)
- Memory overhead (texture per shape variation)
- Scaling issues (blurry when zoomed)
- Cannot dynamically change colors/styles

**Verdict:** Rejected for nodes/connections, Used for icons/decorations

### 3. Geometry Tessellation on CPU

**Approach:** Generate vertices for rounded shapes on CPU, upload to GPU.

**Pros:**
- Maximum control over shape
- No shader complexity

**Cons:**
- 20+ vertices per rounded rectangle (vs 6 with SDF)
- CPU cost for tessellation
- Regenerate geometry on size/radius change
- More GPU memory usage

**Verdict:** Rejected - SDF approach is superior in every metric

### 4. All-Bézier Connections

**Approach:** Every connection uses Bézier curve, no straight lines.

**Pros:**
- Consistent visual style
- Simpler code (one path)

**Cons:**
- Unnecessary curves on short connections
- Looks worse for axis-aligned nodes
- 2x more vertices than adaptive approach
- Subtle curves look like rendering bugs

**Verdict:** Rejected - adaptive approach looks better and performs better

## Implementation Notes

### Key Files to Create/Modify:

```
backend/renderer/src/
  shaders/
    sdf_functions.wgsl        # Reusable SDF library
    node.wgsl                 # Node rendering with shapes
    connection.wgsl           # Connection rendering
    particle.wgsl             # Particle effects
    
  systems/
    particle_system.rs        # Particle spawning/updating
    collision_system.rs       # AABB collision detection
    
backend/model/src/
  node.rs                     # Add shape_type, glow_intensity fields
  connection.rs               # Add geometry type (straight/bezier)
  particle.rs                 # Particle data structures
```

### Dependencies to Add:

```toml
# Already have:
wgpu = "0.18"
glam = "0.24"

# No additional dependencies needed
# (SDF functions are pure math, particle system is custom)
```

### Migration Strategy:

**Phase 1: SDF Rendering (Current)**
- Replace quad shader with SDF-based node shader
- Add shape_type to Node struct
- Implement rounded rectangle + circle SDFs

**Phase 2: Adaptive Connections (Next)**
- Implement connection geometry calculation
- Add straight line rendering path
- Smooth transition between straight/curved

**Phase 3: Particle System (After connections work)**
- Build particle spawning/updating infrastructure
- Add data flow visualization
- Add node state effects (glows, sparks)

**Phase 4: Advanced Effects (Polish)**
- Token burn visualization
- Success/error celebrations
- Execution timeline playback

### Breaking Changes:

- Node struct gains new fields (shape_type, corner_radius, glow_intensity)
- Connection rendering requires geometry calculation before upload
- Particle system adds new render pass

## References

**SDF Rendering:**
- [Inigo Quilez - 2D Distance Functions](https://iquilezles.org/articles/distfunctions2d/)
- [Shadertoy - SDF Examples](https://www.shadertoy.com/results?query=sdf)

**Bézier Curves:**
- [Primer on Bézier Curves](https://pomax.github.io/bezierinfo/)

**GPU Particle Systems:**
- [GPU Gems 3 - Chapter 23: Particle System](https://developer.nvidia.com/gpugems/gpugems3/part-iv-image-effects/chapter-23-high-speed-gpu-particle-simulation)

**Industry Examples:**
- Unreal Engine Blueprint Editor (Bézier connections, particle effects)
- Blender Shader Nodes (SDF rendering, GPU-accelerated)
- Figma (SDF for vector shapes, GPU rendering)

**Related ADRs:**
- ADR-0001: Camera and Grid System
- ADR-0002: Camera Control Input System

## Notes

**Design Philosophy:**

This ADR reflects a fundamental strategic decision: **Cognexus competes on USER EXPERIENCE, not feature parity.** 

By leveraging GPU rendering capabilities that web-based competitors cannot access, we create a visual and performance moat. The goal is that users FEEL the difference within 30 seconds of using the tool - smooth panning, beautiful effects, real-time execution visualization.

**"Gamification" is Intentional:**

The particle effects, token visualization, and execution feedback are not frivolous. They serve three purposes:

1. **Functional** - Visual debugging, performance profiling, cost awareness
2. **Educational** - Users learn how workflows execute through observation
3. **Emotional** - Satisfying visual feedback creates tool attachment

Desktop software can be DELIGHTFUL in ways cloud software cannot. We exploit this advantage.

**Future Considerations:**

- **VR/AR support** - SDF rendering and particle systems are VR-ready
- **Accessibility mode** - Reduced effects for motion sensitivity
- **Recording/replay** - Particle systems enable execution timeline recording
- **Custom particle skins** - Let users theme their workflow executions

**Target Audience Validation:**

This approach is validated by competitor analysis:
- n8n users complain about performance at scale (GitHub issues)
- Zapier users want local execution (trust/privacy concerns)  
- Make users want better visual feedback (execution is opaque)

We address all three with this architecture: performance through GPU, local execution through desktop, visual feedback through effects.

---

**Status Update:** This ADR moves from Proposed to Accepted as of 2024-12-19. Initial fragment shader SDF implementation completed and tested. Performance targets met (1000 quads at 60fps). Visual quality exceeds expectations. Proceeding with Phase 2 (adaptive connections).
