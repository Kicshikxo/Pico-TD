use std::path::Path;

use bevy::prelude::*;
use bevy_persistent::prelude::*;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

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
    }
}
