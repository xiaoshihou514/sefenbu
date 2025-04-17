use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{
    providers::{
        generic::Provider,
        okhsv::{Okhsv2DVizMaterial, Okhsv3DVizMaterial, OkhsvMaterial, OkhsvProvider},
    },
    scene::{ImageCanvas, ImageLoader},
    Viz2DCanvas, Viz3DMesh, COLOR_3D_VIZ_COORD,
};

#[derive(Component)]
pub struct MeshControlConf {
    pub v_pitch: f32,
    pub v_yaw: f32,
    pub orbit_distance: f32,
    pub pitch_max: f32,
    pub pitch_min: f32,
}
impl Default for MeshControlConf {
    fn default() -> Self {
        MeshControlConf {
            v_pitch: 0.01,
            v_yaw: 0.01,
            // 2 * sqrt(3)
            orbit_distance: 3.4641016151377544,
            pitch_max: 0.3,
            pitch_min: -0.8,
        }
    }
}

#[derive(Resource)]
pub struct ColorParam {
    pub delta: f32,
    pub cooldown: KbdCooldown,
}

pub struct KbdCooldown(pub Timer);
const KBD_COOLDOWN_SECS: f32 = 0.15;
impl Default for KbdCooldown {
    fn default() -> Self {
        KbdCooldown(Timer::from_seconds(KBD_COOLDOWN_SECS, TimerMode::Once))
    }
}
impl KbdCooldown {
    fn finished(&mut self, time: Res<Time>) -> bool {
        self.0.tick(time.delta()).finished()
    }

    fn reset(&mut self) {
        self.0.reset();
    }
}

// https://bevyengine.org/examples/camera/camera-orbit/
pub fn control_blob(
    // initialized when setting up scene
    mut blob: Query<(&mut Transform, &MeshControlConf)>,
    mouse: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let Ok((mut transform, conf)) = blob.get_single_mut() else {
        return;
    };

    let delta = accumulated_mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    if mouse.pressed(MouseButton::Left) {
        // 3d polar coordinate
        let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        yaw -= delta.x * conf.v_yaw;
        pitch = (pitch - delta.y * conf.v_pitch).clamp(conf.pitch_min, conf.pitch_max);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

        // Adjust the translation to maintain the correct orientation toward the orbit target.
        transform.translation = Transform::from_translation(
            COLOR_3D_VIZ_COORD - transform.forward() * conf.orbit_distance,
        )
        .translation;
    }
}

pub fn change_param(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut param: ResMut<ColorParam>,
    time: Res<Time>,
    mut p: ResMut<OkhsvProvider>,
    mut img_canvas: Query<(&mut MeshMaterial2d<OkhsvMaterial>, &ImageCanvas)>,
    mut viz2d_canvas: Query<(&mut MeshMaterial2d<Okhsv2DVizMaterial>, &Viz2DCanvas)>,
    mut viz3d_mesh: Query<(&mut MeshMaterial3d<Okhsv3DVizMaterial>, &Viz3DMesh)>,
    mut img_filters: ResMut<Assets<OkhsvMaterial>>,
    mut viz2d_materials: ResMut<Assets<Okhsv2DVizMaterial>>,
    mut viz3d_materials: ResMut<Assets<Okhsv3DVizMaterial>>,
    loader: Query<(Entity, &ImageLoader)>,
) {
    if !param.cooldown.finished(time) || !loader.is_empty() {
        return;
    }

    let change = if keyboard.pressed(KeyCode::ShiftLeft) {
        param.delta * 10.
    } else {
        param.delta
    };

    if keyboard.pressed(KeyCode::KeyJ) {
        // decrement param
        p.decr(change);
        param.cooldown.reset();
    } else if keyboard.pressed(KeyCode::KeyK) {
        // increment param
        p.incr(change);
        param.cooldown.reset();
    }

    // apply change, original material substituted
    if p.is_changed() {
        img_canvas.single_mut().0 .0 = img_filters.add(p.filter.clone());
        viz2d_canvas.single_mut().0 .0 = viz2d_materials.add(p.viz2d_material.clone());
        viz3d_mesh.single_mut().0 .0 = viz3d_materials.add(p.viz3d_material.clone());
    }
}
