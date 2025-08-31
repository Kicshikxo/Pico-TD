use bevy::{
    asset::{AssetLoader, LoadContext, RenderAssetUsages, io::Reader},
    math::VectorSpace,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{
    entities::{
        enemy::EnemyVariant,
        tilemap::tile::{TilemapTile, TilemapTileVariant},
    },
    player::PlayerHealth,
};

#[derive(AssetCollection, Resource)]
pub struct LevelsAssets {
    #[asset(
        paths(
            "embedded://levels/ring.ron",
            "embedded://levels/zigzag.ron",
            "embedded://levels/coastal_highway.ron"
        ),
        collection(typed)
    )]
    pub compain: Vec<Handle<Level>>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
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
        let health_percentage = health.get_percentage();

        if health_percentage >= 0.9 {
            LevelCompletionStars::Three
        } else if health_percentage >= 0.67 {
            LevelCompletionStars::Two
        } else if health_percentage > 0.0 {
            LevelCompletionStars::One
        } else {
            LevelCompletionStars::Zero
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LevelCompletion {
    name: String,
    stars: LevelCompletionStars,
}

impl LevelCompletion {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_stars(&self) -> &LevelCompletionStars {
        &self.stars
    }
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct CompletedLevels(Vec<LevelCompletion>);

impl CompletedLevels {
    pub fn add(&mut self, name: &str, stars: LevelCompletionStars) {
        if let Some(level_completion) = self.get_completion_mut(name) {
            if stars.as_index() > level_completion.stars.as_index() {
                level_completion.stars = stars;
            }
        } else {
            self.0.push(LevelCompletion {
                name: name.into(),
                stars,
            });
        }
    }
    pub fn get_completion(&self, name: &str) -> Option<&LevelCompletion> {
        self.0.iter().find(|level| level.get_name() == name)
    }
    fn get_completion_mut(&mut self, name: &str) -> Option<&mut LevelCompletion> {
        self.0.iter_mut().find(|level| level.get_name() == name)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn reset(&mut self) {
        self.0.clear();
    }
}

#[derive(Default, Clone, Deserialize)]
pub struct Path {
    color: Vec3,
    points: Vec<Vec2>,
}

impl Path {
    pub fn get_color(&self) -> Color {
        LinearRgba::from_vec3(self.color).into()
    }
    pub fn get_points(&self) -> &Vec<Vec2> {
        &self.points
    }
}

#[derive(Clone, Deserialize)]
pub struct Wave {
    reward: u32,
    enemies: Vec<WaveEnemies>,
}

impl Wave {
    pub fn get_reward(&self) -> u32 {
        self.reward
    }
    pub fn get_enemies(&self) -> &Vec<WaveEnemies> {
        &self.enemies
    }
}

#[derive(Clone, Deserialize)]
pub struct WaveEnemies {
    enemy_variant: EnemyVariant,
    count: u32,
    duration: f32,
    spawn_interval: f32,
    spawn_delay: f32,
    path_index: usize,
}

impl WaveEnemies {
    pub fn get_enemy_variant(&self) -> EnemyVariant {
        self.enemy_variant.clone()
    }
    pub fn get_count(&self) -> u32 {
        self.count
    }
    pub fn get_duration(&self) -> f32 {
        self.duration
    }
    pub fn get_spawn_interval(&self) -> f32 {
        self.spawn_interval
    }
    pub fn get_spawn_delay(&self) -> f32 {
        self.spawn_delay
    }
    pub fn get_path_index(&self) -> usize {
        self.path_index
    }
}

#[derive(Asset, TypePath, Deserialize)]
pub struct TileSymbols {
    pub ground: char,
    pub flower: char,
    pub tree: char,
    pub road: char,
    pub bridge: char,
    pub water: char,
}

impl Default for TileSymbols {
    fn default() -> Self {
        Self {
            ground: '#',
            flower: 'F',
            tree: 'T',
            bridge: '=',
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
        } else if char == self.bridge {
            TilemapTileVariant::Bridge
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
    pub viewport_size: Option<UVec2>,
    pub map_size: UVec2,
    pub map: Vec<String>,
    pub tile_symbols: Option<TileSymbols>,
    pub paths: Option<Vec<Path>>,
    pub waves: Option<Vec<Wave>>,
    pub error: Option<String>,
}

impl Default for LevelAsset {
    fn default() -> Self {
        Self {
            name: String::new(),
            player_health: 0,
            player_money: 0,
            viewport_size: None,
            map_size: UVec2::default(),
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

#[derive(Resource, Asset, TypePath, Clone)]
pub struct Level {
    name: String,
    player_health: u32,
    player_money: u32,
    viewport_size: Option<UVec2>,
    map_size: UVec2,
    map: Vec<Vec<TilemapTile>>,
    paths: Vec<Path>,
    waves: Vec<Wave>,
    error: Option<String>,
}

impl Level {
    pub fn from_asset(level_asset: LevelAsset) -> Self {
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

        Self {
            name: level_asset.name,
            player_health: level_asset.player_health,
            player_money: level_asset.player_money,
            viewport_size: level_asset.viewport_size,
            map_size: level_asset.map_size,
            map,
            paths: level_asset.paths.unwrap_or_default(),
            waves: level_asset.waves.unwrap_or_default(),
            error: level_asset.error,
        }
    }
    pub fn from_source(source: &str) -> Self {
        let level_asset = match ron::from_str::<LevelAsset>(source) {
            Ok(asset) => {
                if asset.map.len() < asset.map_size.y as usize
                    || asset
                        .map
                        .iter()
                        .any(|row| row.len() < asset.map_size.x as usize)
                {
                    LevelAsset::error("LevelAsset error: map dimensions are incorrect".to_string())
                } else {
                    asset
                }
            }
            Err(error) => {
                error!("Failed to deserialize RON: {}", error);
                LevelAsset::error(error.to_string())
            }
        };

        Self::from_asset(level_asset)
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_player_health(&self) -> u32 {
        self.player_health
    }
    pub fn get_player_money(&self) -> u32 {
        self.player_money
    }
    pub fn get_map_size(&self) -> UVec2 {
        self.map_size
    }
    pub fn get_viewport_size(&self) -> Option<UVec2> {
        self.viewport_size
    }
    pub fn get_map(&self) -> &Vec<Vec<TilemapTile>> {
        &self.map
    }
    pub fn get_tile(&self, x: u32, y: u32) -> TilemapTile {
        self.get_map()
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .cloned()
            .unwrap_or_default()
    }
    pub fn get_paths(&self) -> &Vec<Path> {
        &self.paths
    }
    pub fn get_path(&self, path_index: usize) -> Path {
        self.get_paths()
            .get(path_index)
            .cloned()
            .unwrap_or_default()
    }
    pub fn get_waves(&self) -> &Vec<Wave> {
        &self.waves
    }
    pub fn get_wave(&self, wave_index: usize) -> Option<&Wave> {
        self.waves.get(wave_index)
    }
    pub fn get_error(&self) -> Option<String> {
        self.error.clone()
    }
    fn get_tile_preview_color(variant: TilemapTileVariant) -> Color {
        match variant {
            TilemapTileVariant::Ground => Color::srgb(132.0 / 255.0, 198.0 / 255.0, 105.0 / 255.0),
            TilemapTileVariant::Flower => Color::srgb(179.0 / 255.0, 195.0 / 255.0, 104.0 / 255.0),
            TilemapTileVariant::Tree => Color::srgb(67.0 / 255.0, 149.0 / 255.0, 69.0 / 255.0),
            TilemapTileVariant::Road => Color::srgb(82.0 / 255.0, 96.0 / 255.0, 124.0 / 255.0),
            TilemapTileVariant::Bridge => Color::srgb(82.0 / 255.0, 96.0 / 255.0, 124.0 / 255.0),
            TilemapTileVariant::Water => Color::srgb(117.0 / 255.0, 227.0 / 255.0, 255.0 / 255.0),
            _ => Color::NONE,
        }
    }
    pub fn get_preview(&self) -> Image {
        let mut image = Image::new_fill(
            Extent3d {
                width: self.get_map_size().x as u32,
                height: self.get_map_size().y as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &Srgba::ZERO.to_u8_array(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );

        for x in 0..self.get_map_size().x {
            for y in 0..self.get_map_size().y {
                image
                    .set_color_at(
                        x,
                        y,
                        Self::get_tile_preview_color(self.get_tile(x, y).get_variant()),
                    )
                    .unwrap();
            }
        }

        image
    }
}

impl Default for Level {
    fn default() -> Self {
        Self {
            name: String::new(),
            player_health: 0,
            player_money: 0,
            viewport_size: None,
            map_size: UVec2::default(),
            map: Vec::new(),
            paths: Vec::new(),
            waves: Vec::new(),
            error: None,
        }
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
                    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
                    directories::ProjectDirs::from("ru", "kicshikxo", "pico-td")
                        .unwrap()
                        .data_dir()
                        .join("completed_levels.ron"),
                    #[cfg(target_arch = "wasm32")]
                    std::path::Path::new("local").join("completed_levels"),
                    #[cfg(target_os = "android")]
                    "/data/data/ru.kicshikxo.pico_td/files/completed_levels.ron",
                )
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .unwrap(),
        );
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

        let source = std::str::from_utf8(&bytes).unwrap_or_default();

        Ok(Level::from_source(source))
    }
    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
