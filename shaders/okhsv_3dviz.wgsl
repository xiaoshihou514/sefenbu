#import bevy_pbr::forward_io::VertexOutput
#import "shaders/oklab_common.wgsl"::{
    HSV,
    RGB,
    okhsv_to_srgb
}

@group(2) @binding(0) var<uniform> h: f32;
@group(2) @binding(1) var<uniform> delta: f32;
@group(2) @binding(2) var<uniform> bottom: vec3<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var pos = mesh.world_position;
    pos.x -= bottom.x;
    pos.y -= bottom.y;
    pos.z -= bottom.z;

    let okhsv: HSV = HSV(h / 360., pos.x, pos.z);
    let rgb: RGB = okhsv_to_srgb(okhsv);

    return vec4<f32>(rgb.r, rgb.g, rgb.b, 1.);
}
