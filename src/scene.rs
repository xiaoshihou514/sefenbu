use bevy::{
    asset::LoadState,
    image::{ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::render_resource::{AddressMode, FilterMode},
};

use crate::{
    cli::ProgOpt,
    controls::{CameraController, MouseSensitivity},
};

#[derive(Component)]
pub struct ImageLoader(pub Handle<Image>);

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    opts: Res<ProgOpt>,
) {
    // defer drawing of image
    let img_handle: Handle<Image> = asset_server.load(&opts.file);
    commands.spawn(ImageLoader(img_handle));

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // cube
    commands.spawn((
        (
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ),
        CameraController,
        MouseSensitivity(Vec2::new(0.003, 0.003)),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

pub fn draw_image(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &ImageLoader)>,
) {
    for (entity, loader) in query.iter() {
        let load_state = asset_server.get_load_state(&loader.0);

        if let Some(LoadState::Loaded) = load_state {
            if let Some(image) = images.get_mut(&loader.0) {
                image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: AddressMode::ClampToEdge.into(),
                    address_mode_v: AddressMode::ClampToEdge.into(),
                    mag_filter: FilterMode::Linear.into(),
                    min_filter: FilterMode::Linear.into(),
                    mipmap_filter: FilterMode::Linear.into(),
                    ..default()
                });

                let (width, height) = (
                    image.texture_descriptor.size.width as f32,
                    image.texture_descriptor.size.height as f32,
                );
                let aspect_ratio = width / height;

                let material = materials.add(StandardMaterial {
                    base_color_texture: Some(loader.0.clone()),
                    unlit: true,
                    ..default()
                });

                commands.spawn((
                    Mesh3d(meshes.add(Rectangle::new(10.0 * aspect_ratio, 10.0))),
                    MeshMaterial3d(material),
                    Transform::from_xyz(0.0, 5.0, -5.0),
                ));

                // camera
                commands.spawn((
                    Camera3d::default(),
                    Transform::from_xyz(0.0, 5.0, 10.0)
                        .looking_at(Vec3::new(0.0, 5.0, -5.0), Vec3::Y),
                ));

                commands.entity(entity).despawn();
            }
        }
    }
}
