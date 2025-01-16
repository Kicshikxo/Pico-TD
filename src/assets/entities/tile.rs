use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::Rng;

use crate::entities::tilemap::tile::TileVariant;

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 18, rows = 11))]
    pub forest_tilemap_atlas: Handle<TextureAtlasLayout>,
    #[asset(path = "embedded://images/tiles/forest_tilemap.png")]
    pub forest_tilemap: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 12, rows = 11))]
    pub show_tilemap_atlas: Handle<TextureAtlasLayout>,
    #[asset(path = "embedded://images/tiles/snow_tilemap.png")]
    pub show_tilemap: Handle<Image>,
}

pub enum TileRoadVariant {
    Road = 108,

    RoadTop = 162,
    RoadRight = 109,
    RoadDown = 126,
    RoadLeft = 111,

    RoadTopDown = 144,
    RoadLeftRight = 110,

    RoadTopLeft = 165,
    RoadTopRight = 163,
    RoadDownLeft = 129,
    RoadDownRight = 127,

    RoadTopLeftRight = 164,
    RoadRightTopDown = 145,
    RoadDownLeftRight = 128,
    RoadLeftTopDown = 147,

    RoadTopRightDownLeft = 146,
}

pub enum TileWaterVariant {
    Water = 37,

    WaterShoreTop = 19,
    WaterShoreRight = 38,
    WaterShoreDown = 55,
    WaterShoreLeft = 36,

    WaterShoreTopDown = 73,
    WaterShoreLeftRight = 57,

    WaterShoreTopLeft = 18,
    WaterShoreTopRight = 20,
    WaterShoreDownLeft = 54,
    WaterShoreDownRight = 56,

    WaterShoreTopLeftRight = 23,
    WaterShoreRightTopDown = 58,
    WaterShoreDownLeftRight = 5,
    WaterShoreLeftTopDown = 74,
}

pub enum TileGroundVariant {
    Ground = 0,
    GroundWithGrass = 1,
    GroundWithFlowers = 2,

    GroundShoreTop = 19,
    GroundShoreRight = 38,
    GroundShoreBottom = 55,
    GroundShoreLeft = 36,

    // GroundShoreTopDown = 73,
    // GroundShoreLeftRight = 57,
    GroundShoreTopLeft = 90,
    GroundShoreTopRight = 91,
    GroundShoreBottomLeft = 93,
    GroundShoreBottomRight = 92,

    GroundShoreTopLeftDiagonal = 18,
    GroundShoreTopRightDiagonal = 20,
    GroundShoreBottomLeftDiagonal = 54,
    GroundShoreBottomRightDiagonal = 56,
    // GroundShoreTopLeftBottomRightDiagonal = 94,
    // GroundShoreTopRightBottomLeftDiagonal = 95,
}

