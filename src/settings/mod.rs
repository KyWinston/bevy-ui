use bevy::prelude::*;

use crate::UiState;

use self::{
    resources::SettingsVals,
    systems::{insert_setting, spawn_settings},
};

pub mod components;
pub mod resources;
pub mod styles;
pub mod systems;
pub mod events;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<SettingsVals>(SettingsVals(vec![]))
            .add_systems(OnEnter(UiState::Settings), spawn_settings)
            .add_systems(Startup, insert_setting);
    }
}
