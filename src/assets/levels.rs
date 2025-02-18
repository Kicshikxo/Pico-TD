use std::{path::Path, time::SystemTime};

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext, RenderAssetUsages},
    math::VectorSpace,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_persistent::prelude::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        enemy::EnemyVariant,
        tilemap::tile::{TilemapTile, TilemapTileVariant},
    },
    player::PlayerHealth,
};

#[derive(AssetCollection, Resource)]
pub struct LevelsAssets {
    #[asset(
        paths("embedded://levels/ring.ron", "embedded://levels/zigzag.ron"),
        collection(typed)
    )]
    pub compain: Vec<Handle<Level>>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub enum LevelCompletionStars {
    #[default]
    Zero,
    One,
    Two,
    Three,
}

impl LevelCompletionStars {
    pub fn as_index(&self) -> usize {
        match self {
            LevelCompletionStars::Zero => 0,
            LevelCompletionStars::One => 1,
            LevelCompletionStars::Two => 2,
            LevelCompletionStars::Three => 3,
        }
    }
}

impl LevelCompletionStars {
    pub fn from_player_health(health: &PlayerHealth) -> Self {
        let health_percent = health.get_current() as f32 / health.get_max() as f32;

        if health_percent > 2.0 / 3.0 {
            LevelCompletionStars::Three
        } else if health_percent > 1.0 / 3.0 {
            LevelCompletionStars::Two
        } else {
            LevelCompletionStars::One
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LevelCompletion {
    name: String,
    stars: LevelCompletionStars,
    completed_at: u64,
}

#[allow(unused)]
impl LevelCompletion {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_stars(&self) -> &LevelCompletionStars {
        &self.stars
    }
    pub fn get_completed_at(&self) -> u64 {
        self.completed_at
    }
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct CompletedLevels(Vec<LevelCompletion>);

impl CompletedLevels {
    pub fn add(&mut self, name: &str, stars: LevelCompletionStars) {
        let timestamp = if cfg!(target_arch = "wasm32") {
            (js_sys::Date::now() / 1000.0) as u64
        } else {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        };

        if let Some(level_completion) = self.get_completion_mut(name) {
            level_completion.stars = stars;
            level_completion.completed_at = timestamp;
        } else {
            self.0.push(LevelCompletion {
                name: name.into(),
                stars,
                completed_at: timestamp,
            });
        }
    }
    pub fn get_completion(&self, name: &str) -> Option<&LevelCompletion> {
        self.0.iter().find(|level| level.name == name)
    }
    fn get_completion_mut(&mut self, name: &str) -> Option<&mut LevelCompletion> {
        self.0.iter_mut().find(|level| level.name == name)
    }
    pub fn reset(&mut self) {
        self.0.clear();
    }
}

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Level>()
            .init_asset_loader::<LevelsLoader>();

        app.insert_resource(Level::default());

        app.insert_resource(
            Persistent::<CompletedLevels>::builder()
                .name("completed_levels")
                .format(StorageFormat::Ron)
                .default(CompletedLevels::default())
                .path(
                    if let Some(proj_dirs) = ProjectDirs::from("ru", "kicshikxo", "pico-td") {
                        proj_dirs.data_dir().join("completed_levels.ron")
                    } else {
                        Path::new("local").join("completed_levels")
                    },
                )
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .unwrap(),
        );
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

impl Level {
    pub fn get_preview(&self) -> Image {
        let mut image = Image::new_fill(
            Extent3d {
                width: self.size.x as u32,
                height: self.size.y as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &Srgba::ZERO.to_u8_array(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all(),
        );

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                // if (x == 0 || x == self.size.x - 1) && (y == 0 || y == self.size.y - 1) {
                //     continue;
                // }
                image
                    .set_color_at(
                        x,
                        y,
                        self.map[y as usize][x as usize]
                            .get_variant()
                            .get_preview_color(),
                    )
                    .unwrap();
            }
        }

        image
    }
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

#[derive(Asset, TypePath, Deserialize)]
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
