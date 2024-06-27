use bevy::{
    color::palettes::{
        css::{BLACK, STEEL_BLUE},
        tailwind::{BLUE_100, BLUE_200},
    },
    prelude::*,
};

pub const NORMAL_BUTTON_COLOR: Srgba = STEEL_BLUE;
pub const HOVERED_BUTTON_COLOR: Srgba = BLUE_100;
pub const PRESSED_BUTTON_COLOR: Srgba = BLUE_200;

pub fn get_title_text_styles(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.0,
        color: Color::srgb_from_array(BLACK.to_f32_array_no_alpha()),
    }
}

pub fn get_button_text_styles(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 50.0,
        color: Color::srgb_from_array(BLACK.to_f32_array_no_alpha()),
    }
}
