use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::Rng;

use crate::entities::tilemap::tile::TilemapTileVariant;

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(path = "embedded://images/tiles/tilemap_tiles.png")]
    pub tilemap_tiles: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 8, rows = 10))]
    pub tilemap_tiles_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "embedded://images/tiles/entities.png")]
    pub entities: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 12, rows = 11))]
    pub entities_layout: Handle<TextureAtlasLayout>,
}

pub enum TilemapTileSpriteVariant {
    Unknown = 0,

    Ground = 1,
    GroundWithGrass = 2,
    GroundWithFlower = 3,
    GroundWithTree = 4,
    GroundWithDoubleTree = 5,

    Road = 48,
    RoadTop = 72,
    RoadRight = 49,
    RoadBottom = 56,
    RoadLeft = 51,
    RoadTopBottom = 64,
    RoadLeftRight = 50,
    RoadTopLeft = 75,
    RoadTopRight = 73,
    RoadBottomLeft = 59,
    RoadBottomRight = 57,
    RoadTopLeftRight = 74,
    RoadRightTopBottom = 65,
    RoadBottomLeftRight = 58,
    RoadLeftTopBottom = 67,
    RoadTopRightBottomLeft = 66,

    Water = 6,
    WaterShoreTop = 12,
    WaterShoreTopDiagonalLeft = 20,
    WaterShoreTopDiagonalRight = 28,
    WaterShoreTopDiagonalLeftRight = 36,
    WaterShoreRight = 13,
    WaterShoreRightDiagonalTop = 21,
    WaterShoreRightDiagonalBottom = 29,
    WaterShoreRightDiagonalTopBottom = 37,
    WaterShoreBottom = 14,
    WaterShoreBottomDiagonalLeft = 22,
    WaterShoreBottomDiagonalRight = 30,
    WaterShoreBottomDiagonalLeftRight = 38,
    WaterShoreLeft = 15,
    WaterShoreLeftDiagonalTop = 23,
    WaterShoreLeftDiagonalBottom = 31,
    WaterShoreLeftDiagonalTopBottom = 39,
    WaterShoreTopBottom = 41,
    WaterShoreLeftRight = 35,
    WaterShoreTopLeftRight = 44,
    WaterShoreRightTopBottom = 45,
    WaterShoreBottomLeftRight = 46,
    WaterShoreLeftTopBottom = 47,
    WaterShoreTopRightBottomLeft = 7,
    WaterShoreTopLeft = 8,
    WaterShoreTopLeftDiagonalBottomRight = 10,
    WaterShoreTopRight = 9,
    WaterShoreTopRightDiagonalBottomLeft = 11,
    WaterShoreBottomLeft = 16,
    WaterShoreBottomLeftDiagonalTopRight = 18,
    WaterShoreBottomRight = 17,
    WaterShoreBottomRightDiagonalTopLeft = 19,
    WaterShoreDiagonalTopLeft = 33,
    WaterShoreDiagonalTopRight = 32,
    WaterShoreDiagonalBottomLeft = 25,
    WaterShoreDiagonalBottomRight = 24,
    WaterShoreDiagonalTopLeftDiagonalBottomRight = 26,
    WaterShoreDiagonalTopRightDiagonalBottomLeft = 34,
    WaterShoreDiagonalTopLeftRight = 43,
    WaterShoreDiagonalRightTopBottom = 40,
    WaterShoreDiagonalBottomLeftRight = 27,
    WaterShoreDiagonalLeftTopBottom = 42,
}

pub enum EntityProjectileVariant {
    Bullet = 120,
    Rocket = 121,
}

pub enum EntityStructureVariant {
    Soldier = 8,
    SoldierFast = 56,
    SoldierStrong = 44,
    RocketLauncher = 9,
}

pub enum EntityUnitVariant {
    Truck = 60,
    Plane = 65,
    Tank = 63,
    Boat = 68,
    Submarine = 70,
}

