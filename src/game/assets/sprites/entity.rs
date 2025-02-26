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

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum EnemySpriteVariant {
    DronGray = 19 + 12 * 0,
    DronRed = 19 + 12 * 1,
    DronGreen = 19 + 12 * 2,
    DronBlue = 19 + 12 * 3,
    DronYellow = 19 + 12 * 4,

    TruckGray = 12 + 12 * 0,
    TruckRed = 12 + 12 * 1,
    TruckGreen = 12 + 12 * 2,
    TruckBlue = 12 + 12 * 3,
    TruckYellow = 12 + 12 * 4,

    TankGray = 15 + 12 * 0,
    TankRed = 15 + 12 * 1,
    TankGreen = 15 + 12 * 2,
    TankBlue = 15 + 12 * 3,
    TankYellow = 15 + 12 * 4,

    PlaneGray = 17 + 12 * 0,
    PlaneRed = 17 + 12 * 1,
    PlaneGreen = 17 + 12 * 2,
    PlaneBlue = 17 + 12 * 3,
    PlaneYellow = 17 + 12 * 4,

    HelicopterGray = 18 + 12 * 0,
    HelicopterRed = 18 + 12 * 1,
    HelicopterGreen = 18 + 12 * 2,
    HelicopterBlue = 18 + 12 * 3,
    HelicopterYellow = 18 + 12 * 4,

    BoatGray = 21 + 12 * 0,
    BoatRed = 21 + 12 * 1,
    BoatGreen = 21 + 12 * 2,
    BoatBlue = 21 + 12 * 3,
    BoatYellow = 21 + 12 * 4,

    SubmarineGray = 23 + 12 * 0,
    SubmarineRed = 23 + 12 * 1,
    SubmarineGreen = 23 + 12 * 2,
    SubmarineBlue = 23 + 12 * 3,
    SubmarineYellow = 23 + 12 * 4,
}

pub enum UtilSpriteVariant {
    TileIndicator = 72,
    Glow = 73,
}
