use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct SettingsVals(pub Vec<SettingsVal>);

pub struct SettingsVal {
    pub tag: String,
    pub value: u32,
}
