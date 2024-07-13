use bevy::prelude::*;



pub fn get_loading_text_styles(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 100.0,
        color: Color::WHITE,
        ..default()
    }
}
