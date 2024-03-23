use bevy::prelude::*;

use crate::{components::Screen, AppState};

use super::components::{SplashScreen, SplashTimer};

pub fn build_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/icon.png");
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            SplashScreen,
            Screen
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(4.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
pub fn countdown(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppState::MainMenu);
    }
}