impl TileAssets {
    pub fn get_tile_index(
        &self,
        variant: TilemapTileVariant,
        tiles_around: [[TilemapTileVariant; 3]; 3],
    ) -> usize {
        match variant {
            TilemapTileVariant::Road => self.get_road_tile_index(tiles_around) as usize,
            TilemapTileVariant::Water => self.get_water_tile_index(tiles_around) as usize,
            TilemapTileVariant::Ground => {
                if rand::thread_rng().gen_bool(0.25) {
                    TilemapTileSpriteVariant::GroundWithGrass as usize
                } else {
                    TilemapTileSpriteVariant::Ground as usize
                }
            }
            TilemapTileVariant::Flower => TilemapTileSpriteVariant::GroundWithFlower as usize,
            TilemapTileVariant::Tree => {
                if rand::thread_rng().gen_bool(0.25) {
                    TilemapTileSpriteVariant::GroundWithDoubleTree as usize
                } else {
                    TilemapTileSpriteVariant::GroundWithTree as usize
                }
            }
            _ => 0,
        }
    }

    pub fn get_road_tile_index(
        &self,
        tiles_around: [[TilemapTileVariant; 3]; 3],
    ) -> TilemapTileSpriteVariant {
        let road_top = tiles_around[0][1] == TilemapTileVariant::Road;
        let road_right = tiles_around[1][2] == TilemapTileVariant::Road;
        let road_bottom = tiles_around[2][1] == TilemapTileVariant::Road;
        let road_left = tiles_around[1][0] == TilemapTileVariant::Road;

        match (road_top, road_right, road_bottom, road_left) {
            (false, false, false, false) => TilemapTileSpriteVariant::Road,

            (true, false, false, false) => TilemapTileSpriteVariant::RoadTop,
            (false, true, false, false) => TilemapTileSpriteVariant::RoadRight,
            (false, false, true, false) => TilemapTileSpriteVariant::RoadBottom,
            (false, false, false, true) => TilemapTileSpriteVariant::RoadLeft,

            (true, false, true, false) => TilemapTileSpriteVariant::RoadTopBottom,
            (false, true, false, true) => TilemapTileSpriteVariant::RoadLeftRight,

            (true, false, false, true) => TilemapTileSpriteVariant::RoadTopLeft,
            (true, true, false, false) => TilemapTileSpriteVariant::RoadTopRight,
            (false, false, true, true) => TilemapTileSpriteVariant::RoadBottomLeft,
            (false, true, true, false) => TilemapTileSpriteVariant::RoadBottomRight,

            (true, true, false, true) => TilemapTileSpriteVariant::RoadTopLeftRight,
            (true, true, true, false) => TilemapTileSpriteVariant::RoadRightTopBottom,
            (false, true, true, true) => TilemapTileSpriteVariant::RoadBottomLeftRight,
            (true, false, true, true) => TilemapTileSpriteVariant::RoadLeftTopBottom,

            (true, true, true, true) => TilemapTileSpriteVariant::RoadTopRightBottomLeft,
        }
    }

