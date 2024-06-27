use bevy::prelude::*;
use systems::{build_main_menu, interact_with_menu_button};

pub mod components;
pub mod events;
pub mod systems;

use crate::UiState;

use self::events::StartLoad;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartLoad>()
            .add_systems(OnEnter(UiState::MainMenu), build_main_menu)
            .add_systems(Update, interact_with_menu_button);
    }
}
