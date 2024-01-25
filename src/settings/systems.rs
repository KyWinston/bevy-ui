use bevy::prelude::*;

use crate::{
    ui::components::QuitButton,
    ui::{
        components::{BasicButtonBundle, ButtonTextBundle},
        styles::*,
    },
};

use super::{
    components::SettingsUi,
    resources::NetworkSettingsHandle,
    styles::{get_subtitle_text_styles, SECTION_STYLE},
};

pub fn spawn_settings(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause(&mut commands, &asset_server);
}

pub fn fetch_settings(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(NetworkSettingsHandle(
        asset_server.load("network.settings.toml"),
    ));
}

// pub fn despawn_settings(mut commands: Commands, settings_q: Query<Entity, With<SettingsUi>>) {
//     if let Ok(settings_entity) = settings_q.get_single() {
//         commands.entity(settings_entity).despawn_recursive();
//     }
// }

pub fn build_pause(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let settings_entity = commands
        .spawn((
            NodeBundle {
                style: CENTRAL_PANEL_STYLES,
                visibility: Visibility::Hidden,
                ..default()
            },
            SettingsUi,
        ))
        .with_children(|parent| {
            //====title
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Image 1
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("images/tile_0003.png").into(),
                        ..default()
                    });
                    //Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Settings",
                                get_title_text_styles(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("images/tile_0003.png").into(),
                        ..default()
                    });
                });
            parent
                .spawn(NodeBundle {
                    style: SECTION_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    //Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Render Settings",
                                get_subtitle_text_styles(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
            parent.spawn(NodeBundle {
                style: SECTION_STYLE,
                ..default()
            });

            // ====Back button
            parent
                .spawn((BasicButtonBundle::new(), QuitButton))
                .with_children(|parent| {
                    parent.spawn(ButtonTextBundle::new("Back".to_owned(), asset_server));
                });
        })
        .id();
    settings_entity
}
