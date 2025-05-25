use bevy::{audio::Volume, prelude::*};
use bevy_persistent::prelude::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::game::GameBackgroundAudio;

#[derive(Resource, Serialize, Deserialize)]
pub struct GameAudioVolume {
    music_volume: f32,
    sfx_volume: f32,
}

impl Default for GameAudioVolume {
    fn default() -> Self {
        Self {
            music_volume: 0.5,
            sfx_volume: 0.5,
        }
    }
}

impl GameAudioVolume {
    pub fn get_music_volume(&self) -> Volume {
        Volume::Linear(self.music_volume)
    }
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }
    pub fn get_sfx_volume(&self) -> Volume {
        Volume::Linear(self.sfx_volume)
    }
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }
}

#[derive(Component)]
pub struct GameAudio;

pub struct GameAudioPlugin;
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            Persistent::<GameAudioVolume>::builder()
                .name("volume")
                .format(StorageFormat::Ron)
                .default(GameAudioVolume::default())
                .path(
                    if let Some(proj_dirs) = ProjectDirs::from("ru", "kicshikxo", "pico-td") {
                        proj_dirs.preference_dir().join("volume.ron")
                    } else {
                        std::path::Path::new("local").join("volume")
                    },
                )
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .unwrap(),
        );

        app.add_systems(Startup, init_game_audio);
        app.add_systems(PostUpdate, despawn_game_audio);

        app.add_systems(
            Update,
            update_game_audio_volume.run_if(resource_changed::<Persistent<GameAudioVolume>>),
        );
    }
}

fn init_game_audio(mut commands: Commands) {
    commands.spawn(GameAudio);
}

fn despawn_game_audio(
    mut commands: Commands,
    mut removed_audio_sinks: RemovedComponents<AudioSink>,
) {
    for removed_audio_sink_entity in removed_audio_sinks.read() {
        if commands.get_entity(removed_audio_sink_entity).is_ok() {
            commands.entity(removed_audio_sink_entity).despawn();
        }
    }
}

fn update_game_audio_volume(
    game_audio: Single<&Children, With<GameAudio>>,
    mut audio_sinks: Query<&mut AudioSink, Without<GameBackgroundAudio>>,
    mut background_audio: Query<&mut AudioSink, With<GameBackgroundAudio>>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
) {
    for game_audio_child in game_audio.iter() {
        let Ok(mut game_audio_child_audio_sink) = audio_sinks.get_mut(game_audio_child) else {
            continue;
        };

        game_audio_child_audio_sink.set_volume(game_audio_volume.get_sfx_volume());
    }

    if let Ok(mut background_audio_sink) = background_audio.single_mut() {
        background_audio_sink.set_volume(game_audio_volume.get_music_volume());
    }
}
