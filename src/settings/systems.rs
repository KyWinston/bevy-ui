use bevy::prelude::*;

use crate::{
    components::{BasicButtonBundle, ButtonTextBundle, QuitButton, Screen},
    styles::{get_title_text_styles, CENTRAL_PANEL_STYLES, IMAGE_STYLE, TITLE_STYLE},
};

use super::{
    components::SettingsUi,
    resources::NetworkSettingsHandle,
    styles::{get_subtitle_text_styles, SECTION_STYLE},
};

pub fn spawn_settings(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_settings(&mut commands, &asset_server);
}

pub fn build_settings(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let settings_entity = commands
        .spawn((
            NodeBundle {
                style: CENTRAL_PANEL_STYLES,
                ..default()
            },
            SettingsUi,
            Screen
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
