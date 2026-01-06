use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod camera;
mod enemy;
mod player;
mod tower;
mod utils;
use bevy::window::PresentMode;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use tower::TowerPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((PlayerPlugin, EnemyPlugin, TowerPlugin, CameraPlugin))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .run();
}
