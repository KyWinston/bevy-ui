use bevy::prelude::*;
use bevy_lunex::UiSystems;
use systems::build_main_menu;
pub mod components;
pub mod events;
pub mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, build_main_menu.before(UiSystems::Compute));
    }
}
