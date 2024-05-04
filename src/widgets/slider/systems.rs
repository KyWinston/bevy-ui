use bevy::prelude::*;
use bevy_mod_picking::events::{Drag, Pointer};

use crate::settings::resources::SettingsVals;

use super::components::{Knob, Rack};

pub fn update_value(
    mut ui_q: Query<(&Knob, &mut Style)>,
    mut rack_q: Query<&Rack>,
    mut ev: EventReader<Pointer<Drag>>,
    mut settings: ResMut<SettingsVals>,
) {
    for e in ev.read() {
        if let Ok((knob, mut style)) = ui_q.get_mut(e.target) {
            style.left =
                Val::Px((e.pointer_location.position.x + e.event.delta.x).clamp(0.0, 500.0));
            for setting in settings.0.iter_mut(){
                for _r in rack_q.iter_mut().filter(|p|{p.index_tag == knob.index_tag}){
                    setting.value = knob.value;
                }
            }
        }
    }
}
