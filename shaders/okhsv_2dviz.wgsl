#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/oklab_common.wgsl"::{
    pi,
    HSV,
    RGB,
    okhsv_to_srgb
}

@group(2) @binding(2) var img_texture: texture_2d<f32>;
@group(2) @binding(3) var img_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let pos = mesh.uv;

    var okhsv: HSV = HSV(pos.x, 1., 1.);
    var rgb: RGB = okhsv_to_srgb(okhsv);

    return vec4<f32>(rgb.r, rgb.g, rgb.b, 1.);
}
