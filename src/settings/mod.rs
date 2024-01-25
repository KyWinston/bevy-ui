use bevy::prelude::*;

use self::{
    resources::RenderSettings,
    systems::{fetch_settings, spawn_settings},
};

pub mod components;
pub mod resources;
pub mod styles;
pub mod systems;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RenderSettings {
            brightness: 1.0,
            crt_opacity: 0.05,
            vignette_opacity: 0.2,
            mask_strength: 0.2,
        })
        .add_systems(Startup, (fetch_settings, spawn_settings));
    }
}
