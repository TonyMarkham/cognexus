# ADR-0004: Node Visual Style System with YAML DSL

**Status:** Proposed  
**Date:** 2024-12-29  
**Deciders:** Tony  
**Supersedes:** Extends ADR-0003 (GPU-Accelerated Visual Effects and Rendering)

## Context

Cognexus needs a flexible, extensible system for defining node visual appearance that serves both first-party nodes and third-party plugins. The system must support:

1. **Diverse Shape Library** - Standard flowchart shapes (rectangles, hexagons, diamonds) plus custom shapes (stored data, document, database symbols)
2. **Port Positioning** - Deterministic placement of connection ports on shape boundaries
3. **Plugin Extensibility** - Third-party developers can define custom node appearances without modifying core code
4. **Designer-Friendly** - Non-programmers can create and modify node shapes
5. **Performance** - Leverage GPU acceleration (from ADR-0003) without geometry regeneration
6. **Hot-Reload** - Change shapes during development without recompilation
7. **Dogfooding** - First-party and plugin nodes use identical systems

### Current Architecture Context

From ADR-0003, we have:
- Fragment shader SDF rendering for shapes
- Instanced rendering (1 draw call for N nodes)
- Transform matrices in vertex shader (scale, rotate already supported)
- Shape types represented as integers in instance data

### Key Design Challenges

**Challenge 1: Complex Shape Boundaries**
- Simple shapes (rectangle, circle) have trivial edge definitions
- Complex shapes (stored data = semicircle + chevron, document = curved bottom) have mixed geometry (straight lines + arcs + bezier curves)
- Port placement requires identifying and referencing specific edges
- Shape must be manifold (closed loop) for fill rendering

**Challenge 2: Port Positioning**
- Ports sit directly on shape boundary (n8n style), not inset (Unity GraphView style)
- Number of ports varies per node instance (1 input vs 5 inputs)
- Automatic distribution (evenly space 3 ports) vs explicit placement (port at exact position)
- Must calculate position + normal vector for connection direction

**Challenge 3: Text Layout**
- Observing n8n: text is rendered OUTSIDE shapes, not inside
- Node name below shape, centered with fixed padding
- Icon/symbol inside shape
- Avoids text scaling/wrapping complexity

**Challenge 4: Plugin Authorship**
- Plugin authors shouldn't need to write Rust code for visual appearance
- Visual changes shouldn't require recompilation
- Shape library should be reusable across different node types
- Same shape (e.g., hexagon) used by multiple node types with different port configurations

## Decision

Implement a **dual-DSL system** using YAML to declaratively define node visual appearance, separated into **shape geometry** (reusable) and **node-type styling** (composes shapes with ports/icons/colors).

### 1. Two-DSL Architecture

**Separation of Concerns:**

```
Shape DSL (shapes/*.yaml)
  ↓ defines geometry
Node-Type DSL (node-types/*.yaml)
  ↓ references shapes, adds ports/styling
Rust Types (ShapeDefinition, NodeVisualStyle)
  ↓ parsed at runtime
Renderer (SDF Shaders + Instancing)
  ↓ GPU rendering
Visual Output
```

**Why Two DSLs:**
- **Shapes** are reusable geometry primitives (one hexagon, many node types)
- **Node-types** compose shapes with specific ports, colors, transforms
- Designer can modify shape library without touching node logic
- Plugin can reference built-in shapes or ship custom shapes

### 2. Shape DSL Specification

**Shape Definition (shapes/hexagon.yaml):**

