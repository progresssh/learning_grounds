use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod camera;
mod enemy;
mod player;
mod tower;
mod utils;
mod world;
mod sound;

use bevy::window::PresentMode;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use tower::TowerPlugin;
use world::WorldPlugin;
use sound::SoundPlugin;

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
        .add_plugins(LdtkPlugin)
        .add_plugins((PlayerPlugin, EnemyPlugin, TowerPlugin, CameraPlugin, WorldPlugin, SoundPlugin))
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        .run();
}
