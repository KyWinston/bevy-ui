use bevy::prelude::*;

use super::styles::{get_button_text_styles, BASIC_BUTTON_STYLE, NORMAL_BUTTON_COLOR};

#[derive(Component)]
pub struct Screen;

#[derive(Component)]
pub struct RespawnButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Bundle)]
pub struct BasicButtonBundle {
    button: ButtonBundle,
}

#[derive(Bundle)]
pub struct ButtonTextBundle {
    button_text: TextBundle,
}

impl BasicButtonBundle {
    pub fn new() -> Self {
        Self {
            button: ButtonBundle {
                style: BASIC_BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
        }
    }
}

impl ButtonTextBundle {
    pub fn new(text: String, asset_server: &Res<AssetServer>) -> Self {
        Self {
            button_text: TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        text,
                        get_button_text_styles(&asset_server),
                    )],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenuScreen;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;
