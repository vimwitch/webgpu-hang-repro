const tuple_size = 4u;
const tuple_size_double = 2u * tuple_size;

///////
// Increase this value to increase the amount of time the program hangs
// Set to 1 to avoid hanging
///////
const iterations = 512u;

const size = iterations * tuple_size_double * tuple_size_double;

// Original multi-dimensional array
// var<workgroup> mul_16_results: array<array<array<u32, tuple_size_double>, tuple_size_double>, iterations>;

// Minimal repro single dimensional
var<workgroup> mul_16_results: array<u32, size>;

///////
// Problem does not occur with storage memory
///////

// @group(0)
// @binding(0)
// var<storage, read_write> unused: array<array<array<u32, tuple_size_double>, tuple_size_double>, iterations>;

// Problem occurs regardless of workgroup size
@compute
@workgroup_size(64)
fn test_mul(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>
) {
    var mul_16_count = tuple_size * 2u; // 8
    var mul_16_index = 2u * global_id.x * 128u + 2u * local_id.x;
    var mul_index = mul_16_index / mul_16_count;
    var limb_index = (mul_16_index % mul_16_count) / 2u;

    for (var i: u32; i < tuple_size_double; i++) {
        mul_16_results[mul_index * 2u * limb_index] = i;
        // mul_16_results[mul_index][2u * limb_index + 1u][i] = i + 10u;
        // mul_16_results[mul_index][2u * limb_index][i] = i;
        // mul_16_results[mul_index][2u * limb_index + 1u][i] = i + 10u;
    }
}
