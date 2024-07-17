use bevy::{color::palettes::css::OLIVE_DRAB, prelude::*};
use bevy_lunex::prelude::*;

use super::components::{List, ListUi};
use crate::widgets::button::components::CustomButton;

pub fn build_list(
    mut commands: Commands,
    query: Query<(Entity, &List), Added<List>>,
    assets: Res<AssetServer>,
) {
    for (entity, list) in &query {
        commands
            .entity(entity)
            .insert(UiTreeBundle::<ListUi>::from(UiTree::new("List")))
            .with_children(|ui| {
                let mut offset = 0.0;
                let list_link = UiLink::<ListUi>::path("List");
                ui.spawn((
                    list_link.clone(),
                    UiLayout::window_full().pack::<Base>(),
                    Pickable::IGNORE,
                ));
                for item in &list.items {
                    ui.spawn((
                        list_link.add("Button-".to_string() + &item),
                        UiLayout::window()
                            .y(Rl(offset))
                            .x(Rl(5.0))
                            .size((Rl(90.0), Ab(list.size)))
                            .pack::<Base>(),
                        Pickable::IGNORE,
                        CustomButton {
                            link: "Button".to_string(),
                            text: item.to_string(),
                            texture: assets.load("images/ui/metalPanel_green.png"),
                            color: OLIVE_DRAB.into(),
                        },
                    ));
                    offset += list.gap + list.size;
                }
            });
    }
}