```yaml
shape:
  # Unique identifier
  id: hexagon
  name: "Hexagon"
  description: "Six-sided polygon for preparation/subroutine nodes"
  
  # Base size (can be scaled by node-type)
  size: [100, 80]
  
  # Shape boundary - must form closed loop (manifold)
  edges:
    - id: top_left
      type: line
      start: [-25, -40]
      end: [-50, 0]
      port_zone: true      # Can place ports on this edge
      
    - id: bottom_left
      type: line
      start: [-50, 0]
      end: [-25, 40]
      port_zone: true
      
    - id: bottom
      type: line
      start: [-25, 40]
      end: [25, 40]
      port_zone: false     # Decorative, no ports
      
    - id: bottom_right
      type: line
      start: [25, 40]
      end: [50, 0]
      port_zone: true
      
    - id: top_right
      type: line
      start: [50, 0]
      end: [25, -40]
      port_zone: true
      
    - id: top
      type: line
      start: [25, -40]
      end: [-25, -40]
      port_zone: false
  
  # Anchor points for content placement
  anchors:
    # Icon/symbol inside shape
    icon:
      position: [0, 0]      # Center of shape
      max_size: [40, 40]
    
    # Text label below shape (n8n pattern)
    label:
      position: [0, 55]     # Below shape + padding
      alignment: center
      max_width: 120
    
    # Optional secondary text (status/error)
    sublabel:
      position: [0, 70]
      alignment: center
      max_width: 120
      style: small
```

**Shape Definition (shapes/stored_data.yaml) - Complex Shape:**

```yaml
shape:
  id: stored_data
  name: "Stored Data"
  description: "Semicircle left, chevron right (tape/disk storage)"
  size: [100, 100]
  
  edges:
    - id: left
      type: arc
      center: [-50, 0]
      radius: 50
      start_angle: -90     # Bottom of semicircle
      end_angle: 90        # Top of semicircle
      port_zone: true
      
    - id: top
      type: line
      start: [-50, -50]
      end: [30, -50]
      port_zone: false
      
    - id: top_chevron
      type: line
      start: [30, -50]
      end: [50, 0]         # Chevron point
      port_zone: false
      
    - id: bottom_chevron
      type: line
      start: [50, 0]
      end: [30, 50]
      port_zone: false
      
    - id: bottom
      type: line
      start: [30, 50]
      end: [-50, 50]
      port_zone: false
  
  anchors:
    icon:
      position: [-10, 0]   # Slightly left of center
      max_size: [30, 30]
    label:
      position: [0, 65]
      alignment: center
```

**Shape Definition (shapes/document.yaml) - Curved Edges:**

```yaml
shape:
  id: document
  name: "Document"
  description: "Rectangle with wavy bottom edge"
  size: [100, 120]
  
  edges:
    - id: left
      type: line
      start: [-50, -60]
      end: [-50, 50]
      port_zone: true
      
    - id: bottom
      type: cubic_bezier
      p0: [-50, 50]
      p1: [-25, 55]
      p2: [25, 45]
      p3: [50, 50]
      port_zone: false     # Decorative wavy edge
      
    - id: right
      type: line
      start: [50, 50]
      end: [50, -60]
      port_zone: true
      
    - id: top
      type: line
      start: [50, -60]
      end: [-50, -60]
      port_zone: false
  
  anchors:
    icon:
      position: [0, -10]
      max_size: [40, 40]
    label:
      position: [0, 75]
      alignment: center
```

**Edge Geometry Types:**

```yaml
# Straight line
type: line
start: [x1, y1]
end: [x2, y2]

# Circular arc
type: arc
center: [cx, cy]
radius: r
start_angle: degrees    # -180 to 180
end_angle: degrees

# Cubic Bezier curve
type: cubic_bezier
p0: [x0, y0]           # Start point
p1: [x1, y1]           # Control point 1
p2: [x2, y2]           # Control point 2
p3: [x3, y3]           # End point
```

**Coordinate System:**
- Origin at shape center (0, 0)
- Positive X = right, Positive Y = down
- Units are design-time units (will be scaled by transform)
- Edge list must form closed loop (last edge end = first edge start)

**Validation Rules:**
- All edges must connect (manifold requirement)
- At least 3 edges (minimum polygon)
- Edge IDs must be unique within shape
- `port_zone` edges should be accessible (not obscured)

### 3. Node-Type DSL Specification

**Node-Type Definition (node-types/http_request.yaml):**

