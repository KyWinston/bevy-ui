use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::widgets::list::components::List;

use super::{
    components::{Panel, PanelUi},
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
            .insert((UiTreeBundle::<PanelUi>::from(UiTree::new("Panel")),))
            .with_children(|ui| {
                let panel_link = UiLink::<PanelUi>::path("Panel");
                let mut panel_bundle = ui.spawn((
                    panel_link.clone(),
                    UiLayout::window_full().pack::<Base>(),
                    Pickable::IGNORE,
                    UiColor::<Base>::new(panel.color.into()),
                ));
                if panel.texture.is_some() {
                    panel_bundle.insert((
                        ImageScaleMode::Sliced(TextureSlicer {
                            border: BorderRect::rectangle(10.0, 10.0),
                            ..default()
                        }),
                        UiImage2dBundle {
                            texture: panel.texture.as_ref().unwrap().clone(),
                            sprite: Sprite {
                                color: Color::Srgba(panel.color.into()),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                }
                ui.spawn((
                    panel_link.add("Heading"),
                    UiLayout::window().pack::<Base>(),
                    UiTextSize::new().size(Rh(13.0)),
                    UiText2dBundle {
                        text: Text::from_section(
                            panel.text.clone().unwrap(),
                            get_panel_text_styles(&assets),
                        ),
                        ..default()
                    },
                    Pickable::IGNORE,
                ));
                let mut buttons = vec![];
                for btn in &panel.content {
                    buttons.push(btn.link.to_string())
                }
                ui.spawn((
                    panel_link.add("List"),
                    UiLayout::window()
                        .size(Rl((80.0, 87.0)))
                        .pos(Rl((10.0, 13.0)))
                        .pack::<Base>(),
                    List {
                        items: buttons.to_vec(),
                        ..default()
                    },
                ));
            });
    }
}
