use std::{f32::consts::PI, ops::Range};
use bevy::{camera::ScalingMode, input::mouse::AccumulatedMouseScroll, prelude::*};use bevy::prelude::*;

use crate::player::Player;


#[derive(Debug, Resource)]
struct CameraSettings {
    pub viewport_height: f32,
    pub zoom_range: Range<f32>,
    pub zoom_speed: f32,
}
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings {
            viewport_height: 1000.,
            zoom_range: 0.1..10.0,
            zoom_speed: 0.2,
        })
            .add_systems(Startup, setup_camera)
            .add_systems(Update, move_camera)
            .add_systems(Update, zoom_camera);
    }
}

fn setup_camera(mut commands: Commands, camera_settings: Res<CameraSettings>) {
    commands.spawn((
        Name::new("Camera"),
        Msaa::Off,
        Camera2d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical { viewport_height: camera_settings.viewport_height },
            scale: 1.,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}

fn move_camera(
    mut camera: Option<Single<&mut Transform, (With<Camera2d>, Without<Player>)>>,
    player: Option<Single<&Transform, (With<Player>, Without<Camera2d>)>>,
) {
    if let Some(player_transform) = player {
        if let Some(camera_transform) = &mut camera {
            camera_transform.translation = player_transform.translation;
        }
    };
}

fn zoom_camera(
    camera: Single<&mut Projection, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    let Projection::Orthographic(ref mut orthographic) = *camera.into_inner() else {
        return;
    };

    let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.zoom_speed;
    let mult_zoom = 1. + delta_zoom;

    orthographic.scale = (orthographic.scale * mult_zoom).clamp(camera_settings.zoom_range.start, camera_settings.zoom_range.end);
}