use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct CustomButtonUi;

#[derive(Component)]
pub struct CustomButton {
    pub link: String,
    pub text: String,
    pub texture: Handle<Image>,
    pub color: Color,
}
#[derive(Component, Clone)]
pub struct CustomButtonRef {
    pub link: String,
    // pub texture: Handle<Image>,
    // pub color: Color,
}
