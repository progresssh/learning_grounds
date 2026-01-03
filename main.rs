use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_player)
        .add_systems(Update, move_player)
        .add_systems(Update, build_tower)
        .add_systems(Update, spawn_enemy)
        .add_systems(Update, move_enemy)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Player;

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
 ) {
    commands.spawn((
        Sprite {
            image: asset_server.load("player.png"),
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        },
        Player,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 200.0;
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * speed * time.delta_secs();
    }
}

#[derive(Component)]
struct Tower;

fn build_tower(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(player_transform) = query.single() {
            let player_pos = player_transform.translation;

            commands.spawn((
                Sprite {
                    image: asset_server.load("tower.png"),
                    custom_size: Some(Vec2::new(48.0, 48.0)),
                    ..default()
                },
                Transform::from_translation(player_pos),
                Tower,
            ));
        }
    }
}

fn defense_tower (
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Player>>,
) {

}

#[derive(Component)]
struct Enemy;

struct EnemySpawner {
    timer: Timer,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut spawner: Local<EnemySpawner>, 
) {
    spawner.timer.tick(time.delta());
    if spawner.timer.is_finished() {
        commands.spawn((
            Sprite {
                image: asset_server.load("enemy.png"),
                ..default()
            },
            Transform::from_xyz(-700.0, 0.0, 0.0),
            Enemy,
        ));
    }
}

fn move_enemy (
    time: Res<Time>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
) {
    for mut transform in &mut enemies {
        transform.translation.x += (time.delta_secs() * 10.0);
        transform.translation.y += ((transform.translation.x + PI * time.delta_secs()).sin() * 100.0) * (5.0 * time.delta_secs());
    } 
}