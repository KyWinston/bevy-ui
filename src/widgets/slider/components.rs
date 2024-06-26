use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Knob {
    pub index_tag: usize,
    pub value: u32,
}

#[derive(Component)]
pub struct Rack {
    pub root_res: String,
    pub index_tag: usize,
}
