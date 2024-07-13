use bevy::{
    core_pipeline::fxaa::Fxaa,
    pbr::ScreenSpaceReflectionsBundle,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    window::PrimaryWindow,
};
use bevy_lunex::{
    prelude::{MainUi, Pickable, Rl, Scaling, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, PackageLayout, PickingPortal, UiImage2dBundle, UiLayout, UiLink,
    UiTreeBundle,
};
use bevy_third_person_camera::{
    ThirdPersonCamera, {Offset, Zoom},
};

use super::components::{Hud, UiDisplay};
use crate::components::MainCam;

pub fn build_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    primary_window: Query<(Entity, &Window), With<PrimaryWindow>>,
    query: Query<Entity, Added<Hud>>,
) {
    for route_entity in &query {
        if let Ok(window) = primary_window.get_single() {
            let size = Extent3d {
                width: window.1.resolution.width() as u32,
                height: window.1.resolution.height() as u32,
                ..default()
            };

            let mut image = Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Bgra8UnormSrgb,
                    mip_level_count: 1,
                    sample_count: 1,
                    usage: TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_DST
                        | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                ..default()
            };

            // Spawn the route
            commands
                .entity(route_entity)
                .insert(SpatialBundle::default())
                .with_children(|route| {
                    image.resize(size);
                    let render_image = asset_server.add(image);
                    //player_slot
                    route
                        .spawn((SpatialBundle::default(), MainCam))
                        .with_children(|route| {
                            // Spawn 3D camera
                            route.spawn((
                                Camera3dBundle {
                                    camera: Camera {
                                        order: -1,
                                        target: render_image.clone().into(),
                                        clear_color: ClearColorConfig::Custom(Color::srgba(
                                            0.0, 0.0, 0.0, 0.0,
                                        )),
                                        ..default()
                                    },
                                    projection: Projection::Perspective(PerspectiveProjection {
                                        fov: 60.0_f32.to_radians(),
                                        ..default()
                                    }),
                                    ..default()
                                },
                                VisibilityBundle::default(),
                                ThirdPersonCamera {
                                    cursor_lock_active: true,
                                    aim_enabled: true,
                                    aim_speed: 6.0,
                                    offset_enabled: true,
                                    offset: Offset::new(1., 1.),
                                    aim_zoom: 0.7,
                                    zoom_enabled: true,
                                    zoom: Zoom::new(5.5, 10.0),
                                    sensitivity: Vec2::new(4.0, 4.0),
                                    ..default()
                                },
                                ScreenSpaceReflectionsBundle::default(),
                                Fxaa::default(),
                                EnvironmentMapLight {
                                    diffuse_map: asset_server.load(
                                        "images/environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2",
                                    ),
                                    specular_map: asset_server.load(
                                        "images/environment_maps/pisa_specular_rgb9e5_zstd.ktx2",
                                    ),
                                    intensity: 400.0,
                                },
                            ));
                        });

                    // Spawn the background
                    route
                        .spawn((
                            UiTreeBundle::<MainUi>::from(UiTree::new("HUD")),
                            MovableByCamera,
                            UiDisplay,
                        ))
                        .with_children(|ui| {
                            // Spawn 3D camera view
                            ui.spawn((
                                UiLink::<MainUi>::path("Camera"),
                                UiLayout::window_full().pack::<Base>(), // Make this resizable
                                UiImage2dBundle::from(render_image),
                                PickingPortal,
                            ));
                            ui.spawn((
                                UiLink::<MainUi>::path("Camera/Hud"),
                                UiLayout::solid()
                                    .size(Rl(100.0))
                                    .scaling(Scaling::Fit)
                                    .pack::<Base>(),
                                Pickable::IGNORE,
                            ));
                        });
                });
        }
    }
}