impl TileAssets {
    pub fn get_road_tile_index(&self, tiles_around: [[TileVariant; 3]; 3]) -> TileRoadVariant {
        let road_top = tiles_around[0][1] == TileVariant::Road;
        let road_right = tiles_around[1][2] == TileVariant::Road;
        let road_down = tiles_around[2][1] == TileVariant::Road;
        let road_left = tiles_around[1][0] == TileVariant::Road;

        match (road_top, road_right, road_down, road_left) {
            (false, false, false, false) => TileRoadVariant::Road,

            (true, false, false, false) => TileRoadVariant::RoadTop,
            (false, true, false, false) => TileRoadVariant::RoadRight,
            (false, false, true, false) => TileRoadVariant::RoadDown,
            (false, false, false, true) => TileRoadVariant::RoadLeft,

            (true, false, true, false) => TileRoadVariant::RoadTopDown,
            (false, true, false, true) => TileRoadVariant::RoadLeftRight,

            (true, false, false, true) => TileRoadVariant::RoadTopLeft,
            (true, true, false, false) => TileRoadVariant::RoadTopRight,
            (false, false, true, true) => TileRoadVariant::RoadDownLeft,
            (false, true, true, false) => TileRoadVariant::RoadDownRight,

            (true, true, false, true) => TileRoadVariant::RoadTopLeftRight,
            (true, true, true, false) => TileRoadVariant::RoadRightTopDown,
            (false, true, true, true) => TileRoadVariant::RoadDownLeftRight,
            (true, false, true, true) => TileRoadVariant::RoadLeftTopDown,

            (true, true, true, true) => TileRoadVariant::RoadTopRightDownLeft,
        }
    }
    pub fn get_water_tile_index(&self, tiles_around: [[TileVariant; 3]; 3]) -> TileWaterVariant {
        return TileWaterVariant::Water;

        let [[ground_top_left, ground_top, ground_top_right], [ground_left, _, ground_right], [ground_bottom_left, ground_bottom, ground_bottom_right]] =
            tiles_around.map(|row| {
                row.map(|tile| match tile {
                    TileVariant::Ground => true,
                    TileVariant::Road => true,
                    _ => false,
                })
            });

        match (ground_top, ground_right, ground_bottom, ground_left) {
            (false, false, false, false) => TileWaterVariant::Water,

            (true, false, false, false) => TileWaterVariant::WaterShoreTop,
            (false, true, false, false) => TileWaterVariant::WaterShoreRight,
            (false, false, true, false) => TileWaterVariant::WaterShoreDown,
            (false, false, false, true) => TileWaterVariant::WaterShoreLeft,

            (true, false, true, false) => TileWaterVariant::WaterShoreTopDown,
            (false, true, false, true) => TileWaterVariant::WaterShoreLeftRight,

            (true, false, false, true) => TileWaterVariant::WaterShoreTopLeft,
            (true, true, false, false) => TileWaterVariant::WaterShoreTopRight,
            (false, false, true, true) => TileWaterVariant::WaterShoreDownLeft,
            (false, true, true, false) => TileWaterVariant::WaterShoreDownRight,

            // (true, true, false, true) => TileWaterVariant::RoadTopLeftRight,
            // (true, true, true, false) => TileWaterVariant::RoadRightTopDown,
            // (false, true, true, true) => TileWaterVariant::RoadDownLeftRight,
            // (true, false, true, true) => TileWaterVariant::RoadLeftTopDown,

            // (true, true, true, true) => TileWaterVariant::RoadTopRightDownLeft,
            _ => TileWaterVariant::Water,
        }
    }
    pub fn get_ground_tile_index(&self, tiles_around: [[TileVariant; 3]; 3]) -> TileGroundVariant {
        let [[water_top_left, water_top, water_top_right], [water_left, _, water_right], [water_bottom_left, water_bottom, water_bottom_right]] =
            tiles_around.map(|row| row.map(|tile| tile == TileVariant::Water));

        // let water_top_left = tiles_around[0][0] == TileVariant::Water;
        // let water_top = tiles_around[0][1] == TileVariant::Water;
        // let water_top_right = tiles_around[0][2] == TileVariant::Water;
        // let water_left = tiles_around[1][0] == TileVariant::Water;
        // let water_right = tiles_around[1][2] == TileVariant::Water;
        // let water_bottom_left = tiles_around[2][0] == TileVariant::Water;
        // let water_bottom = tiles_around[2][1] == TileVariant::Water;
        // let water_bottom_right = tiles_around[2][2] == TileVariant::Water;

        match (water_top, water_right, water_bottom, water_left) {
            (false, false, true, false) => TileGroundVariant::GroundShoreTop,
            (false, false, false, true) => TileGroundVariant::GroundShoreRight,
            (true, false, false, false) => TileGroundVariant::GroundShoreBottom,
            (false, true, false, false) => TileGroundVariant::GroundShoreLeft,

            (false, true, true, false) => TileGroundVariant::GroundShoreTopLeft,
            (false, false, true, true) => TileGroundVariant::GroundShoreTopRight,
            (true, true, false, false) => TileGroundVariant::GroundShoreBottomLeft,
            (true, false, false, true) => TileGroundVariant::GroundShoreBottomRight,

            // (true, true, true, true) => &self.ground_shore_left_top_right_bottom,
            _ => match (
                water_top_left,
                water_top_right,
                water_bottom_left,
                water_bottom_right,
            ) {
                (false, false, false, true) => TileGroundVariant::GroundShoreTopLeftDiagonal,
                (false, false, true, false) => TileGroundVariant::GroundShoreTopRightDiagonal,
                (false, true, false, false) => TileGroundVariant::GroundShoreBottomLeftDiagonal,
                (true, false, false, false) => TileGroundVariant::GroundShoreBottomRightDiagonal,

                // (true, false, false, true) => &self.ground_shore_top_left_bottom_right_diagonal,
                // (false, true, true, false) => &self.ground_shore_top_right_bottom_left_diagonal,
                _ => {
                    if rand::thread_rng().gen_bool(1.0 / 5.0) {
                        if rand::thread_rng().gen_bool(1.0 / 5.0) {
                            TileGroundVariant::GroundWithFlowers
                        } else {
                            TileGroundVariant::GroundWithGrass
                        }
                    } else {
                        TileGroundVariant::Ground
                    }
                }
            },
        }
    }
    // pub fn get_ground(&self, water_around: [[bool; 3]; 3]) -> &Handle<Image> {
    //     let [[water_top_left, water_top, water_top_right], [water_left, _, water_right], [water_bottom_left, water_bottom, water_bottom_right]] =
    //         water_around;

    //     match (water_left, water_top, water_right, water_bottom) {
    //         (true, false, false, false) => &self.ground_shore_left,
    //         (false, true, false, false) => &self.ground_shore_top,
    //         (false, false, true, false) => &self.ground_shore_right,
    //         (false, false, false, true) => &self.ground_shore_bottom,

    //         (true, true, false, false) => &self.ground_shore_top_left,
    //         (false, true, true, false) => &self.ground_shore_top_right,
    //         (false, false, true, true) => &self.ground_shore_bottom_right,
    //         (true, false, false, true) => &self.ground_shore_bottom_left,

    //         (true, true, true, true) => &self.ground_shore_left_top_right_bottom,

    //         _ => match (
    //             water_top_left,
    //             water_top_right,
    //             water_bottom_left,
    //             water_bottom_right,
    //         ) {
    //             (true, false, false, false) => &self.ground_shore_top_left_diagonal,
    //             (false, true, false, false) => &self.ground_shore_top_right_diagonal,
    //             (false, false, true, false) => &self.ground_shore_bottom_left_diagonal,
    //             (false, false, false, true) => &self.ground_shore_bottom_right_diagonal,

    //             (true, false, false, true) => &self.ground_shore_top_left_bottom_right_diagonal,
    //             (false, true, true, false) => &self.ground_shore_top_right_bottom_left_diagonal,

    //             _ => &self.ground,
    //         },
    //     }
    // }
}
