use bevy::prelude::Component;

#[derive(Component)]
pub struct MainMenu;

// #=====================#
// #=== INTERACTIVITY ===#

/// Good practice is to use custom component for buttons, so we can easily know what type of button was pressed
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub enum MainMenuButton {
    Continue,
    #[default]
    NewGame,
    LoadGame,
    Settings,
    AdditionalContent,
    Credits,
    QuitGame,
}
impl MainMenuButton {
    pub fn str(&self) -> String {
        match self {
            MainMenuButton::Continue => "CONTINUE".into(),
            MainMenuButton::NewGame => "New Game".into(),
            MainMenuButton::LoadGame => "LOAD GAME".into(),
            MainMenuButton::Settings => "Settings".into(),
            MainMenuButton::AdditionalContent => "ADDITIONAL CONTENT".into(),
            MainMenuButton::Credits => "CREDITS".into(),
            MainMenuButton::QuitGame => "Quit".into(),
        }
    }
}
