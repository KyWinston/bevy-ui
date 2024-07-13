use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;

use super::{
    components::{Panel, PanelText, PanelUi},
    events::PanelUpdateEvent,
    styles::get_panel_text_styles,
};

pub fn build_panel(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<(Entity, &Panel), Added<Panel>>,
) {
    for (entity, panel) in &query {
        commands
            .entity(entity)
            .insert(UiTreeBundle::<PanelUi>::from(UiTree::new("Panel")))
            .with_children(|ui| {
                ui.spawn((
                    UiLink::<PanelUi>::path("Panel"),
                    UiImage2dBundle {
                        texture: panel.texture.clone(),
                        sprite: Sprite {
                            color: Color::Srgba(panel.color.into()),
                            ..default()
                        },
                        ..default()
                    },
                    UiLayout::window_full()
                        .anchor(Anchor::Center)
                        .pack::<Base>(),
                    ImageScaleMode::Sliced(TextureSlicer {
                        border: BorderRect::rectangle(10.0, 10.0),
                        ..default()
                    }),
                    UiColor::<Base>::new(panel.color.into()),
                    Pickable::IGNORE,
                ));
                ui.spawn((
                    UiLink::<PanelUi>::path("Panel/PanelText"),
                    UiLayout::window_full()
                        .anchor(Anchor::Center)
                        .pos(Rl(50.0))
                        .pack::<Base>(),
                    UiText2dBundle {
                        text: Text::from_section(
                            panel.text.to_string(),
                            get_panel_text_styles(&assets),
                        ),
                        ..default()
                    },
                    PanelText,
                    Pickable::IGNORE,
                ));
            });
    }
}

pub fn update_panel(
    mut panel_ev: EventReader<PanelUpdateEvent>,
    mut panel_q: Query<(&Children, &Panel), With<Panel>>,
    mut p_text: Query<&mut Text, With<PanelText>>,
) {
    for ev in panel_ev.read() {
        for (children, panel) in panel_q.iter_mut() {
            for child in children.iter() {
                if let Ok(mut ptext) = p_text.get_mut(*child) {
                    if panel.label == ev.0 {
                        ptext.sections[0].value = ev.1.clone();
                    }
                }
            }
        }
    }
}
