use crate::SimulationState;

use self::systems::build_pause;
use bevy::prelude::*;
use bevy_lunex::{UiClickEvent, UiSystems};
use systems::{despawn_pause, pause_button_clicked_system};

pub mod components;
mod systems;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, build_pause.before(UiSystems::Compute))
            .add_systems(
                Update,
                pause_button_clicked_system.run_if(on_event::<UiClickEvent>()),
            )
            .add_systems(OnExit(SimulationState::Paused), despawn_pause);
    }
}
