use std::path::Path;

use bevy::prelude::*;
use bevy_persistent::prelude::*;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::game::GameBackgroundSound;

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
    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }
    pub fn get_sfx_volume(&self) -> f32 {
        self.sfx_volume
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
                        Path::new("local").join("volume")
                    },
                )
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .unwrap(),
        );

        app.add_systems(Startup, init_game_audio)
            .add_systems(Update, update_game_audio);
    }
}

fn init_game_audio(mut commands: Commands) {
    commands.spawn(GameAudio);
}

fn update_game_audio(
    mut commands: Commands,
    game_audio: Query<&Children, With<GameAudio>>,
    audio_sinks: Query<Option<&AudioSink>, Without<GameBackgroundSound>>,
    mut background_sound: Query<&mut AudioSink, With<GameBackgroundSound>>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
) {
    if let Ok(game_audio_children) = game_audio.get_single() {
        for game_audio_child in game_audio_children.iter() {
            let Ok(game_audio_child_audio_sink) = audio_sinks.get(*game_audio_child) else {
                continue;
            };
            let Some(game_audio_child_audio_sink) = game_audio_child_audio_sink else {
                commands.entity(*game_audio_child).despawn_recursive();
                continue;
            };

            if game_audio_volume.is_changed() {
                game_audio_child_audio_sink.set_volume(game_audio_volume.get_sfx_volume());
            }
        }
    }
    if let Ok(background_sound_sink) = background_sound.get_single_mut() {
        if game_audio_volume.is_changed() {
            background_sound_sink.set_volume(game_audio_volume.get_music_volume());
        }
    }
}
