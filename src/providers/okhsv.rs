use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

use crate::COLOR_3D_VIZ_COORD;

use super::{
    generic::Provider,
    oklab_common::{Hsv, Lab},
};

// global state
#[derive(Resource)]
pub struct OkhsvProvider {
    pub filter: OkhsvMaterial,
    pub viz2d_material: Okhsv2DVizMaterial,
    pub viz3d_material: Okhsv3DVizMaterial,
}

const OKHSV_DELTA: f32 = 2.;
const OKHSV_MAX: f32 = 360.;
const OKHSV_MIN: f32 = 0.;
impl Provider for OkhsvProvider {
    #[rustfmt::skip]
    fn max(&self) -> f32 { OKHSV_MAX }
    #[rustfmt::skip]
    fn min(&self) -> f32 { OKHSV_MIN }
    #[rustfmt::skip]
    fn delta(&self) -> f32 { OKHSV_DELTA }
    #[rustfmt::skip]
    fn current(&self) -> f32 { self.filter.h }

    fn decr(&mut self, change: f32) {
        // overflow protection
        self.filter.h = OKHSV_MIN.max(self.filter.h - change);
        self.viz2d_material.h = OKHSV_MIN.max(self.viz2d_material.h - change);
        self.viz3d_material.h = OKHSV_MIN.max(self.viz3d_material.h - change);
    }

    fn incr(&mut self, change: f32) {
        // overflow protection
        self.filter.h = OKHSV_MAX.min(self.filter.h + change);
        self.viz2d_material.h = OKHSV_MAX.min(self.viz2d_material.h + change);
        self.viz3d_material.h = OKHSV_MAX.min(self.viz3d_material.h + change);
    }

    fn convert(&self, p: Color) -> i64 {
        let pixel: Oklaba = p.into();
        let okhsv: Hsv = Hsv::from(&Lab {
            L: pixel.lightness,
            a: pixel.a,
            b: pixel.b,
        });
        (okhsv.h * OKHSV_MAX / OKHSV_DELTA) as i64 * (OKHSV_DELTA as i64)
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
        OkhsvMaterial {
            h,
            delta: OKHSV_DELTA,
            color_texture: image,
            _alpha_mode: AlphaMode2d::Blend,
        }
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
    _alpha_mode: AlphaMode2d,
}

impl Okhsv2DVizMaterial {
    pub fn new(h: f32) -> Self {
        Okhsv2DVizMaterial {
            h,
            delta: OKHSV_DELTA,
            _alpha_mode: AlphaMode2d::Blend,
        }
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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Okhsv3DVizMaterial {
    #[uniform(0)]
    pub h: f32,
    #[uniform(1)]
    pub delta: f32,
    #[uniform(2)]
    pub bottom: Vec3,
    _alpha_mode: AlphaMode,
}

impl Okhsv3DVizMaterial {
    pub fn new(h: f32) -> Self {
        Okhsv3DVizMaterial {
            h,
            delta: OKHSV_DELTA,
            bottom: COLOR_3D_VIZ_COORD - Vec3::new(0.5, 0.5, 0.5),
            _alpha_mode: AlphaMode::Blend,
        }
    }
}
impl Material for Okhsv3DVizMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/okhsv_3dviz.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self._alpha_mode
    }
}
