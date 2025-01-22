pub mod audio;
pub mod levels;
pub mod sprites;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use levels::{LevelsAssets, LevelsPlugin};
use sprites::{tile::TileAssets, ui::UiAssets};

use crate::game::GameState;
use audio::{game::GameAudioAssets, ui::UiAudioAssets};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EmbeddedAssetPlugin::default(), LevelsPlugin));
        app.add_loading_state(
            LoadingState::new(GameState::AssetsLoading)
                .load_collection::<TileAssets>()
                .load_collection::<UiAssets>()
                .load_collection::<UiAudioAssets>()
                .load_collection::<GameAudioAssets>()
                .load_collection::<LevelsAssets>()
                .continue_to_state(GameState::Setup),
        );
    }
}
