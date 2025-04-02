#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/oklab_common.wgsl"::{
    HSV,
    RGB,
    okhsv_to_srgb
}

@group(2) @binding(0) var<uniform> h: f32;
@group(2) @binding(1) var<uniform> delta: f32;
@group(2) @binding(2) var img_texture: texture_2d<f32>;
@group(2) @binding(3) var img_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pos = mesh.uv;

    let okhsv: HSV = HSV(pos.x, 1., 1.);
    let rgb: RGB = okhsv_to_srgb(okhsv);
    var a: f32 = 0.25;
    if abs(pos.x * 360. - h) < (delta / 2.) {
        a = 1.;
    }

    return vec4<f32>(rgb.r, rgb.g, rgb.b, a);
}
