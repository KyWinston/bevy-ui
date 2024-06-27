use bevy::prelude::*;

use crate::components::Screen;

use super::{
    components::Loading,
    styles::{get_loading_text_styles, BASIC_BLOCK_STYLE, LOADING_SCREEN_STYLE},
};

pub fn build_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
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
        });
}

pub fn despawn_loading(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    for q in query.iter() {
        commands.entity(q).despawn_recursive();
    }
}
