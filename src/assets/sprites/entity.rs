use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct EntityAssets {
    #[asset(path = "embedded://images/tiles/entities.png")]
    pub tilemap: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 12, rows = 11))]
    pub tilemap_layout: Handle<TextureAtlasLayout>,
}

pub enum ProjectileSpriteVariant {
    Bullet = 120,
    Rocket = 121,
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum SoldierSpriteVariant {
    SoldierGray = 8,
    SoldierGreen = 20,
    SoldierBlue = 32,
    SoldierRed = 44,
    SoldierYellow = 56,
    RocketLauncherGray = 9,
    RocketLauncherGreen = 21,
    RocketLauncherBlue = 33,
    RocketLauncherRed = 45,
    RocketLauncherYellow = 57,
}

pub enum EnemySpriteVariant {
    Dron = 67,
    Truck = 60,
    Tank = 63,
    Plane = 65,
    Helicopter = 66,
    Boat = 69,
    Submarine = 71,
}

pub enum UtilSpriteVariant {
    TileIndicator = 122,
    Glow = 123,
}
