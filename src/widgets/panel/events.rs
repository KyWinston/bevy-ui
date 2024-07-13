use bevy::prelude::Event;


#[derive(Event)]
pub struct PanelUpdateEvent(pub String, pub String);
