use bevy::{color::palettes::css::BLACK, prelude::*};
use bevy_lunex::{
    prelude::{MainUi, Pickable, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, PackageLayout, UiColor, UiLayout, UiLink, UiText2dBundle, UiTreeBundle,
};

use super::{components::Loading, styles::get_loading_text_styles};

pub fn build_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<Loading>>,
) {
    for route_entity in &query {
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
                        let loading_text = "Loading...".to_string();
                        let root = UiLink::<MainUi>::path("Root"); // Here we can define the name of the node
                        ui.spawn((
                            root.clone(),
                            Loading(Some(loading_text.clone())),
                            UiLayout::window_full().pack::<Base>(),
                        ));
                        ui.spawn((
                            root.add("Background"),
                            UiLayout::solid()
                                .size((2968.0, 1656.0))
                                .scaling(bevy_lunex::prelude::Scaling::Fill)
                                .pack::<Base>(),
                            UiColor::<Base>::new(Color::Srgba(BLACK.into())),
                            Pickable::IGNORE,
                            UiText2dBundle {
                                text: Text::from_section(
                                    loading_text,
                                    get_loading_text_styles(&asset_server),
                                ),
                                ..default()
                            },
                        ));
                    });
            });
    }
}

pub fn despawn_loading(mut commands: Commands, loading: Query<Entity, With<Loading>>) {
    for entity in &loading {
        commands.entity(entity).despawn_recursive();
    }
}
