use bevy::prelude::*;
use bevy::{
    color::palettes::css::{BLANCHED_ALMOND, OLIVE_DRAB},
    prelude::EventReader,
    sprite::Anchor,
};
use bevy_lunex::{
    prelude::{Ab, MainUi, Pickable, Rl, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, OnUiClickDespawn, PackageLayout, UiClickEvent, UiImage2dBundle,
    UiLayout, UiLink, UiText2dBundle, UiTreeBundle,
};

use crate::styles::get_title_text_styles;
use crate::{widgets::button::components::CustomButton, SimulationState};

use super::components::{Pause, PauseButton};

pub fn interact_with_pause_button(
    mut events: EventReader<UiClickEvent>,
    mut state: ResMut<NextState<SimulationState>>,
    button_q: Query<&PauseButton>,
) {
    for event in events.read() {
        if let Ok(p_btn) = button_q.get(event.target) {
            match p_btn {
                PauseButton::Resume => {
                    state.set(SimulationState::Running);
                }
                // PauseButton::Menu => {
                //     app_exit_event_writer.send(AppExit::Success);
                // }
                _ => (),
            };
        }
    }
}

pub fn build_pause(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<Pause>>,
) {
    for route_entity in &query {
        commands
            .entity(route_entity)
            .insert(SpatialBundle::default())
            .with_children(|route| {
                route
                    .spawn((
                        UiTreeBundle::<MainUi>::from(UiTree::new("Pause")),
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
                        ));

                        let panel: UiLink = root.add("Panel");
                        ui.spawn((
                            panel.clone(),
                            UiLayout::window()
                                .size(Rl((50.0, 70.0)))
                                .pos(Rl(50.0))
                                .anchor(Anchor::Center)
                                .pack::<Base>(),
                            Pickable::IGNORE,
                        ));
                        let heading: UiLink = panel.add("Heading");
                        ui.spawn((
                            heading.clone(),
                            UiLayout::window().pos(Rl((15.0, 5.0))).pack::<Base>(),
                            UiText2dBundle {
                                text: Text::from_section(
                                    "Paused",
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
                            PauseButton::Resume,
                            PauseButton::Settings,
                            PauseButton::Menu,
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
                                    texture: asset_server
                                        .load("images/ui/metalPanel_greenCorner.png"),
                                    color: OLIVE_DRAB.into(),
                                },
                            ));
                            btn.insert((Pickable::IGNORE, OnUiClickDespawn::new(route_entity)));
                            offset += gap + size;
                        }
                    });
            });
    }
}

pub fn despawn_pause(mut commands: Commands, query: Query<Entity, Added<Pause>>) {
    for ent in &query {
        commands.entity(ent).despawn_recursive();
    }
}
