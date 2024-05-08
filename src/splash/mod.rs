use bevy::prelude::*;

use crate::{systems::despawn_screens, UiState};

use self::systems::{count_down, spawn_splash};

mod components;
mod systems;
pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_splash)
            .add_systems(OnExit(UiState::Splash), despawn_screens)
            .add_systems(Update, count_down.run_if(in_state(UiState::Splash)));
    }
}
