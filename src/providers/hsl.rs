use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
    utils::HashMap,
};

use crate::COLOR_3D_VIZ_COORD;

use super::generic::{CSpaceProvider, FromImage, Provider};

// global state
#[derive(Resource)]
pub struct HslProvider {
    pub filter: HslMaterial,
    pub viz2d_material: Hsl2DVizMaterial,
    pub viz3d_material: Hsl3DVizMaterial,
    cache: HashMap<(u32, u32), Hsla>,
}

impl CSpaceProvider for HslProvider {
    type FilterMaterial = HslMaterial;
    type Viz2dMaterial = Hsl2DVizMaterial;
    type Viz3dMaterial = Hsl3DVizMaterial;

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

impl FromImage for HslProvider {
    fn from_image(img: Handle<Image>) -> Self {
        HslProvider {
            filter: HslMaterial::from_image(img),
            viz2d_material: Hsl2DVizMaterial::default(),
            viz3d_material: Hsl3DVizMaterial::default(),
            cache: HashMap::new(),
        }
    }
}

const HSL_DELTA: f32 = 1.;
impl Provider for HslProvider {
    const MAX: f32 = 100.;
    const MIN: f32 = 0.;
    const DELTA: f32 = HSL_DELTA;

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
        let hsl: Hsla = c.into();
        (hsl.lightness * Self::MAX / Self::DELTA) as i64 * (Self::DELTA as i64)
    }

    fn inspect(&mut self, img: &Image, i: u32, j: u32) -> (bool, (i64, i64)) {
        // cache colors
        let hsl = match self.cache.get(&(i, j)) {
            Some(c) => *c,
            None => {
                let c: Hsla = img.get_color_at(i, j).unwrap().into();
                self.cache.insert((i, j), c);
                c
            }
        };
        if (hsl.lightness * Self::MAX - self.current()).abs() > Self::DELTA {
            // within range
            (
                true,
                (
                    (hsl.hue / Self::X_DELTA) as i64 * (Self::X_DELTA as i64),
                    (hsl.saturation * Self::Z_MAX / Self::Z_DELTA) as i64 * (Self::Z_DELTA as i64),
                ),
            )
        } else {
            (false, (0, 0))
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HslMaterial {
    #[uniform(0)]
    pub l: f32,
    #[uniform(1)]
    pub delta: f32,
    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Handle<Image>,
    _alpha_mode: AlphaMode2d,
}

impl FromImage for HslMaterial {
    fn from_image(image: Handle<Image>) -> Self {
        HslMaterial {
            l: 100.,
            delta: HSL_DELTA,
            color_texture: image,
            _alpha_mode: AlphaMode2d::Blend,
        }
    }
}

impl Material2d for HslMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hsl.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self._alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Hsl2DVizMaterial {
    #[uniform(0)]
    pub l: f32,
    #[uniform(1)]
    pub delta: f32,
    _alpha_mode: AlphaMode2d,
}

impl Default for Hsl2DVizMaterial {
    fn default() -> Self {
        Hsl2DVizMaterial {
            l: 100.,
            delta: HSL_DELTA,
            _alpha_mode: AlphaMode2d::Blend,
        }
    }
}

impl Material2d for Hsl2DVizMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hsl_2dviz.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self._alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Hsl3DVizMaterial {
    #[uniform(0)]
    pub l: f32,
    #[uniform(1)]
    pub delta: f32,
    #[uniform(2)]
    pub bottom: Vec3,
    _alpha_mode: AlphaMode,
}

impl Default for Hsl3DVizMaterial {
    fn default() -> Self {
        Hsl3DVizMaterial {
            l: 100.,
            delta: HSL_DELTA,
            bottom: COLOR_3D_VIZ_COORD - Vec3::new(0.5, 0.5, 0.5),
            _alpha_mode: AlphaMode::AlphaToCoverage,
        }
    }
}
impl Material for Hsl3DVizMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hsl_3dviz.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self._alpha_mode
    }
}
