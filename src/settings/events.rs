use bevy::ecs::event::Event;

#[derive(Event)]
pub struct ValueUpdateEvent(pub u32);
