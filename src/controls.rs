use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{
    providers::{generic::Provider, okhsv::{Okhsv2DVizMaterial, OkhsvMaterial, OkhsvProvider}},
    scene::{ImageCanvas, ImageLoader},
    Viz2DCanvas,
};

#[derive(Component)]
pub struct MeshController(pub Vec2);

#[derive(Resource)]
pub struct ColorParam {
    pub delta: f32,
    pub cooldown: KbdCooldown,
}

pub struct KbdCooldown(pub Timer);
const KBD_COOLDOWN_SECS: f32 = 0.15;
impl Default for KbdCooldown {
    fn default() -> Self {
        return KbdCooldown(Timer::from_seconds(KBD_COOLDOWN_SECS, TimerMode::Once));
    }
}
impl KbdCooldown {
    fn finished(self: &mut Self, time: Res<Time>) -> bool {
        return self.0.tick(time.delta()).finished();
    }

    fn reset(self: &mut Self) {
        self.0.reset();
    }
}

// https://bevyengine.org/examples/camera/first-person-view-model/
pub fn control_blob(
    // initialized when setting up scene
    mut blob: Query<(&mut Transform, &MeshController)>,
    mouse: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let Ok((mut transform, sensitivity)) = blob.get_single_mut() else {
        return;
    };

    let delta = accumulated_mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    if mouse.pressed(MouseButton::Left) {
        // magic code that converts 2d rotation to 3d roration
        let delta_yaw = delta.x * sensitivity.0.x;
        let (yaw, _, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        // only horizontal rotation
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw + delta_yaw, 0.0, roll);
    }

    if mouse.pressed(MouseButton::Middle) {
        transform.translation +=
            Vec3::new(delta.x * sensitivity.0.x, -delta.y * sensitivity.0.y, 0.0)
    }
}

pub fn change_param(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut param: ResMut<ColorParam>,
    time: Res<Time>,
    mut p: ResMut<OkhsvProvider>,
    mut img_canvas: Query<(&mut MeshMaterial2d<OkhsvMaterial>, &ImageCanvas)>,
    mut viz2d_canvas: Query<(&mut MeshMaterial2d<Okhsv2DVizMaterial>, &Viz2DCanvas)>,
    mut img_filters: ResMut<Assets<OkhsvMaterial>>,
    mut viz2d_materials: ResMut<Assets<Okhsv2DVizMaterial>>,
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
        viz2d_canvas.single_mut().0 .0 = viz2d_materials.add(p.viz2d_material.clone())
    }
}
