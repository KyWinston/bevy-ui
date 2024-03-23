use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;
use crate::AppState;

use self::{
    events::StartLoad,
    systems::{interact_with_play_button, spawn_main_menu},
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartLoad>()
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, interact_with_play_button);
    }
}
