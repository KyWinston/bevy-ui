use bevy::{prelude::*, transform};
use bevy_mod_picking::{
    events::{Drag, Move, Pointer},
    prelude::On,
};

use crate::{
    styles::{get_title_text_styles, CENTRAL_PANEL_STYLES, TITLE_STYLE},
    widgets::slider::{
        components::{Knob, Rack},
        systems::update_value,
    },
};

use super::{
    events::ValueUpdateEvent,
    resources::{SettingsVal, SettingsVals},
    styles::get_subtitle_text_styles,
};

pub fn spawn_settings(
    commands: Commands,
    asset_server: Res<AssetServer>,
    knobs_q: Query<(Entity, &Knob)>,
) {
    build_settings(commands, asset_server, knobs_q);
}

pub fn build_settings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    knobs_q: Query<(Entity, &Knob)>,
) {
    let root_node = commands
        .spawn(NodeBundle {
            style: CENTRAL_PANEL_STYLES,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: TITLE_STYLE,
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Settings".to_string(),
                        get_title_text_styles(&asset_server),
                    )],
                    ..default()
                },
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Mouse Sensitivity".to_string(),
                        get_subtitle_text_styles(&asset_server),
                    )],
                    ..default()
                },
                ..default()
            });
            let mut rack = parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Px(30.0),
                        margin: UiRect::all(Val::Px(40.0)),
                        ..default()
                    },

                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                },
                Rack {
                    index_tag: 0,
                    root_res: "mouse_sensitivity".to_string(),
                },
            ));
            for knob in knobs_q.iter() {
                rack.insert_children(0, &[knob.0]);
            }
        })
        .id();
}

pub fn insert_setting(mut commands: Commands, mut _settings: ResMut<SettingsVals>) {
    let val = SettingsVal {
        tag: "mouse_sensitivity".to_string(),
        value: 0,
    };
    commands.insert_resource(SettingsVals(vec![val]));
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                border: UiRect::all(Val::Px(10.0)),
                ..default()
            },

            background_color: BackgroundColor(Color::GRAY),
            ..default()
        },
        Knob {
            index_tag: 0,
            value: 0,
        },
    ));
}
