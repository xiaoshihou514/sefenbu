use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::providers::okhsv::OkhsvMaterial;

#[derive(Component)]
pub struct MouseSensitivity(pub Vec2);

#[derive(Component)]
pub struct ColorParam {
    pub delta: f32,
    pub cooldown: KbdCooldown,
}

pub struct KbdCooldown(pub Timer);
const KBD_COOLDOWN_SECS: f32 = 0.1;
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
    mut blob: Query<(&mut Transform, &MouseSensitivity)>,
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
    mut material_handler: Query<(&MeshMaterial3d<OkhsvMaterial>, &mut ColorParam)>,
    time: Res<Time>,
    mut materials: ResMut<Assets<OkhsvMaterial>>,
) {
    let Ok((mesh_material, mut param)) = material_handler.get_single_mut() else {
        return;
    };
    if !param.cooldown.finished(time) {
        return;
    }

    if keyboard.pressed(KeyCode::KeyJ) {
        println!("decreasing param");
        if let Some(okhsv_material) = materials.remove(&mesh_material.0) {
            materials.add(OkhsvMaterial::new(
                okhsv_material.h - param.delta,
                okhsv_material.color_texture,
            ));
            println!("updated param: {}", okhsv_material.h - param.delta);
        }
        param.cooldown.reset();
    } else if keyboard.pressed(KeyCode::KeyK) {
        println!("increasing param");
        if let Some(okhsv_material) = materials.remove(&mesh_material.0) {
            materials.add(OkhsvMaterial::new(
                okhsv_material.h - param.delta,
                okhsv_material.color_texture,
            ));
            println!("updated param: {}", okhsv_material.h - param.delta);
        }
        param.cooldown.reset();
    }
}
