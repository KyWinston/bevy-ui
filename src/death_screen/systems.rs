use bevy::prelude::*;

use crate::{styles::*, components::{Screen, BasicButtonBundle, RespawnButton, ButtonTextBundle, QuitButton}};

use super::components::Death;

pub fn spawn_death_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("death screen spawned");
    build_death_screen(&mut commands, &asset_server);
}

pub fn build_death_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_over_entity = commands
        .spawn((
            NodeBundle {
                style: CENTRAL_PANEL_STYLES,
                ..default()
            },
            Death,
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
                                "You Died!",
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
            //====play button
            parent
                .spawn((BasicButtonBundle::new(), RespawnButton))
                .with_children(|parent| {
                    parent.spawn(ButtonTextBundle::new("Respawn".to_owned(), asset_server));
                });
            //====quit button
            parent
                .spawn((BasicButtonBundle::new(), QuitButton))
                .with_children(|parent| {
                    parent.spawn(ButtonTextBundle::new("Quit".to_owned(), asset_server));
                });
        })
        .id();
    game_over_entity
}
