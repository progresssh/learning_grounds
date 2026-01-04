use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct Tower;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_spawn)
            .add_systems(Update, tower_shoot);
    }
}

fn tower_spawn(
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

fn tower_shoot (
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Player>>,
) {

}