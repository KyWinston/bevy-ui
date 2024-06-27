use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasBundle,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use bevy_lunex::{
    prelude::{MainUi, Pickable, Scaling, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, PackageLayout, UiImage2dBundle, UiLayout, UiLink, UiTreeBundle,
};
use bevy_third_person_camera::{
    camera::{Offset, Zoom},
    ThirdPersonCamera,
};

use crate::components::MainCam;

use super::components::Hud;

pub fn build_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<Hud>>,
) {
    for route_entity in &query {
        let size = Extent3d {
            width: 1920,
            height: 1080,
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
                // Spawn 3D camera
                route.spawn((
                    Camera3dBundle {
                        camera: Camera {
                            order: -1,
                            target: render_image.clone().into(),
                            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                            hdr: true,
                            ..default()
                        },

                        ..default()
                    },
                    MainCam,
                    TemporalAntiAliasBundle::default(),
                    EnvironmentMapLight {
                        diffuse_map: asset_server
                            .load("images/environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
                        specular_map: asset_server
                            .load("images/environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
                        intensity: 1000.0,
                    },
                    ThirdPersonCamera {
                        cursor_lock_active: false,
                        aim_enabled: true,
                        aim_speed: 6.0,
                        offset_enabled: true,
                        offset: Offset::new(1., 1.),
                        aim_zoom: 0.7,
                        zoom_enabled: true,
                        zoom: Zoom::new(5.5, 10.0),
                        // sensitivity: Vec2::new(settings.0[0].value as f32, settings.0[1].value as f32),
                        ..default()
                    },
                ));

                route
                    .spawn((
                        UiTreeBundle::<MainUi>::from(UiTree::new("CamView")),
                        MovableByCamera,
                    ))
                    .with_children(|ui| {
                        // Spawn the root div
                        let root = UiLink::<MainUi>::path("Root"); // Here we can define the name of the node
                        ui.spawn((
                            root.clone(),                           // Here we add the link
                            UiLayout::window_full().pack::<Base>(), // This is where we define layout
                        ));

                        // Spawn the background
                        ui.spawn((
                            root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
                            UiLayout::solid()
                                .size((1920.0, 1080.0))
                                .scaling(Scaling::Fill)
                                .pack::<Base>(),
                            Pickable::IGNORE,
                        ));

                        ui.spawn((
                            root.add("Background/Camera"),
                            UiLayout::solid()
                                .size((1920.0, 1080.0))
                                .scaling(Scaling::Fill)
                                .pack::<Base>(),
                            UiImage2dBundle::from(render_image.clone()),
                            Pickable::IGNORE,
                        ));
                    });
            });
    }
}
