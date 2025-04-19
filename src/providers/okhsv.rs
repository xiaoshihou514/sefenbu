use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
    utils::HashMap,
};
use palette::{FromColor, Okhsv, Srgb};

use crate::COLOR_3D_VIZ_COORD;

use super::generic::{CSpaceProvider, FromImage, Provider};

// global state
#[derive(Resource)]
pub struct OkhsvProvider {
    pub filter: OkhsvMaterial,
    pub viz2d_material: Okhsv2DVizMaterial,
    pub viz3d_material: Okhsv3DVizMaterial,
    cache: HashMap<(u32, u32), Okhsv>,
}

impl CSpaceProvider for OkhsvProvider {
    type FilterMaterial = OkhsvMaterial;
    type Viz2dMaterial = Okhsv2DVizMaterial;
    type Viz3dMaterial = Okhsv3DVizMaterial;

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

impl FromImage for OkhsvProvider {
    fn from_image(img: Handle<Image>) -> Self {
        OkhsvProvider {
            filter: OkhsvMaterial::from_image(img),
            viz2d_material: Okhsv2DVizMaterial::default(),
            viz3d_material: Okhsv3DVizMaterial::default(),
            cache: HashMap::new(),
        }
    }
}

fn to_okhsv(c: Color) -> Okhsv {
    let s: Srgba = c.into();
    Okhsv::from_color(Srgb::new(s.red, s.green, s.blue))
}

const OKHSV_DELTA: f32 = 2.;
const OKHSV_SV_MAX: f32 = 100.;
const OKHSV_SV_DELTA: f32 = 2.;
impl Provider for OkhsvProvider {
    const MAX: f32 = 360.;
    const MIN: f32 = 0.;
    const DELTA: f32 = OKHSV_DELTA;

    const X_MAX: f32 = OKHSV_SV_MAX;
    const X_DELTA: f32 = OKHSV_SV_DELTA;
    const Z_MAX: f32 = OKHSV_SV_MAX;
    const Z_DELTA: f32 = OKHSV_SV_DELTA;

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
        let okhsv: Okhsv = to_okhsv(c);
        (okhsv.hue.into_positive_degrees() / Self::DELTA) as i64 * (Self::DELTA as i64)
    }

    fn inspect(&mut self, img: &Image, i: u32, j: u32) -> (bool, (i64, i64)) {
        // cache colors
        let okhsv = match self.cache.get(&(i, j)) {
            Some(c) => *c,
            None => {
                let c = to_okhsv(img.get_color_at(i, j).unwrap());
                self.cache.insert((i, j), c);
                c
            }
        };
        if (okhsv.hue.into_positive_degrees() - self.current()).abs() > Self::DELTA {
            // within range
            (
                true,
                (
                    (Into::<f32>::into(okhsv.saturation) * OKHSV_SV_MAX / OKHSV_SV_DELTA) as i64
                        * (OKHSV_SV_DELTA as i64),
                    (Into::<f32>::into(okhsv.value) * OKHSV_SV_MAX / OKHSV_SV_DELTA) as i64
                        * (OKHSV_SV_DELTA as i64),
                ),
            )
        } else {
            (false, (0, 0))
        }
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

impl FromImage for OkhsvMaterial {
    fn from_image(image: Handle<Image>) -> Self {
        OkhsvMaterial {
            h: 360.,
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

impl Default for Okhsv2DVizMaterial {
    fn default() -> Self {
        Okhsv2DVizMaterial {
            h: 360.,
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

impl Default for Okhsv3DVizMaterial {
    fn default() -> Self {
        Okhsv3DVizMaterial {
            h: 360.,
            delta: OKHSV_DELTA,
            bottom: COLOR_3D_VIZ_COORD - Vec3::new(0.5, 0.5, 0.5),
            _alpha_mode: AlphaMode::AlphaToCoverage,
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
