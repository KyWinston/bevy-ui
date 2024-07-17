use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::{components::Quit, splash::components::SplashScreen};

pub fn init_ui_cam(mut commands: Commands) {
    commands.spawn(camera()).with_children(|cam| {
        let mut cursor = Cursor2d::new();
        cursor.request_cursor(CursorIcon::Default, 0.0);
        cursor.request_cursor(CursorIcon::Pointer, 1.0);
        cursor.request_cursor(CursorIcon::Grab, 2.0);

        cam.spawn((
            cursor,
            Pickable::IGNORE,
            PointerBundle::new(PointerId::Custom(pointer::Uuid::new_v4())),
        ));
    });
    commands.spawn(SplashScreen);
}

pub fn exit(mut app_exit_event_writer: EventWriter<AppExit>, quit: Query<Entity, Added<Quit>>) {
    for _ in &quit {
        app_exit_event_writer.send(AppExit::Success);
    }
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
    )
}
