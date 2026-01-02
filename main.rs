use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_player)
        .add_systems(Update, move_player)
        .add_systems(Update, build_tower)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Player;

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
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

fn build_tower (
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(player_transform) = query.single() {
            let player_pos = player_transform.translation;
            
            commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..default()
                },
                Transform::from_translation(player_pos),
                Tower,
            ));
        }
    }
}