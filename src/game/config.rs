use std::path::Path;

use bevy::prelude::*;
use bevy_persistent::prelude::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub enum SoldierPlacement {
    #[default]
    WithConfirmation,
    WithoutConfirmation,
}

impl SoldierPlacement {
    pub fn to_str(&self) -> &'static str {
        match self {
            SoldierPlacement::WithConfirmation => "soldier.placement.with_confirmation",
            SoldierPlacement::WithoutConfirmation => "soldier.placement.without_confirmation",
        }
    }
    pub fn as_index(&self) -> usize {
        match self {
            SoldierPlacement::WithConfirmation => 0,
            SoldierPlacement::WithoutConfirmation => 1,
        }
    }
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => SoldierPlacement::WithConfirmation,
            1 => SoldierPlacement::WithoutConfirmation,
            _ => SoldierPlacement::default(),
        }
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct GameConfig {
    soldier_placement: SoldierPlacement,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            soldier_placement: SoldierPlacement::default(),
        }
    }
}

impl GameConfig {
    pub fn get_soldier_placement(&self) -> SoldierPlacement {
        self.soldier_placement
    }
    pub fn set_soldier_placement(&mut self, placement: SoldierPlacement) {
        self.soldier_placement = placement;
    }
}

pub struct GameConfigPlugin;

impl Plugin for GameConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            Persistent::<GameConfig>::builder()
                .name("config")
                .format(StorageFormat::Ron)
                .default(GameConfig::default())
                .path(
                    if let Some(proj_dirs) = ProjectDirs::from("ru", "kicshikxo", "pico-td") {
                        proj_dirs.preference_dir().join("config.ron")
                    } else {
                        Path::new("local").join("config")
                    },
                )
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .unwrap(),
        );
    }
}
