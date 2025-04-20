use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
    utils::HashMap,
};
use palette::{FromColor, Okhsl, Srgb};

use crate::COLOR_3D_VIZ_COORD;

use super::generic::{CSpaceProvider, FromImage, Provider};

// global state
#[derive(Resource)]
pub struct OkhslProvider {
    pub filter: OkhslMaterial,
    pub viz2d_material: Okhsl2DVizMaterial,
    pub viz3d_material: Okhsl3DVizMaterial,
    cache: HashMap<(u32, u32), Okhsl>,
}

impl CSpaceProvider for OkhslProvider {
    type FilterMaterial = OkhslMaterial;
    type Viz2dMaterial = Okhsl2DVizMaterial;
    type Viz3dMaterial = Okhsl3DVizMaterial;

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

impl FromImage for OkhslProvider {
    fn from_image(img: Handle<Image>) -> Self {
        OkhslProvider {
            filter: OkhslMaterial::from_image(img),
            viz2d_material: Okhsl2DVizMaterial::default(),
            viz3d_material: Okhsl3DVizMaterial::default(),
            cache: HashMap::new(),
        }
    }
}

fn to_okhsl(c: Color) -> Okhsl {
    let s: Srgba = c.into();
    Okhsl::from_color(Srgb::new(s.red, s.green, s.blue))
}

const OKHSL_DELTA: f32 = 1.;
impl Provider for OkhslProvider {
    const MAX: f32 = 100.;
    const MIN: f32 = 0.;
    const DELTA: f32 = OKHSL_DELTA;

    // X hue, Z saturation
    const X_MAX: f32 = 360.;
    const X_DELTA: f32 = 7.2;
    const Z_MAX: f32 = 100.;
    const Z_DELTA: f32 = 2.;

    #[rustfmt::skip]
    fn current(&self) -> f32 { self.filter.l }

    fn decr(&mut self, change: f32) {
        // overflow protection
        self.filter.l = Self::MIN.max(self.filter.l - change);
        self.viz2d_material.l = Self::MIN.max(self.viz2d_material.l - change);
        self.viz3d_material.l = Self::MIN.max(self.viz3d_material.l - change);
    }

    fn incr(&mut self, change: f32) {
        // overflow protection
        self.filter.l = Self::MAX.min(self.filter.l + change);
        self.viz2d_material.l = Self::MAX.min(self.viz2d_material.l + change);
        self.viz3d_material.l = Self::MAX.min(self.viz3d_material.l + change);
    }

    fn set(&mut self, new: f32) {
        let new_adjusted = (new * Self::MAX / Self::DELTA) as i64 as f32 * Self::DELTA;
        self.filter.l = new_adjusted;
        self.viz2d_material.l = new_adjusted;
        self.viz3d_material.l = new_adjusted;
    }

    fn convert(&self, c: Color) -> i64 {
        let okhsl: Okhsl = to_okhsl(c);
        (okhsl.lightness * Self::MAX / Self::DELTA) as i64 * (Self::DELTA as i64)
    }

    fn inspect(&mut self, img: &Image, i: u32, j: u32) -> (bool, (i64, i64)) {
        // cache colors
        let okhsl = match self.cache.get(&(i, j)) {
            Some(c) => *c,
            None => {
                let c = to_okhsl(img.get_color_at(i, j).unwrap());
                self.cache.insert((i, j), c);
                c
            }
        };
        if (okhsl.lightness * Self::MAX - self.current()).abs() > Self::DELTA {
            // within range
            (
                true,
                (
                    (okhsl.hue.into_positive_degrees() / Self::X_DELTA) as i64
                        * (Self::X_DELTA as i64),
                    (Into::<f32>::into(okhsl.saturation) * Self::Z_MAX / Self::Z_DELTA) as i64
                        * (Self::Z_DELTA as i64),
                ),
            )
        } else {
            (false, (0, 0))
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OkhslMaterial {
    #[uniform(0)]
    pub l: f32,
    #[uniform(1)]
    pub delta: f32,
    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Handle<Image>,
    _alpha_mode: AlphaMode2d,
}

impl FromImage for OkhslMaterial {
    fn from_image(image: Handle<Image>) -> Self {
        OkhslMaterial {
            l: 100.,
            delta: OKHSL_DELTA,
            color_texture: image,
            _alpha_mode: AlphaMode2d::Blend,
        }
    }
}

impl Material2d for OkhslMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/okhsl.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self._alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Okhsl2DVizMaterial {
    #[uniform(0)]
    pub l: f32,
    #[uniform(1)]
    pub delta: f32,
    _alpha_mode: AlphaMode2d,
}

impl Default for Okhsl2DVizMaterial {
    fn default() -> Self {
        Okhsl2DVizMaterial {
            l: 100.,
            delta: OKHSL_DELTA,
            _alpha_mode: AlphaMode2d::Blend,
        }
    }
}

impl Material2d for Okhsl2DVizMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/okhsl_2dviz.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self._alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Okhsl3DVizMaterial {
    #[uniform(0)]
    pub l: f32,
    #[uniform(1)]
    pub delta: f32,
    #[uniform(2)]
    pub bottom: Vec3,
    _alpha_mode: AlphaMode,
}

impl Default for Okhsl3DVizMaterial {
    fn default() -> Self {
        Okhsl3DVizMaterial {
            l: 100.,
            delta: OKHSL_DELTA,
            bottom: COLOR_3D_VIZ_COORD - Vec3::new(0.5, 0.5, 0.5),
            _alpha_mode: AlphaMode::AlphaToCoverage,
        }
    }
}
impl Material for Okhsl3DVizMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/okhsl_3dviz.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self._alpha_mode
    }
}
