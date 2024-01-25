use bevy::prelude::*;

pub const LOADING_SCREEN_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::End;
    style.align_items = AlignItems::End;
    style
};

pub const BASIC_BLOCK_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.margin = UiRect::all(Val::Px(20.0));
    style
};

pub fn get_loading_text_styles(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::WHITE,
        ..default()
    }
}
