use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{
    providers::generic::CSpaceProvider,
    scene::{ImageCanvas, ImageLoader},
    Background, Viz2DCanvas, Viz3DMesh, COLOR_3D_VIZ_COORD, IMG_VIEW_W_RATIO, VIZ3D_H_RATIO,
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
            orbit_distance: 3.464_101_6,
            pitch_max: 0.0,
            pitch_min: -0.9,
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
    window: Single<&Window>,
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

    let size = window.size();
    let pos = window.cursor_position().unwrap();
    let x_threshold = size.x * IMG_VIEW_W_RATIO;
    let y_threshold = size.y * VIZ3D_H_RATIO;

    // check in bound
    if mouse.pressed(MouseButton::Left) && pos.x > x_threshold && pos.y < y_threshold {
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

pub fn change_param<A: CSpaceProvider>(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    mut param: ResMut<ColorParam>,
    time: Res<Time>,
    mut p: ResMut<A>,
    img: Option<Res<Background>>,
    loader: Query<(Entity, &ImageLoader)>,
    // queries for entities that needs to be updated
    mut img_canvas: Query<(&mut MeshMaterial2d<A::FilterMaterial>, &ImageCanvas)>,
    mut viz2d_canvas: Query<(&mut MeshMaterial2d<A::Viz2dMaterial>, &Viz2DCanvas)>,
    mut viz3d_mesh: Query<(
        (&mut MeshMaterial3d<A::Viz3dMaterial>, &mut Mesh3d),
        &Viz3DMesh,
    )>,
    mut text: Query<&mut Text2d>,
    // entity managers
    mut img_filters: ResMut<Assets<A::FilterMaterial>>,
    mut viz2d_materials: ResMut<Assets<A::Viz2dMaterial>>,
    mut viz3d_materials: ResMut<Assets<A::Viz3dMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !param.cooldown.finished(time) || !loader.is_empty() || img.is_none() {
        return;
    }

    // response to clicks on viz2d
    if mouse.just_pressed(MouseButton::Left) {
        let size = window.size();
        let pos = window.cursor_position().unwrap();
        let x_threshold = size.x * IMG_VIEW_W_RATIO;
        let y_threshold = size.y * VIZ3D_H_RATIO;

        if pos.x > x_threshold && pos.y > y_threshold {
            p.set((pos.x - x_threshold) / (size.x - x_threshold));
        }
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

    // apply change, original item substituted
    if p.is_changed() {
        // update image filter
        img_canvas.single_mut().0 .0 = img_filters.add(p.get_filter());
        // update viz2d current color indicator
        viz2d_canvas.single_mut().0 .0 = viz2d_materials.add(p.get_viz2d_material());
        // update viz3d material
        viz3d_mesh.single_mut().0 .0 .0 = viz3d_materials.add(p.get_viz3d_material());
        // update viz3d mesh
        viz3d_mesh.single_mut().0 .1 .0 = meshes.add(p.create_mesh(&img.unwrap().0));
        // update param banner
        text.single_mut().0 = format!("{}", p.current());
    }
}
