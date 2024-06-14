// use bevy::prelude::*;

// use crate::{
//     components::Screen,
//     settings::resources::TomlAsset,
//     styles::{get_title_text_styles, CENTRAL_PANEL_STYLES, TITLE_STYLE},
//     // widgets::slider::components::{Knob, Rack},
// };

// use super::{
//     components::SettingsUi,
//     resources::{AllSettings, SettingsVal, SettingsVals},
//     styles::get_subtitle_text_styles,
// };

// pub fn spawn_settings(
//     commands: Commands,
//     asset_server: Res<AssetServer>,
//     knobs_q: Query<(Entity, &Knob)>,
//     settings: Res<SettingsVals>,
// ) {
//     build_settings(commands, asset_server, knobs_q, settings);
// }

// pub fn build_settings(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     knobs_q: Query<(Entity, &Knob)>,
//     settings: Res<SettingsVals>,
// ) {
//     let _root_node = commands
//         .spawn((
//             NodeBundle {
//                 style: CENTRAL_PANEL_STYLES,
//                 ..default()
//             },
//             SettingsUi,
//             Screen,
//         ))
//         .with_children(|parent| {
//             parent.spawn(NodeBundle {
//                 style: TITLE_STYLE,
//                 ..default()
//             });
//             parent.spawn(TextBundle {
//                 text: Text {
//                     sections: vec![TextSection::new(
//                         "Settings".to_string(),
//                         get_title_text_styles(&asset_server),
//                     )],
//                     ..default()
//                 },
//                 ..default()
//             });
//             for (idx, settings) in settings.0.iter().enumerate() {
//                 parent.spawn(TextBundle {
//                     text: Text {
//                         sections: vec![TextSection::new(
//                             settings.tag.to_string(),
//                             get_subtitle_text_styles(&asset_server),
//                         )],
//                         ..default()
//                     },
//                     ..default()
//                 });
//                 let mut rack = parent.spawn((
//                     NodeBundle {
//                         style: Style {
//                             width: Val::Percent(80.0),
//                             height: Val::Px(30.0),
//                             margin: UiRect::all(Val::Px(40.0)),
//                             ..default()
//                         },

//                         background_color: BackgroundColor(Color::rgb(0.4, 0.4, 0.4)),
//                         ..default()
//                     },
//                     Rack {
//                         index_tag: idx,
//                         root_res: settings.tag.to_string(),
//                     },
//                 ));
//                 for knob in knobs_q.iter() {
//                     if knob.1.index_tag == idx {
//                         rack.insert_children(0, &[knob.0]);
//                     }
//                 }
//             }
//         })
//         .id();
// }

// pub fn init_settings(mut commands: Commands, settings: Res<SettingsVals>) {
//     for (idx, x) in settings.0.iter().enumerate() {
//         commands.spawn((
//             NodeBundle {
//                 style: Style {
//                     width: Val::Px(30.0),
//                     height: Val::Px(30.0),
//                     border: UiRect::all(Val::Px(10.0)),
//                     ..default()
//                 },

//                 background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
//                 ..default()
//             },
//             Knob {
//                 index_tag: idx,
//                 value: x.value,
//             },
//         ));
//     }
// }

// pub fn load_settings_toml(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let handle = TomlAsset(asset_server.load("settings.toml"));
//     commands.insert_resource(handle);
// }

// pub fn assign_to_resource(
//     mut commands: Commands,
//     settings: Res<Assets<AllSettings>>,
//     toml: Res<TomlAsset>,
// ) {
//     if let Some(stngs) = settings.get(&toml.0) {
//         let mut new_settings:Vec<SettingsVal> = vec![];
//         for cat in &stngs.categories {
//             for content in cat.contents.as_slice(){
//                 new_settings.push(content.clone());
//             }
//         }
//         commands.insert_resource(SettingsVals(new_settings.to_vec()));

//     }
// }
