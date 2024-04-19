use bevy::prelude::*;

pub mod components;
mod styles;
mod systems;

use crate::UiState;

use self::systems::{despawn_loading, spawn_loading};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Loading), spawn_loading);
        app.add_systems(OnExit(UiState::Loading), despawn_loading);
    }
}