```yaml
node_type:
  # References NodeDefinition in Rust code
  definition_id: "a1b2c3d4-..."  # UUID
  
  # Visual styling
  visual:
    # Reference shape from library
    shape_ref: hexagon
    
    # Optional transform (applied to base shape)
    transform:
      scale: [1.2, 1.0]     # 120% width, 100% height
      skew: [0, 0]          # No skew
      rotation: 0           # No rotation
    
    # Base color (can be overridden by node state)
    color:
      r: 0.2
      g: 0.6
      b: 0.9
      a: 1.0
    
    # Icon reference (optional)
    icon: "icons/http.svg"
    
  # Port placement on shape edges
  ports:
    inputs:
      - name: url
        edge: top_left      # References edge ID from shape
        position: 0.5       # 0.0 = start, 1.0 = end, 0.5 = middle
        
      - name: headers
        edge: bottom_left
        position: 0.5
        
    outputs:
      - name: response
        edge: top_right
        position: 0.5
        
      - name: error
        edge: bottom_right
        position: 0.5
```

**Node-Type with Auto-Distribution:**

```yaml
node_type:
  definition_id: "e5f6g7h8-..."
  
  visual:
    shape_ref: rounded_rectangle
    color: { r: 0.9, g: 0.4, b: 0.2, a: 1.0 }
  
  ports:
    inputs:
      edge: left           # Single edge for all inputs
      distribution: even   # Auto-distribute based on actual port count
      
    outputs:
      edge: right
      distribution: even
```

**Port Placement Strategies:**

```yaml
# Explicit positioning
ports:
  inputs:
    - edge: left
      position: 0.25     # Specific position
    - edge: left
      position: 0.75

# Automatic even distribution
ports:
  inputs:
    edge: left
    distribution: even   # Runtime calculates positions

# Offset from edge start
ports:
  inputs:
    - edge: left
      offset: 10         # 10 units from edge start

# Multiple edges
ports:
  inputs:
    - edge: top_left
      position: 0.5
    - edge: bottom_left
      position: 0.5
```

**Transform Options:**

```yaml
transform:
  # Scale on X and Y independently
  scale: [1.5, 1.0]      # Wide rectangle
  
  # Skew (parallelogram effect)
  skew: [0.2, 0]         # Skew X by 0.2
  
  # Rotation (degrees)
  rotation: 45           # Diamond from square
```

### 4. Rust Type Definitions

```rust
// In backend/visual-style/ crate

/// Shape library loaded from YAML files
pub struct ShapeLibrary {
    shapes: HashMap<String, ShapeDefinition>,
}

/// Single shape definition (parsed from YAML)
pub struct ShapeDefinition {
    pub id: String,
    pub name: String,
    pub size: Vec2,
    pub edges: Vec<EdgeDefinition>,
    pub anchors: Anchors,
}

/// Named edge on shape boundary
pub struct EdgeDefinition {
    pub id: String,
    pub geometry: EdgeGeometry,
    pub port_zone: bool,
}

/// Edge geometry primitives
pub enum EdgeGeometry {
    Line { start: Vec2, end: Vec2 },
    Arc { center: Vec2, radius: f32, start_angle: f32, end_angle: f32 },
    CubicBezier { p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2 },
}

/// Anchor points for content
pub struct Anchors {
    pub icon: Anchor,
    pub label: Anchor,
    pub sublabel: Option<Anchor>,
}

pub struct Anchor {
    pub position: Vec2,
    pub alignment: Alignment,
    pub max_width: Option<f32>,
}

/// Node visual style (parsed from node-type YAML)
pub struct NodeVisualStyle {
    pub shape_id: String,
    pub transform: Transform,
    pub color: Color,
    pub icon: Option<String>,
    pub port_placements: PortPlacements,
}

pub struct Transform {
    pub scale: Vec2,
    pub skew: Vec2,
    pub rotation: f32,
}

pub struct PortPlacements {
    pub inputs: Vec<PortPlacement>,
    pub outputs: Vec<PortPlacement>,
}

pub enum PortPlacement {
    Explicit { edge_id: String, position: f32 },
    Offset { edge_id: String, offset: f32 },
    Distributed { edge_id: String, distribution: Distribution },
}

pub enum Distribution {
    Even,      // Evenly spaced
    Start,     // Grouped at start
    End,       // Grouped at end
}
```

