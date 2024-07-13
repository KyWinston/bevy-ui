use bevy::{color::palettes::css::BLACK, prelude::*, sprite::Anchor};
use bevy_lunex::{
    prelude::{MainUi, Pickable, Rl, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, PackageLayout, UiColor, UiLayout, UiLink, UiText2dBundle, UiTreeBundle,
};

use super::{components::Loading, styles::get_loading_text_styles};

pub fn build_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Loading), Added<Loading>>,
) {
    for (route_entity, load_text) in &query {
        commands
            .entity(route_entity)
            .insert(SpatialBundle::default())
            .with_children(|route| {
                route
                    .spawn((
                        UiTreeBundle::<MainUi>::from(UiTree::new("Loading")),
                        MovableByCamera,
                    ))
                    .with_children(|ui| {
                        let root = UiLink::<MainUi>::path("Root"); // Here we can define the name of the node
                        ui.spawn((
                            root.clone(),
                            UiLayout::solid()
                                .size((2968.0, 1656.0))
                                .scaling(bevy_lunex::prelude::Scaling::Fill)
                                .pack::<Base>(),
                        ));
                        ui.spawn((
                            root.add("Background"),
                            UiLayout::window()
                                .size(Rl((100.0, 10.0)))
                                .pos(Rl((80.0, 90.0)))
                                .pack::<Base>(),
                            UiColor::<Base>::new(Color::Srgba(BLACK.into())),
                            Pickable::IGNORE,
                        ));
                        ui.spawn((
                            root.add("Background/Messege"),
                            UiLayout::window().pack::<Base>(),
                            UiText2dBundle {
                                text: Text::from_section(
                                    load_text.0.as_ref().unwrap().to_string(),
                                    get_loading_text_styles(&asset_server),
                                ),
                                text_anchor: Anchor::BottomRight,
                                ..default()
                            },
                        ));
                    });
            });
    }
}
