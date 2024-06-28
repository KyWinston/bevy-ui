use crate::SimulationState;

use self::systems::{build_pause, interact_with_pause_button};
use bevy::prelude::*;
use systems::despawn_pause;

pub mod components;
mod systems;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (build_pause, interact_with_pause_button))
            .add_systems(OnExit(SimulationState::Paused), despawn_pause);
    }
}
