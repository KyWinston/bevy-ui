use bevy::prelude::*;
use serde::{Deserialize, Serialize};
// use bevy_common_assets::toml::TomlAssetPlugin;

// use crate::UiState;

use self::{
    resources::SettingsVals,
    // systems::{assign_to_resource, init_settings, load_settings_toml, spawn_settings},
};

pub mod components;
pub mod events;
pub mod resources;
pub mod styles;
// pub mod systems;


#[derive(Resource, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "bevy_settings::serde")]
struct Settings {
    master_volume: f64,
    custom_cursor: bool,
}
pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<SettingsVals>(SettingsVals(vec![]))
        .add_plugins(bevy_settings::SettingsPlugin::<Settings>::new(
            "GkPixel",
            "Steelies: Arena"
        ));
            // .add_systems(OnEnter(UiState::Settings), spawn_settings)
        // .add_systems(
        //     OnEnter(UiState::Splash),
        //     (init_settings, assign_to_resource),
        // )
        // .add_systems(Startup, load_settings_toml);
        //  .add_plugins(TomlAssetPlugin::<AllSettings>::new(&["settings.toml"]));
    }
}
