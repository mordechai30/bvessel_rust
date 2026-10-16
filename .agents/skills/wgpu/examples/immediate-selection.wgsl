struct DrawSelection {
    object_index: u32,
    material_index: u32,
}
var<immediate> selection: DrawSelection;
@group(0) @binding(0) var<storage, read> objects: array<vec4<f32>>;
@group(0) @binding(1) var<storage, read> materials: array<vec4<f32>>;

// Record eight immediate bytes on the render pass using the selected layout.
// Real draws may instead select transforms or other persistent storage records.
@vertex
fn vertex(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let positions = array(vec2(-0.5, -0.5), vec2(0.5, -0.5), vec2(0.0, 0.5));
    var offset = vec4(0.0);
    if (selection.object_index < arrayLength(&objects)) {
        offset = objects[selection.object_index];
    }
    return vec4(positions[index] + offset.xy, 0.0, 1.0);
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    if (selection.material_index < arrayLength(&materials)) {
        return materials[selection.material_index];
    }
    return vec4(0.0);
}
