use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTitle(pub String);


#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct IconCache(pub Vec<Handle<Image>>);
