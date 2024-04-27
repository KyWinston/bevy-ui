use bevy::prelude::*;

use crate::{
    components::{BasicButtonBundle, ButtonTextBundle, QuitButton, Screen, UiMainRootNode},
    styles::{get_title_text_styles, CENTRAL_PANEL_STYLES, IMAGE_STYLE, TITLE_STYLE},
};

use super::{
    components::SettingsUi,
    styles::{get_subtitle_text_styles, SECTION_STYLE},
};

pub fn spawn_settings(commands: Commands, asset_server: Res<AssetServer>) {
    build_settings(commands, asset_server)
}

pub fn build_settings(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root_node = commands
        .spawn(NodeBundle {
            style: CENTRAL_PANEL_STYLES,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Settings".to_string(),
                                get_title_text_styles(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
}
