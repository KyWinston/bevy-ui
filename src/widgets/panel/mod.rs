use bevy::prelude::*;
use bevy_lunex::{UiDebugPlugin, UiGenericPlugin, UiSystems};
use components::PanelUi;
use systems::build_panel;

pub mod components;
pub mod styles;
pub mod systems;

#[derive(Clone)]
pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiGenericPlugin::<PanelUi>::new(),
            // UiDebugPlugin::<PanelUi>::new(),
        ))
        .add_systems(Update, build_panel.before(UiSystems::Compute));
    }
}
