use bevy::{
    color::palettes::{
        css::{STEEL_BLUE, WHITE},
        tailwind::{BLUE_100, BLUE_200},
    },
    prelude::*,
};

pub const NORMAL_BUTTON_COLOR: Srgba = STEEL_BLUE;
pub const HOVERED_BUTTON_COLOR: Srgba = BLUE_100;
pub const PRESSED_BUTTON_COLOR: Srgba = BLUE_200;

pub const CENTRAL_PANEL_STYLES: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const BASIC_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style
};

pub const IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(32.0);
    style.height = Val::Px(32.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(600.0);
    style.height = Val::Px(120.0);
    style
};

pub fn get_title_text_styles(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::srgb_from_array(WHITE.to_f32_array_no_alpha()),
    }
}

pub fn get_button_text_styles(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::srgb_from_array(WHITE.to_f32_array_no_alpha()),
    }
}