### 5. Runtime Port Position Calculation

**Two-phase calculation:**

**Phase 1: Parse and Cache (at load time)**
```rust
impl ShapeLibrary {
    pub fn load_from_directory(path: &Path) -> Result<Self> {
        // 1. Scan for *.yaml files
        // 2. Parse each into ShapeDefinition
        // 3. Validate manifold (closed loop)
        // 4. Cache in HashMap by shape_id
    }
}

impl NodeVisualStyleRegistry {
    pub fn load_from_directory(path: &Path, shapes: &ShapeLibrary) -> Result<Self> {
        // 1. Scan for node-type *.yaml files
        // 2. Parse each into NodeVisualStyle
        // 3. Validate shape_ref exists in ShapeLibrary
        // 4. Validate edge references are valid
        // 5. Cache by definition_id
    }
}
```

**Phase 2: Calculate Positions (at render time)**
```rust
impl NodeVisualStyle {
    pub fn calculate_port_positions(
        &self,
        shape: &ShapeDefinition,
        input_count: usize,
        output_count: usize,
    ) -> PortLayout {
        let mut layout = PortLayout::new();
        
        // Apply transform to shape edges
        let transformed_edges = self.apply_transform(&shape.edges);
        
        // Calculate input positions
        for placement in &self.port_placements.inputs {
            let edge = transformed_edges.find(&placement.edge_id)?;
            let positions = match placement {
                Explicit { position } => vec![position],
                Distributed { distribution } => 
                    calculate_distribution(input_count, distribution),
            };
            
            for pos in positions {
                let (point, normal) = edge.evaluate_at(pos);
                layout.inputs.push(PortPosition { point, normal });
            }
        }
        
        // Same for outputs...
        layout
    }
}

impl EdgeGeometry {
    /// Calculate point and normal at parametric position t (0.0 to 1.0)
    pub fn evaluate_at(&self, t: f32) -> (Vec2, Vec2) {
        match self {
            Line { start, end } => {
                let point = start.lerp(*end, t);
                let tangent = (*end - *start).normalize();
                let normal = Vec2::new(-tangent.y, tangent.x); // Perpendicular
                (point, normal)
            }
            Arc { center, radius, start_angle, end_angle } => {
                let angle = start_angle.lerp(*end_angle, t).to_radians();
                let point = *center + Vec2::new(angle.cos(), angle.sin()) * *radius;
                let normal = (point - *center).normalize();
                (point, normal)
            }
            CubicBezier { p0, p1, p2, p3 } => {
                // De Casteljau's algorithm
                let point = cubic_bezier_point(*p0, *p1, *p2, *p3, t);
                let tangent = cubic_bezier_tangent(*p0, *p1, *p2, *p3, t);
                let normal = Vec2::new(-tangent.y, tangent.x).normalize();
                (point, normal)
            }
        }
    }
}
```

### 6. Renderer Integration

**Instance Data Extension:**

```rust
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NodeInstance {
    pub model_matrix: [[f32; 4]; 4],  // Existing: position + base scale
    pub color: [f32; 4],               // Existing: RGBA
    
    // NEW: Shape parameters
    pub shape_type: u32,               // SDF shape selector
    pub corner_radius: f32,            // For rounded shapes
    pub transform_scale: [f32; 2],     // Additional scale from node-type
    pub transform_skew: [f32; 2],      // Skew transform
    pub glow_intensity: f32,           // For selection/hover
    pub _padding: f32,                 // 16-byte alignment
}
```

**Shader Integration (WGSL):**

```wgsl
@fragment
fn fs_node(in: VertexOutput) -> @location(0) vec4<f32> {
    // Transform UV by instance parameters
    let scaled_uv = in.uv * in.transform_scale;
    let skewed_pos = apply_skew(scaled_uv, in.transform_skew);
    let local_pos = skewed_pos - vec2<f32>(0.5, 0.5);
    
    // Calculate SDF based on shape_type
    var dist: f32;
    switch (in.shape_type) {
        case 0u: { // Rectangle
            dist = sdf_box(local_pos, vec2<f32>(0.5, 0.5));
        }
        case 1u: { // Rounded Rectangle
            dist = sdf_rounded_box(local_pos, vec2<f32>(0.5, 0.5), in.corner_radius);
        }
        case 2u: { // Hexagon
            dist = sdf_hexagon(local_pos, 0.5);
        }
        // ... more shapes
    }
    
    // Anti-aliased edge
    let alpha = smoothstep(0.01, 0.0, dist);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
```

