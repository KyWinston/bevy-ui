use bevy::prelude::*;
use bevy_lunex::{
    prelude::{MainUi, Rl, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, PackageLayout, UiImage2dBundle, UiLayout, UiLink, UiTreeBundle,
};

use crate::{main_menu::components::MainMenu, UiState};

use super::components::{SplashScreen, SplashTimer};

pub fn build_splash(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, Added<SplashScreen>>,
) {
    for route_entity in &query {
        // Spawn the route
        commands
            .entity(route_entity)
            .insert(SpatialBundle::default())
            .with_children(|route| {
                // Spawn the master ui tree
                route
                    .spawn((
                        UiTreeBundle::<MainUi>::from(UiTree::new("MainMenu")),
                        MovableByCamera,
                    ))
                    .with_children(|ui| {
                        let root: UiLink<_> = UiLink::<MainUi>::path("Root"); // Here we can define the name of the node
                        ui.spawn((
                            root.clone(),                           // Here we add the link
                            UiLayout::window_full().pack::<Base>(), // This is where we define layout
                        ));

                        // Spawn the background
                        ui.spawn((
                            root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
                            UiLayout::boundary()
                                .pos1(Rl(40.0))
                                .pos2(Rl(60.0))
                                .pack::<Base>(),
                            UiImage2dBundle::from(assets.load("branding/icon.png")), // We use this bundle to add background image to our node
                        ));
                    });
            });
    }
}
pub fn count_down(
    mut commands: Commands,
    mut state: ResMut<NextState<UiState>>,
    mut timer: ResMut<SplashTimer>,
    splash: Query<Entity, With<SplashScreen>>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.finished() {
        state.set(UiState::MainMenu);
        for route_entity in &splash {
            println!("despawn splash");
            commands.entity(route_entity).despawn_recursive();
        }
        commands.spawn(MainMenu);
    }
}
