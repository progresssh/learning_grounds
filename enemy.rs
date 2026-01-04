use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy)
            .add_systems(FixedUpdate, move_enemy);
    }
}

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