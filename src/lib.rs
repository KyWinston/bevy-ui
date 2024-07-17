use bevy::prelude::*;
use bevy_lunex::{UiGenericPlugin, UiPlugin};
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use components::Ui3d;
use hud::HudPlugin;
use loading::LoadingPlugin;
use main_menu::MainMenuPlugin;
use pause::PausePlugin;
use prelude::*;
use resources::GameTitle;
use settings::SettingsUiPlugin;
use splash::SplashPlugin;
use systems::{exit, init_ui_cam};
use widgets::WidgetPlugins;

pub mod prelude {
    use bevy::{prelude::Component, reflect::Reflect, state::state::States};

    #[derive(Default, States, Debug, Reflect, Hash, Eq, PartialEq, Clone)]
    pub enum SimulationState {
        #[default]
        Running,
        Paused,
    }

    #[derive(Default, States, Component, Reflect, Debug, Hash, Eq, PartialEq, Clone)]
    pub enum UiState {
        MainMenu,
        Loading,
        Settings,
        Hud,
        #[default]
        Splash,
        Debug,
    }
    #[derive(Clone)]
    pub struct UiScreensPlugin {
        pub title: String,
    }
}

pub mod components;
pub mod hud;
pub mod loading;
pub mod main_menu;
pub mod pause;
pub mod resources;
pub mod settings;
pub mod splash;
pub mod styles;
pub mod systems;
pub mod widgets;

impl Plugin for UiScreensPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTitle(self.title.clone()))
            .add_plugins((
                ThirdPersonCameraPlugin,
                MainMenuPlugin,
                PausePlugin,
                SettingsUiPlugin,
                SplashPlugin,
                HudPlugin,
                WidgetPlugins,
                UiPlugin,
                LoadingPlugin,
                UiGenericPlugin::<Ui3d>::new(),
            ))
            .init_state::<SimulationState>()
            .init_state::<UiState>()
            .add_systems(Startup, init_ui_cam)
            .add_systems(Update, exit);
    }
}
