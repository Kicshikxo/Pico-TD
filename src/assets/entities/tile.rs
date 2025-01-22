use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::Rng;

use crate::entities::tilemap::tile::TilemapTileVariant;

#[derive(AssetCollection, Resource)]
pub struct TilemapTileAssets {
    #[asset(path = "embedded://images/tiles/tilemap_tiles.png")]
    pub tilemap: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 8, rows = 10))]
    pub tilemap_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "embedded://images/tiles/entities.png")]
    pub entities: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 12, rows = 11))]
    pub entities_layout: Handle<TextureAtlasLayout>,
}

pub enum TilemapTileRoadVariant {
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
}

pub enum TilemapTileWaterVariant {
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

pub enum TilemapTileGroundVariant {
    Ground = 1,
    GroundWithGrass = 2,
    GroundWithFlowers = 3,
    GroundWithTree = 4,
    GroundWithDoubleTree = 5,
}

impl TilemapTileAssets {
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
                    TilemapTileGroundVariant::GroundWithGrass as usize
                } else {
                    TilemapTileGroundVariant::Ground as usize
                }
            }
            TilemapTileVariant::Flower => TilemapTileGroundVariant::GroundWithFlowers as usize,
            TilemapTileVariant::Tree => {
                if rand::thread_rng().gen_bool(0.25) {
                    TilemapTileGroundVariant::GroundWithDoubleTree as usize
                } else {
                    TilemapTileGroundVariant::GroundWithTree as usize
                }
            }
            _ => 0,
        }
    }

    pub fn get_road_tile_index(
        &self,
        tiles_around: [[TilemapTileVariant; 3]; 3],
    ) -> TilemapTileRoadVariant {
        let road_top = tiles_around[0][1] == TilemapTileVariant::Road;
        let road_right = tiles_around[1][2] == TilemapTileVariant::Road;
        let road_bottom = tiles_around[2][1] == TilemapTileVariant::Road;
        let road_left = tiles_around[1][0] == TilemapTileVariant::Road;

        match (road_top, road_right, road_bottom, road_left) {
            (false, false, false, false) => TilemapTileRoadVariant::Road,

            (true, false, false, false) => TilemapTileRoadVariant::RoadTop,
            (false, true, false, false) => TilemapTileRoadVariant::RoadRight,
            (false, false, true, false) => TilemapTileRoadVariant::RoadBottom,
            (false, false, false, true) => TilemapTileRoadVariant::RoadLeft,

            (true, false, true, false) => TilemapTileRoadVariant::RoadTopBottom,
            (false, true, false, true) => TilemapTileRoadVariant::RoadLeftRight,

            (true, false, false, true) => TilemapTileRoadVariant::RoadTopLeft,
            (true, true, false, false) => TilemapTileRoadVariant::RoadTopRight,
            (false, false, true, true) => TilemapTileRoadVariant::RoadBottomLeft,
            (false, true, true, false) => TilemapTileRoadVariant::RoadBottomRight,

            (true, true, false, true) => TilemapTileRoadVariant::RoadTopLeftRight,
            (true, true, true, false) => TilemapTileRoadVariant::RoadRightTopBottom,
            (false, true, true, true) => TilemapTileRoadVariant::RoadBottomLeftRight,
            (true, false, true, true) => TilemapTileRoadVariant::RoadLeftTopBottom,

            (true, true, true, true) => TilemapTileRoadVariant::RoadTopRightBottomLeft,
        }
    }

    pub fn get_water_tile_index(
        &self,
        tiles_around: [[TilemapTileVariant; 3]; 3],
    ) -> TilemapTileWaterVariant {
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
                (false, false) => TilemapTileWaterVariant::WaterShoreTop,
                (true, false) => TilemapTileWaterVariant::WaterShoreTopDiagonalLeft,
                (false, true) => TilemapTileWaterVariant::WaterShoreTopDiagonalRight,
                (true, true) => TilemapTileWaterVariant::WaterShoreTopDiagonalLeftRight,
            },
            (false, true, false, false) => match (ground_top_left, ground_bottom_left) {
                (false, false) => TilemapTileWaterVariant::WaterShoreRight,
                (true, false) => TilemapTileWaterVariant::WaterShoreRightDiagonalTop,
                (false, true) => TilemapTileWaterVariant::WaterShoreRightDiagonalBottom,
                (true, true) => TilemapTileWaterVariant::WaterShoreRightDiagonalTopBottom,
            },
            (false, false, true, false) => match (ground_top_left, ground_top_right) {
                (false, false) => TilemapTileWaterVariant::WaterShoreBottom,
                (true, false) => TilemapTileWaterVariant::WaterShoreBottomDiagonalLeft,
                (false, true) => TilemapTileWaterVariant::WaterShoreBottomDiagonalRight,
                (true, true) => TilemapTileWaterVariant::WaterShoreBottomDiagonalLeftRight,
            },
            (false, false, false, true) => match (ground_top_right, ground_bottom_right) {
                (false, false) => TilemapTileWaterVariant::WaterShoreLeft,
                (true, false) => TilemapTileWaterVariant::WaterShoreLeftDiagonalTop,
                (false, true) => TilemapTileWaterVariant::WaterShoreLeftDiagonalBottom,
                (true, true) => TilemapTileWaterVariant::WaterShoreLeftDiagonalTopBottom,
            },

            (true, false, true, false) => TilemapTileWaterVariant::WaterShoreTopBottom,
            (false, true, false, true) => TilemapTileWaterVariant::WaterShoreLeftRight,

            (true, false, false, true) => {
                if ground_bottom_right {
                    TilemapTileWaterVariant::WaterShoreTopLeftDiagonalBottomRight
                } else {
                    TilemapTileWaterVariant::WaterShoreTopLeft
                }
            }
            (true, true, false, false) => {
                if ground_bottom_left {
                    TilemapTileWaterVariant::WaterShoreTopRightDiagonalBottomLeft
                } else {
                    TilemapTileWaterVariant::WaterShoreTopRight
                }
            }
            (false, false, true, true) => {
                if ground_top_right {
                    TilemapTileWaterVariant::WaterShoreBottomLeftDiagonalTopRight
                } else {
                    TilemapTileWaterVariant::WaterShoreBottomLeft
                }
            }
            (false, true, true, false) => {
                if ground_top_left {
                    TilemapTileWaterVariant::WaterShoreBottomRightDiagonalTopLeft
                } else {
                    TilemapTileWaterVariant::WaterShoreBottomRight
                }
            }

            (true, true, false, true) => TilemapTileWaterVariant::WaterShoreTopLeftRight,
            (true, true, true, false) => TilemapTileWaterVariant::WaterShoreRightTopBottom,
            (false, true, true, true) => TilemapTileWaterVariant::WaterShoreBottomLeftRight,
            (true, false, true, true) => TilemapTileWaterVariant::WaterShoreLeftTopBottom,
            (true, true, true, true) => TilemapTileWaterVariant::WaterShoreTopRightBottomLeft,

            _ => match (
                ground_top_left,
                ground_top_right,
                ground_bottom_left,
                ground_bottom_right,
            ) {
                (true, false, false, false) => TilemapTileWaterVariant::WaterShoreDiagonalTopLeft,
                (false, true, false, false) => TilemapTileWaterVariant::WaterShoreDiagonalTopRight,
                (false, false, true, false) => {
                    TilemapTileWaterVariant::WaterShoreDiagonalBottomLeft
                }
                (false, false, false, true) => {
                    TilemapTileWaterVariant::WaterShoreDiagonalBottomRight
                }

                (true, true, false, false) => {
                    TilemapTileWaterVariant::WaterShoreDiagonalTopLeftRight
                }
                (false, true, false, true) => {
                    TilemapTileWaterVariant::WaterShoreDiagonalRightTopBottom
                }
                (false, false, true, true) => {
                    TilemapTileWaterVariant::WaterShoreDiagonalBottomLeftRight
                }
                (true, false, true, false) => {
                    TilemapTileWaterVariant::WaterShoreDiagonalLeftTopBottom
                }

                (true, false, false, true) => {
                    TilemapTileWaterVariant::WaterShoreDiagonalTopLeftDiagonalBottomRight
                }
                (false, true, true, false) => {
                    TilemapTileWaterVariant::WaterShoreDiagonalTopRightDiagonalBottomLeft
                }

                _ => TilemapTileWaterVariant::Water,
            },
        }
    }
}
