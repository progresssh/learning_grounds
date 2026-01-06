use crate::player::Player;
use bevy::prelude::*;
use rand::prelude::*;
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
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut spawner: Local<EnemySpawner>,
    player: Option<Single<&Transform, With<Player>>>,
) {
    if let Some(player_transform) = player {

        let player_pos = player_transform.translation.xy();
        let safe_radius = 600.0;
        let spawn_radius = 12000.0;
        let min_sq = safe_radius * safe_radius;
        let max_sq = spawn_radius * spawn_radius;
        let area = std::f32::consts::PI * 2.0;
        
        let mut rng = rand::rng();

        let angle = rng.random_range(0.0..area);
        let rand_value: f32 = rng.random_range(min_sq..max_sq);
        let distance: f32= rand_value.sqrt();

        let offset_x = angle.cos() * distance;
        let offset_y = angle.sin() * distance;

        let spawn_pos = player_pos + Vec2::new(offset_x, offset_y);


        spawner.timer.tick(time.delta());
        if spawner.timer.is_finished() {
            commands.spawn((
                Sprite {
                    image: asset_server.load("enemy.png"),
                    ..default()
                },
                Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.0),
                Enemy,
            ));
        }
    }
}

fn move_enemy(time: Res<Time>, mut enemies: Query<&mut Transform, With<Enemy>>) {
    for mut transform in &mut enemies {
        transform.translation.x += (time.delta_secs() * 10.0);
        transform.translation.y += ((transform.translation.x + PI * time.delta_secs()).sin()
            * 100.0)
            * (5.0 * time.delta_secs());
    }
}
