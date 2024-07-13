use bevy::prelude::*;
use bevy_lunex::UiSystems;
use systems::build_loading;

pub mod components;
mod styles;
mod systems;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, build_loading.before(UiSystems::Compute));
    }
}
