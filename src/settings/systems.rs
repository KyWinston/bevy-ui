use bevy::prelude::*;

use crate::{
    settings::resources::TomlAsset,
    styles::{get_title_text_styles, CENTRAL_PANEL_STYLES, TITLE_STYLE},
    widgets::slider::components::{Knob, Rack},
    UiState,
};

use super::{
    resources::{AllSettings, SettingsVals},
    styles::get_subtitle_text_styles,
};

pub fn spawn_settings(
    commands: Commands,
    asset_server: Res<AssetServer>,
    knobs_q: Query<(Entity, &Knob)>,
    settings: Res<SettingsVals>,
) {
    build_settings(commands, asset_server, knobs_q, settings);
}

pub fn build_settings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    knobs_q: Query<(Entity, &Knob)>,
    settings: Res<SettingsVals>,
) {
    let _root_node = commands
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
            for (idx, settings) in settings.0.iter().enumerate() {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            settings.tag.to_string(),
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
                        index_tag: idx,
                        root_res: settings.tag.to_string(),
                    },
                ));
                for knob in knobs_q.iter() {
                    if knob.1.index_tag == idx {
                        rack.insert_children(0, &[knob.0]);
                    }
                }
            }
        })
        .id();
}

pub fn init_settings(
    mut commands: Commands,
    settings: Res<SettingsVals>,
    mut state: ResMut<NextState<UiState>>,
) {
    for (idx, x) in settings.0.iter().enumerate() {
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
                index_tag: idx,
                value: x.value,
            },
        ));
    }
    state.set(UiState::MainMenu);
}

pub fn load_settings_toml(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = TomlAsset(asset_server.load("settings.toml"));
    commands.insert_resource(handle);
}

pub fn assign_to_resource(
    mut commands: Commands,
    settings: Res<Assets<AllSettings>>,
    toml: Res<TomlAsset>,
) {
    if let Some(stngs) = settings.get(&toml.0) {
        let asset = &stngs.categories[0].contents;
        println!("{:?}", asset);
        commands.insert_resource(SettingsVals(asset.to_vec()));
    }
}
