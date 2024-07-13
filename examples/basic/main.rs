use bevy::prelude::*;
use bevy_basic_ui::prelude::*;

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
