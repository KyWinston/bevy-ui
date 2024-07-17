use bevy::prelude::*;
use bevy_lunex::UiSystems;
use components::Loading;
use systems::build_loading;

use crate::UiState;

pub mod components;
mod styles;
mod systems;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, build_loading.before(UiSystems::Compute));
        app.add_systems(
            OnExit(UiState::Loading),
            |mut commands: Commands, load_q: Query<Entity, With<Loading>>| {
                if let Ok(load) = load_q.get_single() {
                    commands.entity(load).despawn_recursive();
                }
            },
        );
    }
}
