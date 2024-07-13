use bevy::prelude::*;

// use super::styles::{get_button_text_styles, BASIC_BUTTON_STYLE};
#[derive(Component, Clone)]
pub struct Ui3d;

// use super::styles::{get_button_text_styles, BASIC_BUTTON_STYLE};
#[derive(Component, Clone)]
pub struct Quit;

#[derive(Component)]
pub struct UiMainRootNode;

#[derive(Component)]
pub struct UiFooterRootNode;

#[derive(Component, Clone)]
pub struct MainCam;

#[derive(Component)]
pub struct Screen;

#[derive(Component)]
pub struct RespawnButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct SettingsButton;
