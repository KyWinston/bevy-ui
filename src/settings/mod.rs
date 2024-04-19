use bevy::prelude::*;

use crate::UiState;

use self::{resources::RenderSettings, systems::spawn_settings};

pub mod components;
pub mod resources;
pub mod styles;
pub mod systems;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RenderSettings { brightness: 1.0 })
            .add_systems(OnEnter(UiState::Settings), spawn_settings);
    }
}
