use bevy::prelude::*;
use resources::GameTitle;
use settings::SettingsPlugin;
use splash::SplashPlugin;

use self::{
    loading::LoadingPlugin,
    main_menu::MainMenuPlugin,
    pause::PausePlugin,
    systems::{interact_with_quit_button, interact_with_settings_button},
};

pub mod components;
pub mod loading;
pub mod main_menu;
pub mod pause;
pub mod resources;
pub mod settings;
pub mod splash;
pub mod styles;
pub mod systems;

#[derive(Clone)]
pub struct UiScreensPlugin {
    pub title: String,
}

impl Plugin for UiScreensPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTitle(self.title.clone()))
            .add_plugins((
                MainMenuPlugin,
                PausePlugin,
                SettingsPlugin,
                SplashPlugin,
                LoadingPlugin,
            ))
            .add_systems(
                Update,
                (interact_with_quit_button, interact_with_settings_button),
            );
    }
}

#[derive(Default, Component, States, Debug, Hash, Eq, PartialEq, Clone, Reflect)]
#[reflect(Component)]
pub enum UiState {
    MainMenu,
    Loading,
    Settings,
    Hud,
    #[default]
    Splash,
    Debug,
}

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
