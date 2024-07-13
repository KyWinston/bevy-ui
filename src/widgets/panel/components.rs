use bevy::prelude::*;

#[derive(Component)]
pub struct PanelUi;

#[derive(Component)]
pub struct PanelText;

#[derive(Component)]
pub struct Panel {
    pub label: String,
    pub text: String,
    pub texture: Handle<Image>,
    pub color: Color,
}
