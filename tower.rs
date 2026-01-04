use bevy::prelude::*;
use crate::player::Player;
use crate::enemy::Enemy;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            tower_spawn,
            tower_rotate,
            tower_shoot,
            move_bullets,
            bullet_collision_system
        ));
    }
}

#[derive(Component)]
pub struct Tower {
    pub fire_timer: Timer,
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
}



fn tower_spawn (
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
                Tower {
                    fire_timer: Timer::from_seconds(0.5, TimerMode::Repeating)
                },
            ));
        }
    }
}

fn tower_rotate (
    mut tower_query: Query<&mut Transform, (With<Tower>, Without<Enemy>)>,
    enemy_query: Query< &Transform, With<Enemy>>
) {
   for mut tower_transform in &mut tower_query {
    let tower_pos = tower_transform.translation.xy();

    let mut closest_target: Option<Vec2> = None;
    let mut min_distance_sq = f32::MAX;

    for enemy_transform in &enemy_query {
        let enemy_pos = enemy_transform.translation.xy();

        let distance_sq = tower_pos.distance_squared(enemy_pos);

        if distance_sq < min_distance_sq {
            min_distance_sq = distance_sq;
            closest_target = Some(enemy_pos);
        }
    }

    if let Some(target_pos) = closest_target {
        let direction = (target_pos - tower_pos).normalize();

        let rotation = Quat::from_rotation_arc(Vec3::Y, direction.extend(0.0));

        tower_transform.rotation = rotation;

    }
   }  

}

fn tower_shoot (
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut tower_query: Query<(&Transform, &mut Tower)>,
) {
    for (transform, mut tower) in &mut tower_query {
        tower.fire_timer.tick(time.delta());

        if tower.fire_timer.just_finished() {
            commands.spawn((
                Sprite::from_image(asset_server.load("bullet.png")),
                Transform {
                    translation: transform.translation,
                    rotation: transform.rotation,
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                Bullet {
                    speed: 500.0,
                }
            ));
        }

    }
}

fn move_bullets (
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Bullet)>,
) {
    for (mut transform, bullet) in &mut query {
        let direction = transform.rotation * Vec3::Y;

        transform.translation += direction * bullet.speed * time.delta_secs();
    }
}

fn bullet_collision_system (
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        for (enemy_entity, enemy_transform) in enemy_query {
            let distance = bullet_transform.translation.distance(enemy_transform.translation);

            if distance < 3.0 {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                break;
            }
        }
    }
}