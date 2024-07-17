use bevy::prelude::*;

#[derive(Component)]
pub struct Loading(pub Option<String>);