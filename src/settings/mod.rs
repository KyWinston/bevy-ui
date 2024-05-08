use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

use crate::{systems::despawn_screens, UiState};

use self::{
    resources::{AllSettings, SettingsVals},
    systems::{assign_to_resource, init_settings, load_settings_toml, spawn_settings},
};

pub mod components;
pub mod events;
pub mod resources;
pub mod styles;
pub mod systems;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<SettingsVals>(SettingsVals(vec![]))
            .add_systems(OnEnter(UiState::Settings), spawn_settings)
            .add_systems(OnExit(UiState::Settings), despawn_screens)
            .add_systems(
                OnEnter(UiState::Splash),
                (init_settings, assign_to_resource),
            )
            .add_systems(Startup, load_settings_toml)
            .add_plugins(TomlAssetPlugin::<AllSettings>::new(&["settings.toml"]));
    }
}
