use bevy::prelude::*;
use bevy_lunex::{UiClickEvent, UiSystems};
use systems::{build_main_menu, main_menu_button_clicked_system};

pub mod components;
pub mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, build_main_menu.before(UiSystems::Compute))
            .add_systems(
                Update,
                main_menu_button_clicked_system.run_if(on_event::<UiClickEvent>()),
            );
    }
}
