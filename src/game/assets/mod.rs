pub mod audio;
pub mod levels;
pub mod sprites;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use crate::game::{
    assets::{
        audio::{game::GameAudioAssets, ui::UiAudioAssets},
        levels::{LevelsAssets, LevelsPlugin},
        sprites::{entity::EntityAssets, tilemap::TilemapTileAssets, ui::UiAssets},
    },
    GameState,
};

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EmbeddedAssetPlugin::default(), LevelsPlugin));
        app.add_loading_state(
            LoadingState::new(GameState::AssetsLoading)
                .load_collection::<EntityAssets>()
                .load_collection::<TilemapTileAssets>()
                .load_collection::<UiAssets>()
                .load_collection::<UiAudioAssets>()
                .load_collection::<GameAudioAssets>()
                .load_collection::<LevelsAssets>()
                .continue_to_state(GameState::Setup),
        );
    }
}
