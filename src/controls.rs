use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{
    providers::okhsv::OkhsvMaterial,
    scene::{ImageCanvas, ImageFilter, ImageLoader},
};

#[derive(Component)]
pub struct MeshController(pub Vec2);

#[derive(Resource)]
pub struct ColorParam {
    pub max: f32,
    pub min: f32,
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
    mut filter: ResMut<ImageFilter>,
    mut canvas: Query<(&mut MeshMaterial3d<OkhsvMaterial>, &ImageCanvas)>,
    mut materials: ResMut<Assets<OkhsvMaterial>>,
    loader: Query<(Entity, &ImageLoader)>,
) {
    if !param.cooldown.finished(time) || !loader.is_empty() {
        return;
    }

    if keyboard.pressed(KeyCode::KeyJ) {
        info!("decreasing param");
        filter.0.h -= if keyboard.pressed(KeyCode::ShiftLeft) {
            param.delta * 10.0
        } else {
            param.delta
        };
        filter.0.h = filter.0.h.max(param.min);
        param.cooldown.reset();
    } else if keyboard.pressed(KeyCode::KeyK) {
        info!("increasing param");
        filter.0.h += if keyboard.pressed(KeyCode::ShiftLeft) {
            param.delta * 10.0
        } else {
            param.delta
        };
        filter.0.h = filter.0.h.min(param.max);
        param.cooldown.reset();
    }

    if filter.is_changed() {
        info!("updated param: {}", filter.0.h);
        info!("going to update shader");
        canvas.single_mut().0 .0 = materials.add(filter.0.clone());
    }
}
