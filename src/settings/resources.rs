use bevy::prelude::*;

#[derive(Resource, serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct RenderSettings {
    pub brightness: f32,
    pub vignette_opacity: f32,
    pub crt_opacity: f32,
    pub mask_strength: f32,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct NetworkSettings {
    pub port: u16,
    pub players: Vec<String>,
}

#[derive(Resource)]
pub struct NetworkSettingsHandle(pub Handle<NetworkSettings>);
