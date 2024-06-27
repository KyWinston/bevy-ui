use bevy::prelude::*;
use systems::build_hud;

use crate::UiState;

pub mod components;
pub mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Hud), build_hud);
    }
}
