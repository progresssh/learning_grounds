use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(Startup, setup_audio)
            .add_systems(Update, toggle_stems);
    }
}
#[derive(Resource)]
struct StemHandles {
    stems: Vec<Handle<AudioInstance>>,
    active: Vec<bool>,
}

fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let stem_files = [
        "audio/stem_bass.wav",
        "audio/stem_drums.wav",
        "audio/stem_others.wav",
    ];

    let mut handles = Vec::new();

    for file in stem_files {
        let handle = audio
            .play(asset_server.load(file))
            .looped()
            .with_volume(-120.0)
            .handle();
        handles.push(handle);
    }

    commands.insert_resource(StemHandles {
        stems: handles,
        active: vec![false; stem_files.len()],
    });

    info!("Press 1-3 to toggle stems");
}

fn toggle_stems(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut stem_handles: ResMut<StemHandles>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let keys = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
    ];

    for (i, key) in keys.iter().enumerate() {
        if keyboard.just_pressed(*key) {
            if let Some(instance) = audio_instances.get_mut(&stem_handles.stems[i]) {
                stem_handles.active[i] = !stem_handles.active[i];

                if stem_handles.active[i] {
                    instance.set_decibels(0.0, AudioTween::default());
                } else {
                    instance.set_decibels(-120.0, AudioTween::default());
                };

                info!(
                    "Stem {}: {}",
                    i + 1,
                    if stem_handles.active[i] { "ON" } else { "OFF" }
                );
            }
        }
    }
}
