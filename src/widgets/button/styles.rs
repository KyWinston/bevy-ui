use bevy::prelude::*;
use bevy::{
    asset::AssetServer,
    color::{palettes::css::BLACK, Color},
    text::TextStyle,
};

pub fn get_button_text_styles(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 50.0,
        color: Color::srgb_from_array(BLACK.to_f32_array_no_alpha()),
    }
}
