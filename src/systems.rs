use bevy::prelude::*;
use bevy_lunex::{
    prelude::{MainUi, Pickable},
    Cursor2d,
};

use crate::{hud::components::Hud, splash::components::SplashScreen, UiState};

pub fn init_ui_cam(mut commands: Commands) {
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

pub fn switch_to_menu(mut state: ResMut<NextState<UiState>>) {
    state.set(UiState::MainMenu);
}
