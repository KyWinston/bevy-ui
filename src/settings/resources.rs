use bevy::prelude::*;

#[derive(Resource, serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct RenderSettings {
    pub brightness: f32,
}

pub struct VolumeSettings;

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct NetworkSettings {
    pub port: u16,
    pub players: Vec<String>,
}

#[derive(Resource)]
pub struct NetworkSettingsHandle(pub Handle<NetworkSettings>);
