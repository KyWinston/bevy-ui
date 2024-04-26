use bevy::prelude::*;
use bevy_basic_ui::{components::UiCamera, SimulationState, UiScreensPlugin, UiState};

fn main() {
    App::new()
        .init_state::<UiState>()
        .init_state::<SimulationState>()
        .add_plugins((
            DefaultPlugins,
            UiScreensPlugin {
                title: "Test".to_string(),
            },
        ))
        .add_systems(Startup, init_cam)
        .run();
}

fn init_cam(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), UiCamera));
}
