use bevy::prelude::*;

use crate::components::Screen;

use super::{
    components::Loading,
    styles::{get_loading_text_styles, BASIC_BLOCK_STYLE, LOADING_SCREEN_STYLE},
};

pub fn spawn_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_loading(&mut commands, &asset_server);
}

pub fn build_loading(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let loading_entity = commands
        .spawn((
            NodeBundle {
                style: LOADING_SCREEN_STYLE,
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            Screen,
            Loading(Some("Loading...".to_owned())),
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle { ..default() });
            parent
                .spawn(NodeBundle {
                    style: BASIC_BLOCK_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "loading...",
                                get_loading_text_styles(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    loading_entity
}
