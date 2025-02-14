use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use serde::Deserialize;

use crate::entities::{
    enemy::EnemyVariant,
    tilemap::tile::{TilemapTile, TilemapTileVariant},
};

#[derive(AssetCollection, Resource)]
pub struct LevelsAssets {
    #[asset(
        paths("embedded://levels/ring.ron", "embedded://levels/zigzag.ron"),
        collection(typed)
    )]
    pub compain: Vec<Handle<Level>>,

    #[asset(path = "embedded://images/levels/ring.png")]
    pub ring_preview: Handle<Image>,
    #[asset(path = "embedded://images/levels/zigzag.png")]
    pub zigzag_preview: Handle<Image>,
    #[asset(path = "embedded://images/levels/error.png")]
    pub error_preview: Handle<Image>,
}

impl LevelsAssets {
    pub fn get_level_preview(&self, level: &Level) -> Handle<Image> {
        match level.name.as_str() {
            "ring" => self.ring_preview.clone(),
            "zigzag" => self.zigzag_preview.clone(),
            _ => self.error_preview.clone(),
        }
    }
}

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Level>()
            .init_asset_loader::<LevelsLoader>();

        app.insert_resource(Level::default());
    }
}

#[derive(Resource, Asset, TypePath, Clone)]
pub struct Level {
    pub name: String,
    pub player_health: u32,
    pub player_money: u32,
    pub size: UVec2,
    pub map: Vec<Vec<TilemapTile>>,
    pub paths: Vec<Vec<Vec2>>,
    pub waves: Vec<Vec<Wave>>,
    pub error: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct Wave {
    pub enemy_variant: EnemyVariant,
    pub count: u32,
    pub duration: f32,
    pub spawn_interval: f32,
    pub spawn_delay: f32,
    pub path_index: usize,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            name: String::new(),
            player_health: 0,
            player_money: 0,
            size: UVec2::new(0, 0),
            map: Vec::new(),
            paths: Vec::new(),
            waves: Vec::new(),
            error: None,
        }
    }
}

#[derive(Asset, TypePath, Deserialize, Debug)]
pub struct TileSymbols {
    pub ground: char,
    pub flower: char,
    pub tree: char,
    pub road: char,
    pub water: char,
}

impl Default for TileSymbols {
    fn default() -> Self {
        Self {
            ground: '#',
            flower: 'F',
            tree: 'T',
            road: '.',
            water: '~',
        }
    }
}

impl TileSymbols {
    pub fn get_tile_variant(&self, char: char) -> TilemapTileVariant {
        if char == self.ground {
            TilemapTileVariant::Ground
        } else if char == self.flower {
            TilemapTileVariant::Flower
        } else if char == self.tree {
            TilemapTileVariant::Tree
        } else if char == self.road {
            TilemapTileVariant::Road
        } else if char == self.water {
            TilemapTileVariant::Water
        } else {
            TilemapTileVariant::Unknown
        }
    }
}

#[derive(Asset, TypePath, Deserialize)]
pub struct LevelAsset {
    pub name: String,
    pub player_health: u32,
    pub player_money: u32,
    pub size: UVec2,
    pub map: Vec<String>,
    pub tile_symbols: Option<TileSymbols>,
    pub paths: Option<Vec<Vec<Vec2>>>,
    pub waves: Option<Vec<Vec<Wave>>>,
    pub error: Option<String>,
}

impl Default for LevelAsset {
    fn default() -> Self {
        Self {
            name: String::new(),
            player_health: 0,
            player_money: 0,
            size: UVec2::new(0, 0),
            map: Vec::new(),
            tile_symbols: None,
            paths: None,
            waves: None,
            error: None,
        }
    }
}

impl LevelAsset {
    fn error(error: String) -> Self {
        Self {
            name: "error".into(),
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

        let tile_symbols = level_asset.tile_symbols.unwrap_or_default();
        let map: Vec<Vec<TilemapTile>> = level_asset
            .map
            .iter()
            .rev()
            .map(|row| {
                row.chars()
                    .map(|char| TilemapTile::new(tile_symbols.get_tile_variant(char)))
                    .collect()
            })
            .collect();

        Ok(Level {
            name: level_asset.name,
            player_health: level_asset.player_health,
            player_money: level_asset.player_money,
            size: level_asset.size,
            map,
            paths: level_asset.paths.unwrap_or_default(),
            waves: level_asset.waves.unwrap_or_default(),
            error: level_asset.error,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
