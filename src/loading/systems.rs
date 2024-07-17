use bevy::{color::palettes::css::BLACK, prelude::*, window::PrimaryWindow};
use bevy_lunex::{
    prelude::{Ab, MainUi, Pickable, Rh, Rl, Scaling, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, PackageLayout, UiColor, UiLayout, UiLink, UiText2dBundle, UiTextSize,
    UiTreeBundle,
};

use crate::UiState;

use super::{components::Loading, styles::get_loading_text_styles};

pub fn build_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut load_state: ResMut<NextState<UiState>>,
    window: Query<&Window, With<PrimaryWindow>>,
    query: Query<(Entity, &Loading), Added<Loading>>,
) {
    for (route_entity, load_text) in &query {
        load_state.set(UiState::Loading);

        if let Ok(resolution) = window.get_single() {
            let r_size = (resolution.width(), resolution.height());
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
                            let root = UiLink::<MainUi>::path("Root");
                            ui.spawn((
                                root.clone(),
                                UiLayout::solid()
                                    .size(r_size)
                                    .scaling(Scaling::Fit)
                                    .pack::<Base>(),
                            ));
                            ui.spawn((
                                root.add("Load_text"),
                                UiLayout::window()
                                    .pos((Rl(80.0) - Ab(60.0), Rl(90.0) - Ab(20.0)))
                                    .pack::<Base>(),
                                UiColor::<Base>::new(Color::Srgba(BLACK.into())),
                                Pickable::IGNORE,
                                UiTextSize::new().size(Rh(10.0)),
                                UiText2dBundle {
                                    text: Text::from_section(
                                        load_text.0.as_ref().unwrap().to_string(),
                                        get_loading_text_styles(&asset_server),
                                    ),
                                    ..default()
                                },
                            ));
                        });
                });
        }
    }
}
