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
    cli::Cli,
    controls::{ColorParam, KbdCooldown},
    providers::generic::CSpaceProvider,
    MeshControlConf,
};

#[derive(Component)]
pub struct ImageCanvas;
#[derive(Component)]
pub struct Viz2DCanvas;
#[derive(Component)]
pub struct Viz3DMesh;

#[derive(Component)]
pub struct ImageLoader(pub Handle<Image>);
#[derive(Resource)]
pub struct Background(pub Image);

#[derive(Component)]
pub enum CamViewPort {
    ImageFilter,
    Viz2d,
    Viz3d,
}

pub fn setup_scene_pre<A: CSpaceProvider>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    opts: Res<Cli>,
) {
    // defer drawing of image
    let img_handle: Handle<Image> = asset_server.load(&opts.file);
    // associate the handle with an entity
    commands.spawn(ImageLoader(img_handle.clone()));

    // create the global image filter shader
    let p = A::from_image(img_handle.clone());

    // create the controls, consisting of the keybind timeout timer and the current value of the
    // params
    commands.insert_resource(ColorParam {
        delta: A::DELTA,
        cooldown: KbdCooldown::default(),
    });

    commands.insert_resource(p);
}

const IMG_BASE_SIZE: f32 = 1080. * 4. / 5.;
const COLOR_2D_VIZ_COORD: Vec3 = Vec3::new(2000., 0., 0.);
const COLOR_2D_VIZ_SIZE: f32 = 350.;
pub const COLOR_3D_VIZ_COORD: Vec3 = Vec3::new(-2000., 0., 0.);

pub fn draw_scene<A: CSpaceProvider>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &ImageLoader)>,
    mut provider: ResMut<A>,
    image_filters: ResMut<Assets<A::FilterMaterial>>,
    mut viz2d_materials: ResMut<Assets<A::Viz2dMaterial>>,
    mut viz3d_materials: ResMut<Assets<A::Viz3dMaterial>>,
    color_materials: ResMut<Assets<ColorMaterial>>,
) {
    if query.is_empty() {
        // image already loaded
        return;
    }

    let (entity, loader) = query.single();
    let load_state = asset_server.get_load_state(&loader.0);

    if let (Some(LoadState::Loaded), Some(image)) = (load_state, images.get_mut(&loader.0)) {
        // delete marker entity
        commands.entity(entity).despawn();
        // add image to background entity
        commands.insert_resource(Background(image.clone()));

        // display image
        spawn_image::<A>(image, &mut commands, &mut meshes, &provider, image_filters);

        // display 2d viz
        spawn_2dviz_square::<A>(&mut commands, &mut meshes, &mut viz2d_materials);

        // spawn rectangles that would generate the histogram shape
        // by covering extra parts
        spawn_histogram_covering(
            &mut provider,
            image,
            &mut commands,
            &mut meshes,
            color_materials,
        );

        commands.spawn((
            Text2d::new(format!("{}", provider.current())),
            Transform::from_translation(
                COLOR_2D_VIZ_COORD + Vec3::new(0., -COLOR_2D_VIZ_SIZE * 0.6, 2.),
            ),
        ));

        commands.spawn((
            (
                Mesh3d(meshes.add(provider.create_mesh(image))),
                MeshMaterial3d(viz3d_materials.add(provider.get_viz3d_material())),
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
            (CamViewPort::Viz3d, MeshControlConf::default()),
        ));
    }
}

fn spawn_image<A: CSpaceProvider>(
    image: &mut Image,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    provider: &ResMut<A>,
    mut image_filters: ResMut<Assets<A::FilterMaterial>>,
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
            MeshMaterial2d(image_filters.add(provider.get_filter())),
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

fn spawn_2dviz_square<A: CSpaceProvider>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    viz2d_materials: &mut ResMut<Assets<A::Viz2dMaterial>>,
) {
    // Spawn corresponding 2d color distribution
    commands.spawn((
        (
            Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
            MeshMaterial2d(viz2d_materials.add(A::Viz2dMaterial::default())),
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
        CamViewPort::Viz2d,
    ));
}

fn spawn_histogram_covering<A: CSpaceProvider>(
    provider: &mut ResMut<A>,
    image: &Image,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut data = provider.histogram_data(image);
    // normalize
    let max = data
        .iter()
        .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
        .unwrap()
        .1;
    data.iter_mut().for_each(|(_, y)| *y /= max);

    let mut x = A::MIN;
    let mut iter = data.iter().peekable();
    while x <= A::MAX {
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
            Mesh2d(meshes.add(Mesh::from(Rectangle::new(A::DELTA / A::MAX, 1. - ratio)))),
            MeshMaterial2d(color_materials.add(Color::srgb_u8(42, 44, 46))),
            Transform::from_translation(
                COLOR_2D_VIZ_COORD
                            // render on top of distribution
                            + Vec3::Z
                            // move bar to corresponding color pos
                            // HACK: provider.delta() / 2. seems to fix off by 1 error
                            + Vec3::X * ((x + A::DELTA / 2.) / A::MAX - 0.5) * COLOR_2D_VIZ_SIZE
                            // align top with 2d viz top
                            + Vec3::Y * (ratio * COLOR_2D_VIZ_SIZE / 2.),
            )
            .with_scale(Vec3::splat(COLOR_2D_VIZ_SIZE)),
        ));
        x += A::DELTA;
    }
}

pub const IMG_VIEW_W_RATIO: f32 = 0.8;
pub const VIZ3D_H_RATIO: f32 = 0.5;
pub fn set_viewports(window: Single<&Window>, mut query: Query<(&CamViewPort, &mut Camera)>) {
    let size = window.physical_size();
    let img_filter_width = (size.x as f32 * IMG_VIEW_W_RATIO) as u32;
    let viz_width = (size.x as f32 * (1. - IMG_VIEW_W_RATIO)) as u32;
    let viz3d_height = (size.y as f32 * VIZ3D_H_RATIO) as u32;

    for (camera_position, mut camera) in &mut query {
        let (physical_position, physical_size) = match camera_position {
            CamViewPort::ImageFilter => (UVec2::new(0, 0), UVec2::new(img_filter_width, size.y)),
            CamViewPort::Viz2d => (
                UVec2::new(img_filter_width, viz3d_height),
                UVec2::new(viz_width, size.y - viz3d_height),
            ),
            CamViewPort::Viz3d => (
                UVec2::new(img_filter_width, 0),
                UVec2::new(viz_width, viz3d_height),
            ),
        };

        camera.viewport = Some(Viewport {
            physical_position,
            physical_size,
            ..default()
        });
    }
}
