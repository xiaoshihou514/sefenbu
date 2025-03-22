use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

const SHADER_PATH: &str = "shaders/okhsv.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OkhsvMaterial {
    #[uniform(0)]
    h: f32,
    #[uniform(1)]
    delta: f32,
    #[texture(2)]
    #[sampler(3)]
    color_texture: Handle<Image>,
    _alpha_mode: AlphaMode,
}

impl OkhsvMaterial {
    pub fn new(h: f32, image: Handle<Image>) -> OkhsvMaterial {
        return OkhsvMaterial {
            h,
            delta: 1.0,
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

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
