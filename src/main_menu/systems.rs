use bevy::{color::palettes::css::BLACK, prelude::*, window::PrimaryWindow};

use bevy_lunex::{
    prelude::{MainUi, Pickable, Rl, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, PackageLayout, UiClickEvent, UiImage2dBundle, UiLayout, UiLink,
    UiTreeBundle,
};

use crate::{
    loading::components::Loading,
    main_menu::components::MainMenuButton,
    resources::GameTitle,
    widgets::{
        button::components::{CustomButton, CustomButtonRef},
        panel::components::Panel,
    },
};

use super::components::MainMenu;

pub fn build_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<MainMenu>>,
    window: Query<&Window, With<PrimaryWindow>>,
    title: Res<GameTitle>,
) {
    for route_entity in &query {
        if let Ok(resolution) = window.get_single() {
            let r_size = (resolution.width(), resolution.height());
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
                            let root = UiLink::<MainUi>::path("Root");
                            ui.spawn((
                                root.clone(),
                                UiLayout::window().size(r_size).pack::<Base>(),
                            ));
                            let background = root.add("Background");

                            ui.spawn((
                                background.clone(),
                                UiLayout::window_full().pack::<Base>(),
                                Pickable::IGNORE,
                                UiImage2dBundle::from(asset_server.load("Level_base_diffuse.png")),
                            ));
                            let content = vec![
                                CustomButtonRef {
                                    link: MainMenuButton::NewGame.str(),
                                },
                                CustomButtonRef {
                                    link: MainMenuButton::Settings.str(),
                                },
                                CustomButtonRef {
                                    link: MainMenuButton::QuitGame.str(),
                                },
                            ];
                            ui.spawn((
                                background.add("Panel"),
                                UiLayout::window()
                                    .size(Rl((40.0, 80.0)))
                                    .pos(Rl((10.0, 10.0)))
                                    .pack::<Base>(),
                                Panel {
                                    text: Some(title.0.to_string()),
                                    color: BLACK.into(),
                                    content,
                                    ..default()
                                },
                                Pickable::IGNORE,
                            ));
                        });
                });
        }
    }
}

pub fn main_menu_button_clicked_system(
    mut commands: Commands,
    mut events: EventReader<UiClickEvent>,
    query: Query<&CustomButton>,
    menu_q: Query<Entity, With<MainMenu>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for event in events.read() {
        if let Ok(ent) = menu_q.get_single() {
            if let Ok(button) = query.get(event.target) {
                if button.text == MainMenuButton::NewGame.str() {
                    commands.entity(ent).despawn_recursive();
                    commands.spawn(Loading(Some("Loading...".to_string())));
                } else if button.text == MainMenuButton::QuitGame.str() {
                    commands.entity(event.target).despawn_recursive();
                    exit.send(bevy::app::AppExit::Success);
                }
            }
        }
    }
}