**For complex shapes (stored data, document):**
- Simple shapes use SDF in shader (fast, procedural)
- Complex shapes may need tesselation on CPU (acceptable tradeoff)
- Or use texture atlases for very complex custom shapes

### 7. Directory Structure

```
cognexus/
  shapes/                    # Shape library (built-in)
    rectangle.yaml
    rounded_rectangle.yaml
    circle.yaml
    diamond.yaml
    hexagon.yaml
    pentagon.yaml
    parallelogram.yaml
    trapezoid.yaml
    stored_data.yaml
    document.yaml
    database.yaml
    # ... flowchart symbol set
  
  node-types/                # First-party node visual styles
    start.yaml
    end.yaml
    http_request.yaml
    database_query.yaml
    code_function.yaml
    # ...
  
  plugins/
    my-custom-plugin/
      shapes/                # Plugin-specific shapes (optional)
        custom_shape.yaml
      node-types/            # Plugin node visual styles
        my_node.yaml
```

### 8. Plugin Workflow

**Plugin Author Workflow:**

1. **Use existing shape:**
```yaml
# my-plugin/node-types/weather_api.yaml
node_type:
  definition_id: "uuid-from-rust-code"
  visual:
    shape_ref: hexagon      # Reference built-in shape
    color: { r: 0.3, g: 0.7, b: 0.9, a: 1.0 }
  ports:
    inputs:
      edge: left
      distribution: even
    outputs:
      edge: right
      distribution: even
```

2. **Create custom shape:**
```yaml
# my-plugin/shapes/weather_cloud.yaml
shape:
  id: weather_cloud
  # ... custom geometry
```

```yaml
# my-plugin/node-types/weather_api.yaml
node_type:
  visual:
    shape_ref: weather_cloud   # Reference plugin's custom shape
```

3. **Package plugin:**
```
my-plugin/
  my_plugin.wasm             # Rust code (NodeDefinition impl)
  shapes/                    # Optional custom shapes
  node-types/                # Required visual definitions
  icons/                     # Optional icons
  plugin.yaml                # Metadata
```

4. **Runtime loads:**
- Scan plugin directory
- Load shapes (merge with built-in library)
- Load node-types (validate shape refs)
- Register NodeDefinition (WASM)
- Nodes appear in palette with correct visuals

## Consequences

### Positive

**For Plugin Authors:**
- No Rust code needed for visual appearance
- Designer-friendly (YAML + optional SVG icons)
- Can reuse built-in shape library
- Can ship custom shapes if needed
- Changes don't require recompilation
- Same system as first-party nodes (dogfooding validated)

**For Core Development:**
- Hot-reload shapes during development
- Designer/developer workflow separation
- Shape library grows independently of code
- Easy to add flowchart symbol sets
- Visual changes don't break node logic

**For Performance:**
- GPU transforms (scale/skew/rotate) are free
- Instanced rendering (1 draw call per frame)
- SDF rendering (perfect anti-aliasing at any zoom)
- Port positions calculated once per node type, cached
- No geometry regeneration on transform changes

**For Extensibility:**
- Two-DSL system allows independent evolution
- Shape library reusable across many node types
- Port placement strategies extensible
- Transform system can add more operations
- Edge geometry can add new primitives (elliptical arc, quadratic bezier)

### Negative

**Complexity:**
- Two parsers needed (shape YAML, node-type YAML)
- Validation at multiple stages (parse-time, load-time, render-time)
- Edge position calculation has geometric complexity (bezier curves)
- Error messages must be clear (YAML errors are notoriously cryptic)

