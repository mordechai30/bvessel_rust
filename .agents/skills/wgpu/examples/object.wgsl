struct ObjectParams {
    tint: vec4<f32>,
    transform: mat4x4<f32>,
}
@group(0) @binding(0) var<uniform> object: ObjectParams;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tint: vec4<f32>,
}

// Consume one dynamic uniform record, whose binding size is 80 bytes.
// The allocation's device-aligned slot stride is not part of this ABI.
@vertex
fn vertex(@builtin(vertex_index) index: u32) -> VertexOutput {
    let positions = array(vec2(-0.5, -0.5), vec2(0.5, -0.5), vec2(0.0, 0.5));
    var output: VertexOutput;
    output.position = object.transform * vec4(positions[index], 0.0, 1.0);
    output.tint = object.tint;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.tint;
}
