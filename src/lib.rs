use bevy::prelude::*;
use splash::SplashPlugin;

use self::{
    death_screen::DeathScreenPlugin,
    loading::LoadingPlugin,
    main_menu::MainMenuPlugin,
    pause::PausePlugin,
    // settings::SettingsPlugin,
    systems::{interact_with_quit_button, interact_with_settings_button},
};

pub mod components;
pub mod death_screen;
pub mod loading;
pub mod main_menu;
pub mod pause;
pub mod splash;
// pub mod settings;
pub mod styles;
pub mod systems;
// pub mod victory_screen;

pub struct UiScreensPlugin;

impl Plugin for UiScreensPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .insert_state::<AppState>(AppState::Splash)
            .add_plugins((
                MainMenuPlugin,
                PausePlugin,
                // SettingsPlugin,
                SplashPlugin,
                LoadingPlugin,
                DeathScreenPlugin,
                // VictoryScreenPlugin,
            ))
            .add_systems(
                Update,
                (interact_with_quit_button, interact_with_settings_button),
            );
    }
}

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    MainMenu,
    Loading,
    Editor,
    Game,
    #[default]
    Splash,
}

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
    Dead,
}
