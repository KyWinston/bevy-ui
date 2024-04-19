use bevy::prelude::*;

use crate::{systems::despawn_screens, UiState};

use self::systems::{build_splash, countdown};

mod components;
mod systems;
pub struct SplashPlugin;
// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app.add_systems(OnEnter(UiState::Splash), build_splash)
            .add_systems(Update, countdown.run_if(in_state(UiState::Splash)))
            .add_systems(OnExit(UiState::Splash), despawn_screens);
    }
}
