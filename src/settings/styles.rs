use bevy::prelude::*;

pub const SECTION_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Start;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(400.0);
    style.height = Val::Px(200.0);
    style.overflow = Overflow {
        x: OverflowAxis::Clip,
        y: OverflowAxis::Visible,
    };
    style
};

pub fn get_subtitle_text_styles(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
        ..default()
    }
}

