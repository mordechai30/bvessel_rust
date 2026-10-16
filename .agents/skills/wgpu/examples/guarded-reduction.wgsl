@group(0) @binding(0) var<storage, read> input: array<f32>;
@group(0) @binding(1) var<storage, read_write> partials: array<f32>;
@group(0) @binding(2) var<uniform> logical_count: u32;
var<workgroup> tile: array<f32, 64>;

// Write one partial sum per workgroup, with neutral values for inactive lanes.
// A later global reduction requires another dispatch, even within the same pass.
@compute @workgroup_size(64)
fn main(@builtin(local_invocation_index) lane: u32,
        @builtin(global_invocation_id) gid: vec3<u32>,
        @builtin(workgroup_id) group: vec3<u32>) {
    var value = 0.0;
    if (gid.x < logical_count && gid.x < arrayLength(&input)) {
        value = input[gid.x];
    }
    tile[lane] = value;
    workgroupBarrier();
    if (lane == 0u && group.x < arrayLength(&partials)) {
        var sum = 0.0;
        for (var index = 0u; index < 64u; index++) {
            sum += tile[index];
        }
        partials[group.x] = sum;
    }
}
