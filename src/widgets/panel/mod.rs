use bevy::prelude::*;
use bevy_lunex::{UiGenericPlugin, UiSystems};
use components::PanelUi;
use events::PanelUpdateEvent;
use systems::{build_panel, update_panel};

pub mod components;
pub mod events;
pub mod styles;
pub mod systems;

#[derive(Clone)]
pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PanelUpdateEvent>()
            .add_plugins(UiGenericPlugin::<PanelUi>::new())
            .add_systems(
                Update,
                (update_panel, build_panel.before(UiSystems::Compute)),
            );
    }
}
