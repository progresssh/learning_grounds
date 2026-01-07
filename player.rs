use bevy::prelude::*;
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, *};

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component, Clone, Copy, PartialEq)]
pub enum FacingDirection {
    #[default]
    Right,
    Left,
    Up,
    Down
}

#[derive(Default, Component)]
pub struct IsMoving(pub bool);

#[derive(Default, Component)]
struct AnimationConfig {
    first_index: usize,
    last_index: usize,
    y_index: usize,
    timer: Timer,
}

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[with(player_animation)]
    animation: AnimationConfig,
    facing: FacingDirection,
    is_moving: IsMoving,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            //.add_systems(Startup, setup_player)
            .add_systems(Update, move_player)
            .add_systems(Update, animate_sprites);
    }
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut FacingDirection, &mut IsMoving), With<Player>>,
) {
    let speed = 60.0;
    for (mut transform, mut facing, mut is_moving) in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
            *facing = FacingDirection::Up;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
            *facing = FacingDirection::Down;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            *facing = FacingDirection::Left;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            *facing = FacingDirection::Right;
        }

        is_moving.0 = direction.length_squared() > 0.0;

        if is_moving.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * speed * time.delta_secs();
    }
}

fn player_animation(_: &EntityInstance) -> AnimationConfig {
    AnimationConfig {
        first_index: 0,
        last_index: 3,
        y_index: 0,
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
    }
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite, &FacingDirection, &IsMoving)>,
) {
    const COLUMNS: usize = 4;
    const ROW_RIGHT: usize = 2; // Third row (0-indexed)
    const ROW_LEFT: usize = 3;  // Fourth row (0-indexed)
    const IDLE_RIGHT: usize = 4;
    const IDLE_LEFT: usize = 5;
    const IDLE_UP: usize = 1;
    const IDLE_DOWN: usize = 0;

    for (mut config, mut sprite, facing, is_moving) in &mut query {
        config.timer.tick(time.delta());

        if let Some(atlas) = &mut sprite.texture_atlas {
            if !is_moving.0 {
                atlas.index = match facing {
                    FacingDirection::Right => IDLE_RIGHT,
                    FacingDirection::Left => IDLE_LEFT,
                    FacingDirection::Up => IDLE_UP,
                    FacingDirection::Down => IDLE_DOWN,
                };
            } else if config.timer.just_finished() {
                let row = match facing {
                    FacingDirection::Right => ROW_RIGHT,
                    FacingDirection::Left => ROW_LEFT,
                    FacingDirection::Up => ROW_LEFT,
                    FacingDirection::Down => ROW_RIGHT,
                };
                let row_start = row * COLUMNS;
                let row_end = row_start + config.last_index;

                atlas.index = if atlas.index >= row_end || atlas.index < row_start {
                    row_start
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}