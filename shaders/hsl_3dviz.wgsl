#import bevy_pbr::forward_io::VertexOutput
#import "shaders/hsl_common.wgsl"::{
    HSL,
    hsl_to_srgb
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

    let rgb: vec3<f32> = hsl_to_srgb(pos.x, pos.z, h / 100.);

    return vec4<f32>(rgb.r, rgb.g, rgb.b, 1.);
}
