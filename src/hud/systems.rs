use crate::styles::CENTRAL_PANEL_STYLES;
use bevy::prelude::*;

use super::components::Hud;

pub fn spawn_hud(commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(commands, asset_server);
}

pub fn build_hud(mut commands: Commands, _asset_server: Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: CENTRAL_PANEL_STYLES,
                ..default()
            },
            Hud,
        ))
        .id()
}
