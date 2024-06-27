use bevy::prelude::*;
use bevy_basic_ui::{splash::components::SplashScreen, SimulationState, UiScreensPlugin, UiState};
use bevy_lunex::prelude::MainUi;

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
        .run();
}
