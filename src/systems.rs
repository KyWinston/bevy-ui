use bevy::prelude::*;
use bevy_lunex::{
    prelude::{MainUi, Pickable},
    Cursor2d,
};

use crate::{hud::components::Hud, splash::components::SplashScreen, UiState};

pub fn init_ui_cam(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera { ..default() },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            MainUi,
        ))
        .with_children(|cam| {
            cam.spawn((
                Cursor2d::new()
                    .native_cursor(false)
                    .register_cursor(CursorIcon::Default, 0, (14.0, 14.0))
                    .register_cursor(CursorIcon::Pointer, 1, (10.0, 12.0))
                    .register_cursor(CursorIcon::Grab, 2, (40.0, 40.0)),
                Pickable::IGNORE,
            ));
        });

    commands.spawn(Hud);
    commands.spawn(SplashScreen);
}

// pub fn interact_with_settings_button(
//     mut button_q: Query<
//         (&Interaction, &mut BackgroundColor),
//         (Changed<Interaction>, With<SettingsButton>),
//     >,
//     mut settings_state: ResMut<NextState<UiState>>,
// ) {
//     if let Ok((interaction, mut background_color)) = button_q.get_single_mut() {
//         match *interaction {
//             Interaction::Pressed => {
//                 *background_color = PRESSED_BUTTON_COLOR.into();
//                 settings_state.set(UiState::Settings);
//             }
//             Interaction::Hovered => {
//                 *background_color = HOVERED_BUTTON_COLOR.into();
//             }
//             Interaction::None => {
//                 *background_color = NORMAL_BUTTON_COLOR.into();
//             }
//         }
//     }
// }

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

pub fn switch_to_menu(mut state: ResMut<NextState<UiState>>) {
    state.set(UiState::MainMenu);
}
