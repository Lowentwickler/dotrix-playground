// STAGE: VERTEX ---------------------------------------------------------------------------------

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] cube: vec3<f32>;
};

[[block]]
struct SkyBox {
    proj_view: mat4x4<f32>;
    scale: mat4x4<f32>;
};
[[group(0), binding(0)]]
var u_skybox: SkyBox;

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.cube = position;
    out.position = vec4<f32>(position, 1.0);
    return out;
}


// STAGE: FRAGMENT -------------------------------------------------------------------------------
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
}
