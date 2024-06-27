use bevy::prelude::*;
use bevy_lunex::UiPlugin;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use hud::HudPlugin;
use loading::LoadingPlugin;
use main_menu::MainMenuPlugin;
use resources::GameTitle;
use splash::SplashPlugin;
use systems::init_ui_cam;
use widgets::button::ButtonPlugin;


pub mod components;
pub mod hud;
pub mod loading;
pub mod main_menu;
// pub mod pause;
pub mod resources;
// pub mod settings;
pub mod splash;
pub mod styles;
pub mod systems;
pub mod widgets;

#[derive(Clone)]
pub struct UiScreensPlugin {
    pub title: String,
}

impl Plugin for UiScreensPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTitle(self.title.clone()))
            .add_plugins((
                MainMenuPlugin,
                // PausePlugin,
                // SettingsPlugin,
                SplashPlugin,
                HudPlugin,
                ButtonPlugin,
                ThirdPersonCameraPlugin,
                UiPlugin,
                LoadingPlugin,
            )).add_systems(Startup, init_ui_cam);
    }
}

#[derive(Default, States, Component, Debug, Hash, Eq, PartialEq, Clone, Reflect)]
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
