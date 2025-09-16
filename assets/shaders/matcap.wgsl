#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::view

@group(2) @binding(0) var matcap_texture: texture_2d<f32>;
@group(2) @binding(1) var matcap_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    // https://github.com/nidorx/matcaps?tab=readme-ov-file#applying-matcaps
    let view_normal = normalize((view.view_from_world * vec4<f32>(in.world_normal, 0.0)).xyz);
    let matcap_uv = view_normal.xy * 0.5 + 0.5;

    return textureSample(matcap_texture, matcap_sampler, vec2<f32>(matcap_uv.x, 1.0 - matcap_uv.y));
}
