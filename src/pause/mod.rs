use bevy::prelude::*;

pub mod components;
mod systems;

use crate::SimulationState;

use self::systems::{interact_with_resume_button, spawn_pause};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Paused), spawn_pause)
            .add_systems(
                Update,
                interact_with_resume_button.run_if(in_state(SimulationState::Paused)),
            );
    }
}
