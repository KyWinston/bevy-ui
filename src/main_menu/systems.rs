use bevy::{
    color::palettes::css::{BLANCHED_ALMOND, OLIVE_DRAB},
    prelude::*,
    sprite::Anchor,
};
use bevy_lunex::{
    prelude::{Ab, MainUi, Pickable, Rl, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, OnUiClickCommands, OnUiClickDespawn, PackageLayout, UiImage2dBundle,
    UiLayout, UiLink, UiText2dBundle, UiTreeBundle,
};

use crate::{
    components::Quit, loading::components::Loading, resources::GameTitle,
    styles::*, widgets::button::components::CustomButton,
};

use super::components::{MainMenu, MainMenuButton};

pub fn build_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<MainMenu>>,
    title: Res<GameTitle>,
) {
    for route_entity in &query {
        commands
            .entity(route_entity)
            .insert(SpatialBundle::default())
            .with_children(|route| {
                route
                    .spawn((
                        UiTreeBundle::<MainUi>::from(UiTree::new("MainMenu")),
                        MovableByCamera,
                    ))
                    .with_children(|ui| {
                        let root = UiLink::<MainUi>::path("Root"); // Here we can define the name of the node
                        ui.spawn((root.clone(), UiLayout::window_full().pack::<Base>()));
                        ui.spawn((
                            root.add("Background"),
                            UiLayout::solid()
                                .size((2968.0, 1656.0))
                                .scaling(bevy_lunex::prelude::Scaling::Fill)
                                .pack::<Base>(),
                            Pickable::IGNORE,
                            UiImage2dBundle::from(asset_server.load("Level_base_diffuse.png")),
                        ));

                        let panel: UiLink = root.add("Panel");
                        ui.spawn((
                            panel.clone(),
                            UiLayout::window()
                                .size(Rl((55.0, 70.0)))
                                .pos(Rl((40.0, 50.0)))
                                .anchor(Anchor::Center)
                                .pack::<Base>(),
                            Pickable::IGNORE,
                        ));
                        let heading: UiLink = panel.add("Heading");
                        ui.spawn((
                            heading.clone(),
                            UiLayout::window().pos(Rl(5.0)).pack::<Base>(),
                            UiText2dBundle {
                                text: Text::from_section(
                                    title.0.as_str(),
                                    get_title_text_styles(&asset_server),
                                ),
                                text_anchor: Anchor::Center,
                                ..default()
                            },
                            Pickable::IGNORE,
                        ));

                        let list = panel.add("List");
                        ui.spawn((
                            list.clone(),
                            UiLayout::window()
                                .pos((Rl(20.0), Rl(40.0)))
                                .size(Rl(60.0))
                                .pack::<Base>(),
                            Pickable::IGNORE,
                        ));

                        // Spawn buttons
                        let gap = 2.0;
                        let size = 40.0;
                        let mut offset = 0.0;
                        for button in [
                            // MainMenuButton::Continue,
                            MainMenuButton::NewGame,
                            // MainMenuButton::LoadGame,
                            MainMenuButton::Settings,
                            // MainMenuButton::AdditionalContent,
                            // MainMenuButton::Credits,
                            MainMenuButton::QuitGame,
                        ] {
                            let mut btn = ui.spawn((
                                list.add(button.str()),
                                button.clone(),
                                UiImage2dBundle {
                                    sprite: Sprite {
                                        color: Color::srgb_from_array(
                                            BLANCHED_ALMOND.to_f32_array_no_alpha(),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                },
                                UiLayout::window()
                                    .y(Rl(offset))
                                    .size((Rl(100.0), Ab(size)))
                                    .pack::<Base>(),
                                CustomButton {
                                    text: button.str(),
                                    texture: asset_server.load("images/ui/metalPanel.png"),
                                    color: OLIVE_DRAB.into(),
                                },
                            ));
                            if button == MainMenuButton::NewGame {
                                btn.insert((
                                    OnUiClickDespawn::new(route_entity),
                                    OnUiClickCommands::new(|commands| {
                                        commands.spawn(Loading(Some("Loading...".to_string())));
                                    }),
                                ));
                            } else if button == MainMenuButton::QuitGame {
                                btn.insert((
                                    OnUiClickDespawn::new(route_entity),
                                    OnUiClickCommands::new(|commands| {
                                        commands.spawn(Quit);
                                    }),
                                ));
                            }
                            offset += gap + size;
                        }
                    });
            });
    }
}