    pub fn get_water_tile_index(
        &self,
        tiles_around: [[TilemapTileVariant; 3]; 3],
    ) -> TilemapTileSpriteVariant {
        let [[ground_top_left, ground_top, ground_top_right], [ground_left, _, ground_right], [ground_bottom_left, ground_bottom, ground_bottom_right]] =
            tiles_around.map(|row| {
                row.map(|tile| {
                    matches!(
                        tile,
                        TilemapTileVariant::Ground
                            | TilemapTileVariant::Flower
                            | TilemapTileVariant::Tree
                            | TilemapTileVariant::Road
                    )
                })
            });

        match (ground_top, ground_right, ground_bottom, ground_left) {
            (true, false, false, false) => match (ground_bottom_left, ground_bottom_right) {
                (false, false) => TilemapTileSpriteVariant::WaterShoreTop,
                (true, false) => TilemapTileSpriteVariant::WaterShoreTopDiagonalLeft,
                (false, true) => TilemapTileSpriteVariant::WaterShoreTopDiagonalRight,
                (true, true) => TilemapTileSpriteVariant::WaterShoreTopDiagonalLeftRight,
            },
            (false, true, false, false) => match (ground_top_left, ground_bottom_left) {
                (false, false) => TilemapTileSpriteVariant::WaterShoreRight,
                (true, false) => TilemapTileSpriteVariant::WaterShoreRightDiagonalTop,
                (false, true) => TilemapTileSpriteVariant::WaterShoreRightDiagonalBottom,
                (true, true) => TilemapTileSpriteVariant::WaterShoreRightDiagonalTopBottom,
            },
            (false, false, true, false) => match (ground_top_left, ground_top_right) {
                (false, false) => TilemapTileSpriteVariant::WaterShoreBottom,
                (true, false) => TilemapTileSpriteVariant::WaterShoreBottomDiagonalLeft,
                (false, true) => TilemapTileSpriteVariant::WaterShoreBottomDiagonalRight,
                (true, true) => TilemapTileSpriteVariant::WaterShoreBottomDiagonalLeftRight,
            },
            (false, false, false, true) => match (ground_top_right, ground_bottom_right) {
                (false, false) => TilemapTileSpriteVariant::WaterShoreLeft,
                (true, false) => TilemapTileSpriteVariant::WaterShoreLeftDiagonalTop,
                (false, true) => TilemapTileSpriteVariant::WaterShoreLeftDiagonalBottom,
                (true, true) => TilemapTileSpriteVariant::WaterShoreLeftDiagonalTopBottom,
            },

            (true, false, true, false) => TilemapTileSpriteVariant::WaterShoreTopBottom,
            (false, true, false, true) => TilemapTileSpriteVariant::WaterShoreLeftRight,

            (true, false, false, true) => {
                if ground_bottom_right {
                    TilemapTileSpriteVariant::WaterShoreTopLeftDiagonalBottomRight
                } else {
                    TilemapTileSpriteVariant::WaterShoreTopLeft
                }
            }
            (true, true, false, false) => {
                if ground_bottom_left {
                    TilemapTileSpriteVariant::WaterShoreTopRightDiagonalBottomLeft
                } else {
                    TilemapTileSpriteVariant::WaterShoreTopRight
                }
            }
            (false, false, true, true) => {
                if ground_top_right {
                    TilemapTileSpriteVariant::WaterShoreBottomLeftDiagonalTopRight
                } else {
                    TilemapTileSpriteVariant::WaterShoreBottomLeft
                }
            }
            (false, true, true, false) => {
                if ground_top_left {
                    TilemapTileSpriteVariant::WaterShoreBottomRightDiagonalTopLeft
                } else {
                    TilemapTileSpriteVariant::WaterShoreBottomRight
                }
            }

            (true, true, false, true) => TilemapTileSpriteVariant::WaterShoreTopLeftRight,
            (true, true, true, false) => TilemapTileSpriteVariant::WaterShoreRightTopBottom,
            (false, true, true, true) => TilemapTileSpriteVariant::WaterShoreBottomLeftRight,
            (true, false, true, true) => TilemapTileSpriteVariant::WaterShoreLeftTopBottom,
            (true, true, true, true) => TilemapTileSpriteVariant::WaterShoreTopRightBottomLeft,

            _ => match (
                ground_top_left,
                ground_top_right,
                ground_bottom_left,
                ground_bottom_right,
            ) {
                (true, false, false, false) => TilemapTileSpriteVariant::WaterShoreDiagonalTopLeft,
                (false, true, false, false) => TilemapTileSpriteVariant::WaterShoreDiagonalTopRight,
                (false, false, true, false) => {
                    TilemapTileSpriteVariant::WaterShoreDiagonalBottomLeft
                }
                (false, false, false, true) => {
                    TilemapTileSpriteVariant::WaterShoreDiagonalBottomRight
                }

                (true, true, false, false) => {
                    TilemapTileSpriteVariant::WaterShoreDiagonalTopLeftRight
                }
                (false, true, false, true) => {
                    TilemapTileSpriteVariant::WaterShoreDiagonalRightTopBottom
                }
                (false, false, true, true) => {
                    TilemapTileSpriteVariant::WaterShoreDiagonalBottomLeftRight
                }
                (true, false, true, false) => {
                    TilemapTileSpriteVariant::WaterShoreDiagonalLeftTopBottom
                }

                (true, false, false, true) => {
                    TilemapTileSpriteVariant::WaterShoreDiagonalTopLeftDiagonalBottomRight
                }
                (false, true, true, false) => {
                    TilemapTileSpriteVariant::WaterShoreDiagonalTopRightDiagonalBottomLeft
                }

                _ => TilemapTileSpriteVariant::Water,
            },
        }
    }
}
