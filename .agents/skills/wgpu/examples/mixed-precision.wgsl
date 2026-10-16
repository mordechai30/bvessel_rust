enable f16;
@group(0) @binding(0) var<storage, read> pairs: array<vec2<f16>>;
@group(0) @binding(1) var<storage, read_write> energy: array<f32>;
@group(0) @binding(2) var<uniform> logical_count: u32;

// Store compact half inputs, but square and accumulate converted f32 operands.
// The guard checks live count and both bound capacities before either load/store.
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let index = gid.x;
    if (index >= logical_count || index >= arrayLength(&pairs) || index >= arrayLength(&energy)) {
        return;
    }
    let pair = vec2<f32>(pairs[index]);
    energy[index] = dot(pair, pair);
}
