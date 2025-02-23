use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

#[derive(Component)]
pub struct CameraController;

#[derive(Component)]
pub struct MouseSensitivity(pub Vec2);

// https://bevyengine.org/examples/camera/first-person-view-model/
pub fn control_blob(
    mut blob: Query<(&mut Transform, &MouseSensitivity), With<CameraController>>,
    buttons: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let Ok((mut transform, sensitivity)) = blob.get_single_mut() else {
        return;
    };

    let delta = accumulated_mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    if buttons.pressed(MouseButton::Left) {
        // magic code that converts 2d rotation to 3d roration
        let delta_yaw = delta.x * sensitivity.0.x;
        let (yaw, _, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        // only horizontal rotation
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw + delta_yaw, 0.0, roll);
    }

    if buttons.pressed(MouseButton::Middle) {
        transform.translation +=
            Vec3::new(delta.x * sensitivity.0.x, -delta.y * sensitivity.0.y, 0.0)
    }
}
