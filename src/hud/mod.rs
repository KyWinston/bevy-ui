
use bevy::prelude::*;
// use bevy::state::{OnEnter, OnExit};

use crate::{systems::despawn_screens, UiState};

use self::systems::spawn_hud;

pub mod components;
pub mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Hud), spawn_hud)
            .add_systems(OnExit(UiState::Hud), despawn_screens);
    }
}
