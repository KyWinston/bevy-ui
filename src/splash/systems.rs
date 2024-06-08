use bevy::prelude::*;

use crate::{components::Screen, UiState};

use super::components::{SplashScreen, SplashTimer};

pub fn spawn_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SplashTimer(Timer::from_seconds(5.0, TimerMode::Once)));
    build_splash(commands, asset_server);
}

pub fn build_splash(mut commands: Commands, asset_server: Res<AssetServer>) -> Entity {
    let splash = commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                ..default()
            },
            SplashScreen,
            Screen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(asset_server.load("branding/icon.png")),
                ..default()
            });
        })
        .id();
    splash
}

pub fn count_down(
    mut state: ResMut<NextState<UiState>>,
    mut timer: ResMut<SplashTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.finished() {
        state.set(UiState::MainMenu);
    }
}
