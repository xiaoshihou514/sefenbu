#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/oklab_common.wgsl"::{
    HSV,
    RGB,
    srgb_to_okhsv,
    to_non_linear_rgb
}

@group(2) @binding(0) var<uniform> h: f32;
@group(2) @binding(1) var<uniform> delta: f32;
@group(2) @binding(2) var img_texture: texture_2d<f32>;
@group(2) @binding(3) var img_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var pt: vec4<f32> = textureSample(img_texture, img_sampler, mesh.uv);
    let okhsv: HSV = srgb_to_okhsv(to_non_linear_rgb(pt.r, pt.g, pt.b));

    // Make opaque and grayscale if not in color slice
    if abs(okhsv.h * 360. - h) > (delta / 2.) {
        let avg = (pt.r + pt.g + pt.b) / 3;
        pt.r = avg;
        pt.g = avg;
        pt.b = avg;
        pt.a = 0.15;
    }

    return pt;
}
