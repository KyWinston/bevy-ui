use bevy::{color::palettes::css::BLACK, prelude::*};

pub const TITLE_FONT: f32 = 15.0;
pub const BASE_FONT: f32 = 10.0;

pub fn get_title_text_styles(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: TITLE_FONT,
        color: BLACK.into(),
    }
}
