use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::entities::{enemy::path::EnemyPathVisibility, soldier::SoldierPlacement};

#[derive(Resource, Serialize, Deserialize)]
pub struct GameConfig {
    soldier_placement: SoldierPlacement,
    enemy_path_visibility: EnemyPathVisibility,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            soldier_placement: SoldierPlacement::default(),
            enemy_path_visibility: EnemyPathVisibility::default(),
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
    pub fn get_enemy_path_visibility(&self) -> EnemyPathVisibility {
        self.enemy_path_visibility
    }
    pub fn set_enemy_path_visibility(&mut self, visibility: EnemyPathVisibility) {
        self.enemy_path_visibility = visibility;
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
                    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
                    directories::ProjectDirs::from("ru", "kicshikxo", "pico-td")
                        .unwrap()
                        .preference_dir()
                        .join("config.ron"),
                    #[cfg(target_arch = "wasm32")]
                    std::path::Path::new("local").join("config"),
                    #[cfg(target_os = "android")]
                    "/data/data/ru.kicshikxo.pico_td/files/config.ron",
                )
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .unwrap(),
        );
    }
}