**Limitations:**
- Complex shapes may need CPU tessellation (not pure SDF)
- Port auto-distribution may not match all use cases
- Transform system doesn't support arbitrary affine transforms
- Anchor points are static (can't animate text position)

**Learning Curve:**
- Plugin authors must learn YAML schema
- Understanding edge IDs and port placement requires geometry knowledge
- Bezier curve control points are non-intuitive for designers

### Neutral

**Trade-offs:**
- YAML vs JSON: Chose YAML for comments and readability
- Two DSLs vs one: Chose separation for reusability
- Edge-based vs coordinate-based port placement: Chose edge-based for shape independence
- SDF vs geometry: Using both (SDF for simple, geometry for complex)

**Mitigation Strategies:**
- Provide comprehensive examples for all flowchart shapes
- Build YAML validator with clear error messages
- Create visual editor tool (future) for WYSIWYG shape design
- Document common patterns (port placement strategies)
- Provide shape template library (copy-paste starting point)

## Alternatives Considered

### Alternative 1: Hardcoded Shapes in Rust

**Approach:** Define shapes as Rust enums/structs.

```rust
pub enum NodeShape {
    Rectangle,
    RoundedRectangle { corner_radius: f32 },
    Circle,
    Hexagon,
    Custom { vertices: Vec<Vec2> },
}
```

**Pros:**
- Type-safe
- No parsing overhead
- IDE autocomplete

**Cons:**
- Requires recompilation for new shapes
- No hot-reload
- Plugin authors must write Rust
- Doesn't dogfood (first-party gets easier path than plugins)

**Verdict:** Rejected - violates plugin extensibility and dogfooding requirements

### Alternative 2: SVG Path String

**Approach:** Use SVG path syntax directly.

```yaml
shape:
  path: "M -50,-40 L -25,-60 L 25,-60 L 50,-40 L 50,40 L -50,40 Z"
```

**Pros:**
- Industry standard
- Can import from design tools
- Compact representation

**Cons:**
- Parsing complexity (string → geometry)
- No semantic edge IDs (how to reference "left side"?)
- SVG arc syntax is notoriously complex
- Harder to validate

**Verdict:** Rejected for primary format, but could be added as import/export feature

### Alternative 3: Single Unified DSL

**Approach:** Combine shape geometry and node-type styling in one file.

```yaml
node:
  name: "HTTP Request"
  shape:
    edges:
      - id: left
        type: line
        # ...
  ports:
    # ...
```

**Pros:**
- One file per node
- All information in one place

**Cons:**
- Shape duplication (10 nodes use hexagon = 10 copies of hexagon definition)
- Can't reuse shapes
- Harder to maintain shape library
- Mixing concerns (geometry + styling)

**Verdict:** Rejected - two-DSL separation is cleaner and more reusable

### Alternative 4: Code Generation from Design Tools

**Approach:** Export from Figma/Illustrator, generate Rust code.

**Pros:**
- Designer tools for shape creation
- Visual workflow

**Cons:**
- Build-time code generation complexity
- Generated code is unreadable
- Still requires recompilation
- Doesn't solve plugin extensibility

**Verdict:** Rejected as primary approach, but could complement YAML system (design in Figma → export to YAML)

### Alternative 5: Texture-Based Shapes

**Approach:** Pre-render shapes to PNG, use as textures.

**Pros:**
- Any arbitrary shape possible
- Designer-friendly (use any design tool)

**Cons:**
- Fixed resolution (blurry when zoomed)
- Memory overhead (one texture per shape variation)
- Can't parameterize (corner radius, size)
- Doesn't support transforms well (skew distorts texture)
- No dynamic colors (need texture per color)

**Verdict:** Rejected for shapes, but appropriate for icons/decorations

## Implementation Plan

### Phase 1: Foundation (Week 1)
- [ ] Create `backend/visual-style/` crate
- [ ] Define Rust types (`ShapeDefinition`, `EdgeGeometry`, etc.)
- [ ] Implement YAML parser for shape DSL
- [ ] Implement shape validation (manifold check, edge continuity)
- [ ] Create 3 example shapes (rectangle, circle, hexagon)

### Phase 2: Node-Type DSL (Week 2)
- [ ] Implement YAML parser for node-type DSL
- [ ] Implement shape reference resolution
- [ ] Implement port position calculation
- [ ] Create example node-types (start, end, http_request)
- [ ] Unit tests for edge evaluation (lines, arcs, bezier)

### Phase 3: Renderer Integration (Week 3)
- [ ] Extend `NodeInstance` struct with transform parameters
- [ ] Update vertex shader with scale/skew transforms
- [ ] Implement SDF functions for basic shapes (in existing shader system)
- [ ] Wire `NodeVisualStyleRegistry` to renderer
- [ ] Render first nodes with YAML-defined appearance

### Phase 4: Complex Shapes (Week 4)
- [ ] Implement stored_data shape (arc + line)
- [ ] Implement document shape (bezier bottom edge)
- [ ] CPU tessellation fallback for non-SDF shapes
- [ ] Shape library completion (all flowchart symbols from reference image)

### Phase 5: Polish & Documentation (Week 5)
- [ ] Hot-reload support (watch YAML files in dev mode)
- [ ] Error message improvements
- [ ] YAML schema documentation
- [ ] Example gallery (all shapes + variations)
- [ ] Plugin author guide

### Phase 6: Advanced Features (Future)
- [ ] Visual shape editor (WYSIWYG tool)
- [ ] SVG import/export
- [ ] Animation support (transition between shapes)
- [ ] Theme system (color palette overrides)
- [ ] Shape variants (success/error state colors)

### Breaking Changes

**Data Model Changes:**
- `Node` in data model doesn't need shape info (comes from definition)
- Renderer receives shape ID + transform, not explicit geometry

**Renderer Changes:**
- Instance buffer gains transform parameters
- Vertex shader applies additional transforms
- Fragment shader interprets shape type integer

**Registry Changes:**
- New `ShapeLibrary` registry alongside `NodeDefinitionRegistry`
- New `NodeVisualStyleRegistry` maps definition_id → visual style
- Load from filesystem at startup

## References

**YAML Parsing:**
- [serde_yaml](https://docs.rs/serde_yaml/) - Rust YAML parser
- [YAML Specification](https://yaml.org/spec/1.2/spec.html)

**Geometry Algorithms:**
- [De Casteljau's Algorithm](https://en.wikipedia.org/wiki/De_Casteljau%27s_algorithm) - Bezier evaluation
- [Primer on Bézier Curves](https://pomax.github.io/bezierinfo/) - Interactive guide

**SDF Rendering:**
- [Inigo Quilez - 2D Distance Functions](https://iquilezles.org/articles/distfunctions2d/)
- Existing implementation in ADR-0003

**Industry Examples:**
- n8n - Node visual style (text below nodes, ports on boundary)
- Unreal Blueprint - Shape library system
- Figma - Vector shape primitives

**Related ADRs:**
- ADR-0003: GPU-Accelerated Visual Effects and Rendering (establishes SDF + instancing foundation)

## Future Considerations

**Visual Editor Tool:**
Build a GUI tool for shape creation:
- Draw shapes visually
- Define edge IDs interactively
- Preview port placement
- Export to YAML

**Procedural Shapes:**
Allow YAML to reference procedural generation:
```yaml
shape:
  type: procedural
  generator: polygon
  params:
    sides: 8
    radius: 50
```

**Animation System:**
Animate between shape states:
```yaml
animations:
  on_execute:
    duration: 0.5s
    property: glow_intensity
    from: 0.0
    to: 1.0
```

**Theme System:**
Override colors globally:
```yaml
theme:
  primary: "#4A90E2"
  secondary: "#50E3C2"
  error: "#E24A4A"
```

**Accessibility:**
- High-contrast mode (stronger colors, thicker edges)
- Shape patterns (in addition to colors for colorblind users)
- Screen reader descriptions (in YAML metadata)

---

**Status:** This ADR documents the long-term vision for the visual style system. Initial implementation will start with a simplified MVP (hardcoded shapes for Start/End nodes) to validate the rendering pipeline, then build toward this full DSL system incrementally.
