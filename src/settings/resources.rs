use bevy::{
    asset::{Asset, Handle},
    ecs::system::Resource,
    reflect::TypePath,
};

use serde::Deserialize;

#[derive(Resource, Asset, TypePath)]
pub struct SettingsVals(pub Vec<SettingsVal>);

#[derive(Deserialize, Asset, TypePath, Clone, Debug)]
pub struct SettingsVal {
    pub tag: String,
    pub value: u32,
}

#[derive(Asset, TypePath, Deserialize)]
pub struct SettingsCategory {
    pub name: String,
    pub contents: Vec<SettingsVal>,
}

#[derive(Asset, TypePath, Deserialize)]
pub struct AllSettings {
    pub categories: Vec<SettingsCategory>,
}

#[derive(Resource)]
pub struct TomlAsset(pub Handle<AllSettings>);
