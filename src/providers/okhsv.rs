use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OkhsvMaterial {
    #[uniform(0)]
    pub h: f32,
    #[uniform(1)]
    pub delta: f32,
    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Handle<Image>,
    _alpha_mode: AlphaMode2d,
}

impl OkhsvMaterial {
    pub fn new(h: f32, image: Handle<Image>) -> OkhsvMaterial {
        return OkhsvMaterial {
            h,
            delta: 10.0,
            color_texture: image,
            _alpha_mode: AlphaMode2d::Blend,
        };
    }
}

impl Material2d for OkhsvMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/okhsv.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self._alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Okhsv2DVizMaterial {
    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Handle<Image>,
    _alpha_mode: AlphaMode2d,
}

impl Okhsv2DVizMaterial {
    pub fn new(image: Handle<Image>) -> Okhsv2DVizMaterial {
        return Okhsv2DVizMaterial {
            color_texture: image,
            _alpha_mode: AlphaMode2d::Blend,
        };
    }
}

impl Material2d for Okhsv2DVizMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/okhsv_2dviz.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self._alpha_mode
    }
}
