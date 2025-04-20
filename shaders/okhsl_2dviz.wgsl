#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/oklab_common.wgsl"::{
    HSL,
    RGB,
    okhsl_to_srgb
}

@group(2) @binding(0) var<uniform> h: f32;
@group(2) @binding(1) var<uniform> delta: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pos = mesh.uv;

    let okhsl: HSL = HSL(0., 0., pos.x);
    let rgb: RGB = okhsl_to_srgb(okhsl);

    return vec4<f32>(rgb.r, rgb.g, rgb.b, 1.);
}
