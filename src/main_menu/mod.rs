use bevy::prelude::*;

pub mod components;
mod systems;

use crate::AppState;

use self::systems::{interact_with_play_button, spawn_main_menu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, interact_with_play_button);
    }
}
