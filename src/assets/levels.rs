use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::entities::tilemap::tile::TilemapTile;

#[derive(AssetCollection, Resource)]
pub struct LevelsAssets {
    #[asset(
        paths(
            "embedded://levels/compain/forest.ron",
            "embedded://levels/compain/example.ron",
            "embedded://levels/compain/zig-zag.ron"
        ),
        collection(typed)
    )]
    pub compain: Vec<Handle<Level>>,
}

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Level>()
            .init_asset_loader::<LevelsLoader>();

        app.insert_resource(Level::default());
    }
}

#[derive(Resource, Asset, TypePath, Clone, Debug)]
pub struct Level {
    pub name: String,
    pub size: UVec2,
    pub map: Vec<Vec<TilemapTile>>,
    pub paths: Vec<Vec<Vec2>>,
    pub error: Option<String>,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            name: String::new(),
            size: UVec2::new(0, 0),
            map: Vec::new(),
            paths: Vec::new(),
            error: None,
        }
    }
}

#[derive(Asset, TypePath, Debug, serde::Deserialize)]
pub struct LevelAsset {
    pub name: String,
    pub size: UVec2,
    pub map: Vec<String>,
    pub paths: Option<Vec<Vec<Vec2>>>,
    pub error: Option<String>,
}

impl Default for LevelAsset {
    fn default() -> Self {
        Self {
            name: String::new(),
            size: UVec2::new(0, 0),
            map: Vec::new(),
            paths: None,
            error: None,
        }
    }
}

impl LevelAsset {
    fn error(error: String) -> Self {
        Self {
            name: "Ошибка".into(),
            error: Some(error),
            ..default()
        }
    }
}

#[derive(Default)]
struct LevelsLoader;

impl AssetLoader for LevelsLoader {
    type Asset = Level;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let data = std::str::from_utf8(&bytes).unwrap_or("");
        let level_asset: LevelAsset = ron::from_str(&data).unwrap_or_else(|error| {
            error!("Failed to deserialize RON: {}", error);
            LevelAsset::error(error.to_string())
        });

        let map: Vec<Vec<TilemapTile>> = level_asset
            .map
            .iter()
            .rev()
            .map(|row| row.chars().map(TilemapTile::from).collect())
            .collect();

        Ok(Level {
            name: level_asset.name,
            size: level_asset.size,
            map,
            paths: level_asset.paths.unwrap_or_default(),
            error: level_asset.error,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
