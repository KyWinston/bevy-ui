use bevy::prelude::*;

#[derive(Bundle)]
pub struct CardWidget {
    pub base: NodeBundle,
}

impl CardWidget {
    pub fn new(width: f32, height: f32, background_color: BackgroundColor) -> Self {
        Self {
            base: NodeBundle {
                style: Style {
                    width: Val::Percent(width),
                    height: Val::Percent(height),
                    justify_content: JustifyContent::Center,

                    ..default()
                },

                background_color,
                ..default()
            },
        }
    }
}
