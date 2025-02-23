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
    Bullet = 10,
    Rocket = 11,
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum SoldierSpriteVariant {
    SoldierGray = 0,
    SoldierRed = 1,
    SoldierGreen = 2,
    SoldierBlue = 3,
    SoldierYellow = 4,
    RocketLauncherGray = 5,
    RocketLauncherRed = 6,
    RocketLauncherGreen = 7,
    RocketLauncherBlue = 8,
    RocketLauncherYellow = 9,
}

pub enum EnemySpriteVariant {
    Dron = 19,
    Truck = 12,
    Tank = 15,
    Plane = 17,
    Helicopter = 18,
    Boat = 21,
    Submarine = 23,
}

pub enum UtilSpriteVariant {
    TileIndicator = 72,
    Glow = 73,
}
