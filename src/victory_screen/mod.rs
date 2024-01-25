use bevy::prelude::*;

pub mod components;
pub mod systems;


use crate::SimulationState;

use self::systems::spawn_victory_screen;
pub struct VictoryScreenPlugin;

impl Plugin for VictoryScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState), spawn_victory_screen);
    }
}
