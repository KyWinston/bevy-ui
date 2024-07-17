use bevy::prelude::Component;

#[derive(Component)]
pub struct Pause;

#[derive(Component)]
pub struct ResumeButton;

// #=====================#
// #=== INTERACTIVITY ===#

/// Good practice is to use custom component for buttons, so we can easily know what type of button was pressed
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub enum PauseButton {
    #[default]
    Resume,
    Settings,
    Menu,
}
impl PauseButton {
    pub fn str(&self) -> String {
        match self {
            PauseButton::Resume => "Resume".into(),
            PauseButton::Settings => "Settings".into(),
            PauseButton::Menu => "Return to menu".into(),
        }
    }
}
