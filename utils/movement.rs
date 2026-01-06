use bevy::prelude::*;

pub fn sine_wave_offset(time: f32, amplitude: f32, frequency: f32) -> f32 {
    amplitude * (time * frequency).sin()
}

pub fn apply_sine_movement(
    transform: &mut Transform,
    base_y: f32,
    time: f32,
    amplitude: f32,
    frequency: f32
) {
    transform.translation.y = base_y + sine_wave_offset(time, amplitude, frequency);
}