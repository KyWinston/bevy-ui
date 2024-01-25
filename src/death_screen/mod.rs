use bevy::prelude::*;

pub mod components;
pub mod systems;


use crate::SimulationState;

use self::systems::spawn_death_screen;
pub struct DeathScreenPlugin;

impl Plugin for DeathScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Dead), spawn_death_screen);
    }
}
