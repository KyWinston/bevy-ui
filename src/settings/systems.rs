use bevy::prelude::*;


use crate::{
    components::{BasicButtonBundle, ButtonTextBundle, QuitButton, Screen, UiMainRootNode},
    styles::{get_title_text_styles, CENTRAL_PANEL_STYLES, IMAGE_STYLE, TITLE_STYLE},
};

use super::{
    components::SettingsUi,
    styles::{get_subtitle_text_styles, SECTION_STYLE},
};

pub fn build_settings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    main_node: Query<Entity, With<UiMainRootNode>>,
) {
    let Ok(main) = main_node.get_single() else {
        return;
    };
    commands
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
        });
}
