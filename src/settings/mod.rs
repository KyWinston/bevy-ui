use bevy::prelude::*;

use crate::UiState;

use self::systems::{build_settings, spawn_settings};

pub mod components;
pub mod resources;
pub mod styles;
pub mod systems;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(RenderSettings { brightness: 1.0 })
        app.add_systems(OnEnter(UiState::Settings), spawn_settings);
    }
}
