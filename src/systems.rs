use bevy::{app::AppExit, prelude::*};
use sickle_ui::{
    ui_builder::{UiBuilderExt, UiRoot},
    widgets::container::UiContainerExt,
};

use crate::{
    components::{UiCamera, UiMainRootNode},
    resources::IconCache,
    styles::CENTRAL_PANEL_STYLES,
    UiState,
};

use super::{
    components::{QuitButton, Screen, SettingsButton},
    styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
};

pub fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut cam_q: Query<Entity, With<UiCamera>>,
    mut icon_cache: ResMut<IconCache>,
) {
    let icons_to_cache: Vec<&str> = vec![
        "embedded://sickle_ui/icons/checkmark.png",
        "embedded://sickle_ui/icons/chevron_down.png",
        "embedded://sickle_ui/icons/chevron_left.png",
        "embedded://sickle_ui/icons/chevron_right.png",
        "embedded://sickle_ui/icons/chevron_up.png",
        "embedded://sickle_ui/icons/close.png",
        "embedded://sickle_ui/icons/exit_white.png",
        "embedded://sickle_ui/icons/popout_white.png",
        "embedded://sickle_ui/icons/redo_white.png",
        "embedded://sickle_ui/icons/submenu_white.png",
    ];

    for icon in icons_to_cache.iter() {
        icon_cache.0.push(asset_server.load(*icon));
    }

    if let Ok(cam) = cam_q.get_single_mut() {
        commands.ui_builder(UiRoot).container(
            (
                NodeBundle {
                    style: CENTRAL_PANEL_STYLES,
                    background_color: Color::rgb(0.29, 0.29, 0.29).into(),
                    ..default()
                },
                UiMainRootNode,
                Screen,
                TargetCamera(cam),
            ),
            |_| {},
        );
    }
}

pub fn interact_with_quit_button(
    commands: Commands,
    mut button_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    state: Res<State<UiState>>,
    screen_q: Query<Entity, With<Screen>>,
    mut next_state: ResMut<NextState<UiState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_q.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                despawn_screens(commands, screen_q);
                match state.get() {
                    UiState::Hud => {
                        next_state.set(UiState::MainMenu);
                    }
                    UiState::MainMenu => {
                        app_exit_event_writer.send(AppExit);
                    }
                    UiState::Settings => {
                        next_state.set(UiState::MainMenu);
                    }
                    _ => (),
                }
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

pub fn interact_with_settings_button(
    mut button_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SettingsButton>),
    >,
    mut settings_state: ResMut<NextState<UiState>>,
) {
    if let Ok((interaction, mut background_color)) = button_q.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                settings_state.set(UiState::Settings);
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

// pub fn interact_with_respawn_button(
//     mut button_q: Query<
//         (&Interaction, &mut BackgroundColor),
//         (Changed<Interaction>, With<RespawnButton>),
//     >,
//     mut spawn_ev: EventWriter<SpawnEvent>,
//     mut state: ResMut<NextState<SimulationState>>,
//     cam_q: Query<&CameraTarget, With<Camera>>,
//     commands: Commands,
//     respawn_q: Query<Entity, With<Screen>>,
//     SimulationState maps: Res<Assets<Map>>,
//     map: Res<MapHandle>,
// ) {
//     if let Ok(cam) = cam_q.get_single() {
//         if let Some(map) = maps.get(map.0.id()) {
//             if let Ok((interaction, mut background_color)) = button_q.get_single_mut() {
//                 match *interaction {
//                     Interaction::Pressed => {
//                         *background_color = PRESSED_BUTTON_COLOR.into();
//                         despawn_screens(commands, respawn_q);
//                         state.set(SimulationState::Running);
//                         spawn_ev.send(SpawnEvent {
//                             player_number: cam.target,
//                             transform: Vec3 {
//                                 x: 5.0,
//                                 y: map.map_height as f32 * 2.0 + 1.0,
//                                 z: 0.0,
//                             },
//                         });
//                     }
//                     Interaction::Hovered => {
//                         *background_color = HOVERED_BUTTON_COLOR.into();
//                     }
//                     Interaction::None => {
//                         *background_color = NORMAL_BUTTON_COLOR.into();
//                     }
//                 }
//             }
//         }
//     }
// }

pub fn despawn_screens(mut commands: Commands, mut screen_q: Query<Entity, With<Screen>>) {
    for ent in screen_q.iter_mut() {
        commands.entity(ent).despawn_recursive();
    }
}

pub fn switch_to_menu(mut state: ResMut<NextState<UiState>>) {
    state.set(UiState::MainMenu);
}
