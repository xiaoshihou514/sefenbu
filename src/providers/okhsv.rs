use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

const SHADER_PATH: &str = "shaders/okhsv.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OkhsvMaterial {
    #[uniform(0)]
    h: f32,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
    _alpha_mode: AlphaMode,
}

impl OkhsvMaterial {
    pub fn new(h: f32, image: Handle<Image>) -> OkhsvMaterial {
        return OkhsvMaterial {
            h,
            color_texture: image,
            _alpha_mode: AlphaMode::Blend,
        };
    }

    pub fn update(self: &mut OkhsvMaterial, h: f32) {
        self.h = h;
    }
}

impl Material for OkhsvMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self._alpha_mode
    }
}
