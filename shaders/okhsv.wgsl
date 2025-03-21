#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> h: f32;
@group(2) @binding(1) var img_texture: texture_2d<f32>;
@group(2) @binding(2) var img_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return textureSample(img_texture, img_sampler, mesh.uv);
}
