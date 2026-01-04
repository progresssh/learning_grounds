use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, move_player);
    }
}

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

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * speed * time.delta_secs();
    }
}