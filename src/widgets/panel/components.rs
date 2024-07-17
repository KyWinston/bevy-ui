use bevy::{color::palettes::css::WHITE, prelude::*, sprite::Anchor};

use crate::widgets::button::components::CustomButtonRef;

#[derive(Component, Clone)]
pub struct PanelUi;

#[derive(Component)]
pub struct PanelText;

#[derive(Component)]
pub struct Panel {
    // pub label: String,
    pub text: Option<String>,
    pub texture: Option<Handle<Image>>,
    pub color: Color,
    pub text_alignment: Anchor,
    pub content: Vec<CustomButtonRef>,
}

impl Default for Panel {
    fn default() -> Self {
        Self {
            text: None,
            texture: None,
            color: WHITE.into(),
            text_alignment: Anchor::TopCenter,
            content: vec![],
        }
    }
}
