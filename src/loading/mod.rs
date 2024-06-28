use bevy::prelude::*;
use systems::{build_loading, despawn_loading};

pub mod components;
mod styles;
mod systems;

use crate::UiState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Loading), build_loading);
        app.add_systems(OnExit(UiState::Loading), despawn_loading);
    }
}
