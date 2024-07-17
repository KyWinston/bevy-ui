use bevy::prelude::*;
use bevy_lunex::{UiDebugPlugin, UiGenericPlugin, UiSystems};
use components::ListUi;
use systems::build_list;

pub mod components;
pub mod systems;

#[derive(Clone)]
pub struct ListPlugin;

impl Plugin for ListPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiGenericPlugin::<ListUi>::new(),
            // UiDebugPlugin::<ListUi>::new(),
        ))
        .add_systems(Update, build_list.before(UiSystems::Compute));
    }
}
