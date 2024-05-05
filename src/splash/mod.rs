use bevy::prelude::*;

use crate::{systems::despawn_screens, UiState};

use self::systems::build_splash;

mod components;
mod systems;
pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Splash), build_splash)
            .add_systems(OnExit(UiState::Splash), despawn_screens);
    }
}
