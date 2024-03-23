use bevy::prelude::*;

use crate::{
    components::{BasicButtonBundle, ButtonTextBundle, QuitButton, Screen, SettingsButton},
    styles::*,
    systems::despawn_screens,
};

use super::{
    components::{MainMenu, PlayButton},
    events::StartLoad,
};

pub fn interact_with_play_button(
    commands: Commands,
    screen_q: Query<Entity, With<Screen>>,
    mut button_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut load: EventWriter<StartLoad>,
) {
    if let Ok((interaction, mut background_color)) = button_q.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                despawn_screens(commands, screen_q);
                load.send(StartLoad);
            }

            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: CENTRAL_PANEL_STYLES,
                background_color: Color::GRAY.into(),
                ..default()
            },
            MainMenu,
            Screen,
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
                                "Millipede Railgun",
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
                .spawn((BasicButtonBundle::new(), PlayButton))
                .with_children(|parent| {
                    parent.spawn(ButtonTextBundle::new("Play".to_owned(), asset_server));
                });
            //====options button
            parent
                .spawn((BasicButtonBundle::new(), SettingsButton))
                .with_children(|parent| {
                    parent.spawn(ButtonTextBundle::new("Options".to_owned(), asset_server));
                });
            //====quit button
            parent
                .spawn((BasicButtonBundle::new(), QuitButton))
                .with_children(|parent| {
                    parent.spawn(ButtonTextBundle::new("Quit".to_owned(), asset_server));
                });
        })
        .id();
    main_menu_entity
}
