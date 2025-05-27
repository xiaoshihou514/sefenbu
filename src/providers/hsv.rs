use crate::COLOR_3D_VIZ_COORD;
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};
use std::collections::HashMap;

use super::generic::{CSpaceProvider, FromImage, Provider};

// global state
#[derive(Resource)]
pub struct HsvProvider {
    pub filter: HsvMaterial,
    pub viz2d_material: Hsv2DVizMaterial,
    pub viz3d_material: Hsv3DVizMaterial,
    cache: HashMap<(u32, u32), Hsva>,
}

impl CSpaceProvider for HsvProvider {
    type FilterMaterial = HsvMaterial;
    type Viz2dMaterial = Hsv2DVizMaterial;
    type Viz3dMaterial = Hsv3DVizMaterial;

    fn get_filter(&self) -> Self::FilterMaterial {
        self.filter.clone()
    }

    fn get_viz2d_material(&self) -> Self::Viz2dMaterial {
        self.viz2d_material.clone()
    }

    fn get_viz3d_material(&self) -> Self::Viz3dMaterial {
        self.viz3d_material.clone()
    }
}

impl FromImage for HsvProvider {
    fn from_image(img: Handle<Image>) -> Self {
        HsvProvider {
            filter: HsvMaterial::from_image(img),
            viz2d_material: Hsv2DVizMaterial::default(),
            viz3d_material: Hsv3DVizMaterial::default(),
            cache: HashMap::new(),
        }
    }
}

const HSV_DELTA: f32 = 2.;
const HSV_SV_MAX: f32 = 100.;
const HSV_SV_DELTA: f32 = 2.;
impl Provider for HsvProvider {
    const MAX: f32 = 360.;
    const MIN: f32 = 0.;
    const DELTA: f32 = HSV_DELTA;

    const X_MAX: f32 = HSV_SV_MAX;
    const X_DELTA: f32 = HSV_SV_DELTA;
    const Z_MAX: f32 = HSV_SV_MAX;
    const Z_DELTA: f32 = HSV_SV_DELTA;

    #[rustfmt::skip]
    fn current(&self) -> f32 { self.filter.h }

    fn decr(&mut self, change: f32) {
        // overflow protection
        self.filter.h = Self::MIN.max(self.filter.h - change);
        self.viz2d_material.h = Self::MIN.max(self.viz2d_material.h - change);
        self.viz3d_material.h = Self::MIN.max(self.viz3d_material.h - change);
    }

    fn incr(&mut self, change: f32) {
        // overflow protection
        self.filter.h = Self::MAX.min(self.filter.h + change);
        self.viz2d_material.h = Self::MAX.min(self.viz2d_material.h + change);
        self.viz3d_material.h = Self::MAX.min(self.viz3d_material.h + change);
    }

    fn set(&mut self, new: f32) {
        let new_adjusted = (new * Self::MAX / Self::DELTA) as i64 as f32 * Self::DELTA;
        self.filter.h = new_adjusted;
        self.viz2d_material.h = new_adjusted;
        self.viz3d_material.h = new_adjusted;
    }

    fn convert(&self, c: Color) -> i64 {
        let hsv: Hsva = c.into();
        (hsv.hue / Self::DELTA) as i64 * (Self::DELTA as i64)
    }

    fn inspect(&mut self, img: &Image, i: u32, j: u32) -> (bool, (i64, i64)) {
        // cache colors
        let hsv = match self.cache.get(&(i, j)) {
            Some(c) => *c,
            None => {
                let c: Hsva = img.get_color_at(i, j).unwrap().into();
                self.cache.insert((i, j), c);
                c
            }
        };
        if (hsv.hue - self.current()).abs() > Self::DELTA {
            // within range
            (
                true,
                (
                    (Into::<f32>::into(hsv.saturation) * HSV_SV_MAX / HSV_SV_DELTA) as i64
                        * (HSV_SV_DELTA as i64),
                    (Into::<f32>::into(hsv.value) * HSV_SV_MAX / HSV_SV_DELTA) as i64
                        * (HSV_SV_DELTA as i64),
                ),
            )
        } else {
            (false, (0, 0))
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HsvMaterial {
    #[uniform(0)]
    pub h: f32,
    #[uniform(1)]
    pub delta: f32,
    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Handle<Image>,
    _alpha_mode: AlphaMode2d,
}

impl FromImage for HsvMaterial {
    fn from_image(image: Handle<Image>) -> Self {
        HsvMaterial {
            h: 360.,
            delta: HSV_DELTA,
            color_texture: image,
            _alpha_mode: AlphaMode2d::Blend,
        }
    }
}

impl Material2d for HsvMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hsv.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self._alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Hsv2DVizMaterial {
    #[uniform(0)]
    pub h: f32,
    #[uniform(1)]
    pub delta: f32,
    _alpha_mode: AlphaMode2d,
}

impl Default for Hsv2DVizMaterial {
    fn default() -> Self {
        Hsv2DVizMaterial {
            h: 360.,
            delta: HSV_DELTA,
            _alpha_mode: AlphaMode2d::Blend,
        }
    }
}

impl Material2d for Hsv2DVizMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hsv_2dviz.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self._alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Hsv3DVizMaterial {
    #[uniform(0)]
    pub h: f32,
    #[uniform(1)]
    pub delta: f32,
    #[uniform(2)]
    pub bottom: Vec3,
    _alpha_mode: AlphaMode,
}

impl Default for Hsv3DVizMaterial {
    fn default() -> Self {
        Hsv3DVizMaterial {
            h: 360.,
            delta: HSV_DELTA,
            bottom: COLOR_3D_VIZ_COORD - Vec3::new(0.5, 0.5, 0.5),
            _alpha_mode: AlphaMode::AlphaToCoverage,
        }
    }
}
impl Material for Hsv3DVizMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hsv_3dviz.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self._alpha_mode
    }
}
