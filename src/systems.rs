use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::{components::Quit, splash::components::SplashScreen, UiState};

pub fn init_ui_cam(mut commands: Commands) {
    commands.spawn(camera()).with_children(|cam| {
        cam.spawn((
            Cursor2d::new()
                .native_cursor(false)
                .register_cursor(CursorIcon::Default, 0, (14.0, 14.0))
                .register_cursor(CursorIcon::Pointer, 1, (10.0, 12.0))
                .register_cursor(CursorIcon::Grab, 2, (40.0, 40.0)),
            Pickable::IGNORE,
        ));
    });
    commands.spawn(SplashScreen);
}

pub fn exit(mut app_exit_event_writer: EventWriter<AppExit>, quit: Query<Entity, Added<Quit>>) {
    for _ in &quit {
        app_exit_event_writer.send(AppExit::Success);
    }
}

pub fn switch_to_menu(mut state: ResMut<NextState<UiState>>) {
    state.set(UiState::MainMenu);
}

/// Function to return camera will all appropriate settings
pub fn camera() -> impl Bundle {
    (
        MainUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        InheritedVisibility::default(),
    )
}
