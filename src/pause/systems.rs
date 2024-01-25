use bevy::prelude::*;

use crate::{SimulationState, components::{Screen, BasicButtonBundle, QuitButton, ButtonTextBundle}, styles::*, systems::despawn_screens};

use super::components::{Pause, ResumeButton};

pub fn interact_with_resume_button(
    commands: Commands,
    mut button_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    pause_q: Query<Entity, With<Screen>>,
    mut state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_q.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                despawn_screens(commands, pause_q);
                state.set(SimulationState::Running);
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

pub fn spawn_pause(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause(&mut commands, &asset_server);
}

pub fn build_pause(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_entity = commands
        .spawn((
            NodeBundle {
                style: CENTRAL_PANEL_STYLES,
                ..default()
            },
            Pause,
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
                                "Paused",
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
                .spawn((BasicButtonBundle::new(), ResumeButton))
                .with_children(|parent| {
                    parent.spawn(ButtonTextBundle::new("Resume".to_owned(), asset_server));
                });
            //====quit button
            parent
                .spawn((BasicButtonBundle::new(), QuitButton))
                .with_children(|parent| {
                    parent.spawn(ButtonTextBundle::new("Quit".to_owned(), asset_server));
                });
        })
        .id();
    pause_entity
}
