use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::Rng;

use crate::entities::tilemap::tile::TilemapTileVariant;

#[derive(AssetCollection, Resource)]
#[allow(unused)]
pub struct TilemapTileAssets {
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 18, rows = 11))]
    pub forest_tilemap_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "embedded://images/tiles/forest_tilemap.png")]
    pub forest_tilemap: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 12, rows = 11))]
    pub show_tilemap_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "embedded://images/tiles/snow_tilemap.png")]
    pub show_tilemap: Handle<Image>,
}

pub enum TilemapTileRoadVariant {
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

pub enum TilemapTileWaterVariant {
    Water = 37,
    // WaterShoreTop = 19,
    // WaterShoreRight = 38,
    // WaterShoreDown = 55,
    // WaterShoreLeft = 36,

    // WaterShoreTopDown = 73,
    // WaterShoreLeftRight = 57,

    // WaterShoreTopLeft = 18,
    // WaterShoreTopRight = 20,
    // WaterShoreDownLeft = 54,
    // WaterShoreDownRight = 56,

    // WaterShoreTopLeftRight = 23,
    // WaterShoreRightTopDown = 58,
    // WaterShoreDownLeftRight = 5,
    // WaterShoreLeftTopDown = 74,
}

pub enum TilemapTileGroundVariant {
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

impl TilemapTileAssets {
    pub fn get_road_tile_index(
        &self,
        tiles_around: [[TilemapTileVariant; 3]; 3],
    ) -> TilemapTileRoadVariant {
        let road_top = tiles_around[0][1] == TilemapTileVariant::Road;
        let road_right = tiles_around[1][2] == TilemapTileVariant::Road;
        let road_down = tiles_around[2][1] == TilemapTileVariant::Road;
        let road_left = tiles_around[1][0] == TilemapTileVariant::Road;

        match (road_top, road_right, road_down, road_left) {
            (false, false, false, false) => TilemapTileRoadVariant::Road,

            (true, false, false, false) => TilemapTileRoadVariant::RoadTop,
            (false, true, false, false) => TilemapTileRoadVariant::RoadRight,
            (false, false, true, false) => TilemapTileRoadVariant::RoadDown,
            (false, false, false, true) => TilemapTileRoadVariant::RoadLeft,

            (true, false, true, false) => TilemapTileRoadVariant::RoadTopDown,
            (false, true, false, true) => TilemapTileRoadVariant::RoadLeftRight,

            (true, false, false, true) => TilemapTileRoadVariant::RoadTopLeft,
            (true, true, false, false) => TilemapTileRoadVariant::RoadTopRight,
            (false, false, true, true) => TilemapTileRoadVariant::RoadDownLeft,
            (false, true, true, false) => TilemapTileRoadVariant::RoadDownRight,

            (true, true, false, true) => TilemapTileRoadVariant::RoadTopLeftRight,
            (true, true, true, false) => TilemapTileRoadVariant::RoadRightTopDown,
            (false, true, true, true) => TilemapTileRoadVariant::RoadDownLeftRight,
            (true, false, true, true) => TilemapTileRoadVariant::RoadLeftTopDown,

            (true, true, true, true) => TilemapTileRoadVariant::RoadTopRightDownLeft,
        }
    }
    #[allow(unused)]
    pub fn get_water_tile_index(
        &self,
        tiles_around: [[TilemapTileVariant; 3]; 3],
    ) -> TilemapTileWaterVariant {
        return TilemapTileWaterVariant::Water;

        // let [[ground_top_left, ground_top, ground_top_right], [ground_left, _, ground_right], [ground_bottom_left, ground_bottom, ground_bottom_right]] =
        //     tiles_around.map(|row| {
        //         row.map(|tile| match tile {
        //             TilemapTileVariant::Ground => true,
        //             TilemapTileVariant::Road => true,
        //             _ => false,
        //         })
        //     });

        // match (ground_top, ground_right, ground_bottom, ground_left) {
        //     (false, false, false, false) => TileWaterVariant::Water,

        //     (true, false, false, false) => TileWaterVariant::WaterShoreTop,
        //     (false, true, false, false) => TileWaterVariant::WaterShoreRight,
        //     (false, false, true, false) => TileWaterVariant::WaterShoreDown,
        //     (false, false, false, true) => TileWaterVariant::WaterShoreLeft,

        //     (true, false, true, false) => TileWaterVariant::WaterShoreTopDown,
        //     (false, true, false, true) => TileWaterVariant::WaterShoreLeftRight,

        //     (true, false, false, true) => TileWaterVariant::WaterShoreTopLeft,
        //     (true, true, false, false) => TileWaterVariant::WaterShoreTopRight,
        //     (false, false, true, true) => TileWaterVariant::WaterShoreDownLeft,
        //     (false, true, true, false) => TileWaterVariant::WaterShoreDownRight,

        //     // (true, true, false, true) => TileWaterVariant::RoadTopLeftRight,
        //     // (true, true, true, false) => TileWaterVariant::RoadRightTopDown,
        //     // (false, true, true, true) => TileWaterVariant::RoadDownLeftRight,
        //     // (true, false, true, true) => TileWaterVariant::RoadLeftTopDown,

        //     // (true, true, true, true) => TileWaterVariant::RoadTopRightDownLeft,
        //     _ => TileWaterVariant::Water,
        // }
    }
    pub fn get_ground_tile_index(
        &self,
        tiles_around: [[TilemapTileVariant; 3]; 3],
    ) -> TilemapTileGroundVariant {
        let [[water_top_left, water_top, water_top_right], [water_left, _, water_right], [water_bottom_left, water_bottom, water_bottom_right]] =
            tiles_around.map(|row| row.map(|tile| tile == TilemapTileVariant::Water));

        // let water_top_left = tiles_around[0][0] == TilemapTileVariant::Water;
        // let water_top = tiles_around[0][1] == TilemapTileVariant::Water;
        // let water_top_right = tiles_around[0][2] == TilemapTileVariant::Water;
        // let water_left = tiles_around[1][0] == TilemapTileVariant::Water;
        // let water_right = tiles_around[1][2] == TilemapTileVariant::Water;
        // let water_bottom_left = tiles_around[2][0] == TilemapTileVariant::Water;
        // let water_bottom = tiles_around[2][1] == TilemapTileVariant::Water;
        // let water_bottom_right = tiles_around[2][2] == TilemapTileVariant::Water;

        match (water_top, water_right, water_bottom, water_left) {
            (false, false, true, false) => TilemapTileGroundVariant::GroundShoreTop,
            (false, false, false, true) => TilemapTileGroundVariant::GroundShoreRight,
            (true, false, false, false) => TilemapTileGroundVariant::GroundShoreBottom,
            (false, true, false, false) => TilemapTileGroundVariant::GroundShoreLeft,

            (false, true, true, false) => TilemapTileGroundVariant::GroundShoreTopLeft,
            (false, false, true, true) => TilemapTileGroundVariant::GroundShoreTopRight,
            (true, true, false, false) => TilemapTileGroundVariant::GroundShoreBottomLeft,
            (true, false, false, true) => TilemapTileGroundVariant::GroundShoreBottomRight,

            // (true, true, true, true) => &self.ground_shore_left_top_right_bottom,
            _ => match (
                water_top_left,
                water_top_right,
                water_bottom_left,
                water_bottom_right,
            ) {
                (false, false, false, true) => TilemapTileGroundVariant::GroundShoreTopLeftDiagonal,
                (false, false, true, false) => {
                    TilemapTileGroundVariant::GroundShoreTopRightDiagonal
                }
                (false, true, false, false) => {
                    TilemapTileGroundVariant::GroundShoreBottomLeftDiagonal
                }
                (true, false, false, false) => {
                    TilemapTileGroundVariant::GroundShoreBottomRightDiagonal
                }

                // (true, false, false, true) => &self.ground_shore_top_left_bottom_right_diagonal,
                // (false, true, true, false) => &self.ground_shore_top_right_bottom_left_diagonal,
                _ => {
                    if rand::thread_rng().gen_bool(1.0 / 5.0) {
                        if rand::thread_rng().gen_bool(1.0 / 5.0) {
                            TilemapTileGroundVariant::GroundWithFlowers
                        } else {
                            TilemapTileGroundVariant::GroundWithGrass
                        }
                    } else {
                        TilemapTileGroundVariant::Ground
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
