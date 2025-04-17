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
    controls::{ColorParam, KbdCooldown},
    providers::{
        generic::Provider,
        okhsv::{Okhsv2DVizMaterial, Okhsv3DVizMaterial, OkhsvMaterial, OkhsvProvider},
    },
};

#[derive(Component)]
pub struct ImageCanvas;
#[derive(Component)]
pub struct Viz2DCanvas;
#[derive(Component)]
pub struct Viz3DMesh;

#[derive(Component)]
pub struct ImageLoader(pub Handle<Image>);

#[derive(Component)]
pub enum CamViewPort {
    ImageFilter,
    ColorDistribution,
    ColorTunnel,
}

pub fn setup_scene_pre(mut commands: Commands, asset_server: Res<AssetServer>, opts: Res<ProgOpt>) {
    // defer drawing of image
    let img_handle: Handle<Image> = asset_server.load(&opts.file);
    // associate the handle with an entity
    commands.spawn(ImageLoader(img_handle.clone()));

    // TODO: generate from progopt
    // create the global image filter shader
    let p = OkhsvProvider {
        filter: OkhsvMaterial::new(360., img_handle.clone()),
        viz2d_material: Okhsv2DVizMaterial::new(360.),
        viz3d_material: Okhsv3DVizMaterial::new(360.),
    };

    // create the controls, consisting of the keybind timeout timer and the current value of the
    // params
    commands.insert_resource(ColorParam {
        delta: p.delta(),
        cooldown: KbdCooldown::default(),
    });

    commands.insert_resource(p);
}

const IMG_BASE_SIZE: f32 = 1080. * 4. / 5.;
const COLOR_2D_VIZ_COORD: Vec3 = Vec3::new(2000., 0., 0.);
const COLOR_2D_VIZ_SIZE: f32 = 300.;
pub const COLOR_3D_VIZ_COORD: Vec3 = Vec3::new(-2000., 0., 0.);

pub fn setup_scene<A>(
    mut commands: Commands,
    provider: Res<A>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &ImageLoader)>,
    _opts: ResMut<ProgOpt>,
    filter: Res<OkhsvProvider>,
    image_filters: ResMut<Assets<OkhsvMaterial>>,
    mut viz2d_materials: ResMut<Assets<Okhsv2DVizMaterial>>,
    mut viz3d_materials: ResMut<Assets<Okhsv3DVizMaterial>>,
    color_materials: ResMut<Assets<ColorMaterial>>,
) where
    A: Provider + Resource,
{
    if query.is_empty() {
        // image already loaded
        return;
    }

    let (entity, loader) = query.single();
    let load_state = asset_server.get_load_state(&loader.0);

    if let (Some(LoadState::Loaded), Some(image)) = (load_state, images.get_mut(&loader.0)) {
        // delete marker entity
        commands.entity(entity).despawn();

        // display image
        spawn_image(image, &mut commands, &mut meshes, &filter, image_filters);

        // display 2d viz
        spawn_2dviz_square(&mut commands, &mut meshes, &mut viz2d_materials);

        // spawn rectangles that would generate the histogram shape
        // by covering extra parts
        spawn_histogram_covering(provider, image, &mut commands, &mut meshes, color_materials);

        commands.spawn((
            (
                // TODO: change to custom mesh
                Mesh3d(meshes.add(Cuboid::default())),
                MeshMaterial3d(viz3d_materials.add(filter.viz3d_material.clone())),
                Transform::from_translation(COLOR_3D_VIZ_COORD),
            ),
            Viz3DMesh,
        ));

        // camera
        commands.spawn((
            (
                Camera3d::default(),
                Camera {
                    order: 3,
                    ..default()
                },
                Transform::from_translation(COLOR_3D_VIZ_COORD + Vec3::new(-2., 2., -2.))
                    .looking_at(COLOR_3D_VIZ_COORD, Vec3::Y),
            ),
            CamViewPort::ColorTunnel,
        ));
    }
}

fn spawn_image(
    image: &mut Image,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    filter: &Res<OkhsvProvider>,
    mut image_filters: ResMut<Assets<OkhsvMaterial>>,
) {
    // don't downscale the image
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: AddressMode::ClampToEdge.into(),
        address_mode_v: AddressMode::ClampToEdge.into(),
        mag_filter: FilterMode::Linear.into(),
        min_filter: FilterMode::Linear.into(),
        mipmap_filter: FilterMode::Linear.into(),
        ..default()
    });

    let aspect_ratio =
        image.texture_descriptor.size.width as f32 / image.texture_descriptor.size.height as f32;

    // spawn a square the has the right dimensions and use the image as material
    commands.spawn((
        (
            Mesh2d(meshes.add(Rectangle::new(IMG_BASE_SIZE * aspect_ratio, IMG_BASE_SIZE))),
            MeshMaterial2d(image_filters.add(filter.filter.clone())),
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
}

fn spawn_2dviz_square(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    viz2d_materials: &mut ResMut<Assets<Okhsv2DVizMaterial>>,
) {
    // Spawn corresponding 2d color distribution
    commands.spawn((
        (
            Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
            MeshMaterial2d(viz2d_materials.add(Okhsv2DVizMaterial::new(360.))),
            Transform::from_translation(COLOR_2D_VIZ_COORD)
                .with_scale(Vec3::splat(COLOR_2D_VIZ_SIZE)),
        ),
        Viz2DCanvas,
    ));

    // Spawn camera to show the 2d color distribution
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
}

fn spawn_histogram_covering<A>(
    provider: Res<A>,
    image: &Image,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) where
    A: Provider + Resource,
{
    let mut data = provider.histogram_data(image);
    // normalize
    let max = data
        .iter()
        .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
        .unwrap()
        .1;
    data.iter_mut().for_each(|(_, y)| *y /= max);

    let mut x = provider.min();
    let mut iter = data.iter().peekable();
    while x < provider.max() {
        // data is in ascending order, so just iter through
        let ratio = match iter.peek() {
            Some((y, z)) => {
                if *y == x {
                    iter.next();
                    *z
                } else {
                    0.
                }
            }
            None => 0.,
        };
        commands.spawn((
                    Mesh2d(meshes.add(Mesh::from(Rectangle::new(
                        provider.delta() / provider.max(),
                        1. - ratio,
                    )))),
                    MeshMaterial2d(color_materials.add(Color::srgb_u8(42, 44, 46))),
                    Transform::from_translation(
                        COLOR_2D_VIZ_COORD
                            // render on top of distribution
                            + Vec3::Z
                            // move bar to corresponding color pos
                            // HACK: provider.delta() / 2. seems to fix off by 1 error
                            + Vec3::X * ((x + provider.delta() / 2.) / provider.max() - 0.5) * COLOR_2D_VIZ_SIZE
                            // align top with 2d viz top
                            + Vec3::Y * (ratio * COLOR_2D_VIZ_SIZE / 2.),
                    )
                    .with_scale(Vec3::splat(COLOR_2D_VIZ_SIZE)),
                ));
        x += provider.delta();
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
