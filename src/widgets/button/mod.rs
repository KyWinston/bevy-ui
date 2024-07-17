use bevy::prelude::*;
use bevy_lunex::{UiDebugPlugin, UiGenericPlugin, UiSystems};
use components::CustomButtonUi;
use systems::build_button;

pub mod components;
pub mod styles;
pub mod systems;

#[derive(Clone)]
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiGenericPlugin::<CustomButtonUi>::new(),
            // UiDebugPlugin::<CustomButtonUi>::new(),
        ))
        .add_systems(Update, build_button.before(UiSystems::Compute));
    }
}
