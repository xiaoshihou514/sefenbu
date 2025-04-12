use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

use super::{generic::Provider, oklab_common::{Hsv, Lab}};

// global state
#[derive(Resource)]
pub struct OkhsvProvider {
    pub filter: OkhsvMaterial,
    pub viz2d_material: Okhsv2DVizMaterial,
}

const OKHSV_DELTA: f32 = 2.;
impl Provider for OkhsvProvider {
    #[rustfmt::skip]
    fn max(&self) -> f32 { 360. }
    #[rustfmt::skip]
    fn min(&self) -> f32 { 0. }
    #[rustfmt::skip]
    fn delta(&self) -> f32 { OKHSV_DELTA }

    fn decr(&mut self, change: f32) {
        self.filter.h -= change;
        // overflow protection
        self.filter.h = self.filter.h.max(0.);
        self.viz2d_material.h -= change;
        // overflow protection
        self.viz2d_material.h = self.filter.h.max(0.);
    }

    fn incr(&mut self, change: f32) {
        self.filter.h += change;
        // overflow protection
        self.filter.h = self.filter.h.min(360.);
        self.viz2d_material.h += change;
        // overflow protection
        self.viz2d_material.h = self.filter.h.min(360.);
    }
    
    fn convert(&self,pixel:&Oklaba) -> i64{
        let okhsv: Hsv = Hsv::from(&Lab {
            L: pixel.lightness,
            a: pixel.a,
            b: pixel.b,
        });
        (okhsv.h * 360. / self.delta()) as i64 * (self.delta() as i64)
    }
}

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
            delta: OKHSV_DELTA,
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
    #[uniform(0)]
    pub h: f32,
    #[uniform(1)]
    pub delta: f32,
    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Handle<Image>,
    _alpha_mode: AlphaMode2d,
}

impl Okhsv2DVizMaterial {
    pub fn new(h: f32, image: Handle<Image>) -> Okhsv2DVizMaterial {
        return Okhsv2DVizMaterial {
            h,
            delta: OKHSV_DELTA,
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
