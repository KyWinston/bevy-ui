use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct ListUi;

#[derive(Component)]
pub struct List {
    pub items: Vec<String>,
    pub gap: f32,
    pub size: f32,
}

impl Default for List {
    fn default() -> Self {
        Self {
            items: vec![],
            gap: 2.0,
            size: 30.0,
        }
    }
}
