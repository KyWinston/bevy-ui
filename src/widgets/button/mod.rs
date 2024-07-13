use bevy::prelude::*;
use bevy_lunex::{UiGenericPlugin, UiSystems};
use components::CustomButtonUi;
use systems::build_button;

pub mod components;
pub mod systems;
pub mod styles;

#[derive(Clone)]
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiGenericPlugin::<CustomButtonUi>::new())
            .add_systems(Update, build_button.before(UiSystems::Compute));
    }
}
