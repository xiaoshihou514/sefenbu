use bevy::{
    asset::LoadState,
    image::{ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::{
        camera::Viewport,
        render_resource::{AddressMode, FilterMode},
    },
};

use crate::{
    cli::ProgOpt,
    controls::{ColorParam, KbdCooldown, MeshController},
    providers::okhsv::OkhsvMaterial,
};

#[derive(Component)]
pub struct ImageLoader(pub Handle<Image>);

#[derive(Resource)]
pub struct ImageFilter(pub OkhsvMaterial);

#[derive(Component)]
pub struct ImageCanvas;

#[derive(Component)]
pub enum CamViewPort {
    ImageFilter,
    ColorDistribution,
    ColorTunnel,
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    opts: Res<ProgOpt>,
) {
    // defer drawing of image
    let img_handle: Handle<Image> = asset_server.load(&opts.file);
    // associate the handle with an entity
    commands.spawn(ImageLoader(img_handle.clone()));

    // create the global image filter shader
    commands.insert_resource(ImageFilter(OkhsvMaterial::new(360., img_handle)));

    // create the controls, consisting of the keybind timeout timer and the current value of the
    // params
    commands.insert_resource(ColorParam {
        max: 360.,
        min: 0.,
        delta: 1.,
        cooldown: KbdCooldown::default(),
    });

    // cube
    commands.spawn((
        (
            Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0., 3., 0.),
        ),
        MeshController(Vec2::new(0.01, 0.01)),
    ));
}

const IMG_BASE_SIZE: f32 = 1080. * 4. / 5.;
const COLOR_2D_VIZ_COORD: Vec3 = Vec3::new(2000., 0., 0.);
const COLOR_2D_VIZ_SIZE: Vec3 = Vec3::splat(256.);

pub fn draw_image_await_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &ImageLoader)>,
    _opts: ResMut<ProgOpt>,
    filter: Res<ImageFilter>,
    mut materials: ResMut<Assets<OkhsvMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    if query.is_empty() {
        // image already loaded
        return;
    }

    let (entity, loader) = query.single();
    let load_state = asset_server.get_load_state(&loader.0);

    match (load_state, images.get_mut(&loader.0)) {
        (Some(LoadState::Loaded), Some(image)) => {
            // don't downscale the image
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: AddressMode::ClampToEdge.into(),
                address_mode_v: AddressMode::ClampToEdge.into(),
                mag_filter: FilterMode::Linear.into(),
                min_filter: FilterMode::Linear.into(),
                mipmap_filter: FilterMode::Linear.into(),
                ..default()
            });

            let aspect_ratio = image.texture_descriptor.size.width as f32
                / image.texture_descriptor.size.height as f32;

            // spawn a cube the has the right dimensions and use the image as material
            commands.spawn((
                (
                    Mesh2d(meshes.add(Rectangle::new(IMG_BASE_SIZE * aspect_ratio, IMG_BASE_SIZE))),
                    MeshMaterial2d(materials.add(filter.0.clone())),
                ),
                ImageCanvas,
            ));

            // camera
            commands.spawn((
                (
                    Camera2d,
                    Camera {
                        order: 1,
                        ..default()
                    },
                ),
                CamViewPort::ImageFilter,
            ));

            commands.entity(entity).despawn();

            // TESTING
            let mut mesh = Mesh::from(Rectangle::default());
            // Build vertex colors for the quad. One entry per vertex (the corners of the quad)
            let vertex_colors: Vec<[f32; 4]> = vec![
                LinearRgba::RED.to_f32_array(),
                LinearRgba::GREEN.to_f32_array(),
                LinearRgba::BLUE.to_f32_array(),
                LinearRgba::WHITE.to_f32_array(),
            ];
            // Insert the vertex colors as an attribute
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

            let mesh_handle = meshes.add(mesh);

            // Spawn camera
            commands.spawn((
                (
                    Camera2d,
                    Transform::from_translation(COLOR_2D_VIZ_COORD),
                    Camera {
                        order: 2,
                        ..default()
                    },
                ),
                CamViewPort::ColorDistribution,
            ));

            // Spawn the quad with vertex colors
            commands.spawn((
                Mesh2d(mesh_handle.clone()),
                MeshMaterial2d(color_materials.add(ColorMaterial::default())),
                Transform::from_translation(COLOR_2D_VIZ_COORD).with_scale(COLOR_2D_VIZ_SIZE),
            ));
        }
        _ => (),
    }
}

pub fn set_viewports(windows: Query<&Window>, mut query: Query<(&CamViewPort, &mut Camera)>) {
    let window = windows.get_single().unwrap();
    let size = window.physical_size();
    let img_filter_width = size.x * 4 / 5;
    let viz_width = size.x / 5;

    for (camera_position, mut camera) in &mut query {
        let (physical_position, physical_size) = match camera_position {
            CamViewPort::ImageFilter => (UVec2::new(0, 0), UVec2::new(img_filter_width, size.y)),
            CamViewPort::ColorDistribution => (
                UVec2::new(img_filter_width, size.y / 2),
                UVec2::new(viz_width, size.y / 2),
            ),
            CamViewPort::ColorTunnel => (
                UVec2::new(img_filter_width, 0),
                UVec2::new(viz_width, size.y / 2),
            ),
        };

        camera.viewport = Some(Viewport {
            physical_position,
            physical_size,
            ..default()
        });
    }
}
