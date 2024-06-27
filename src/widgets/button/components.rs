use bevy::prelude::*;

#[derive(Component)]
pub struct CustomButtonUi;

#[derive(Component)]
pub struct CustomButton {
    pub text: String,
    pub texture: Handle<Image>,
    pub color: Color
}


