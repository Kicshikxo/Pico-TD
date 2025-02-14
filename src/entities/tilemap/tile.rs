use bevy::prelude::*;

use crate::entities::tile::position::TilePosition;

#[derive(Clone, Copy, PartialEq)]
pub enum TilemapTileVariant {
    Ground,
    Flower,
    Tree,
    Road,
    Water,
    Unknown,
}

#[derive(Component, Clone, Copy)]
#[require(TilePosition)]
pub struct TilemapTile {
    variant: TilemapTileVariant,
}

impl Default for TilemapTile {
    fn default() -> Self {
        Self {
            variant: TilemapTileVariant::Unknown,
        }
    }
}

impl TilemapTile {
    pub fn new(variant: TilemapTileVariant) -> Self {
        Self { variant }
    }
    pub fn get_variant(&self) -> TilemapTileVariant {
        self.variant
    }
    #[allow(unused)]
    pub fn set_variant(&mut self, variant: TilemapTileVariant) {
        self.variant = variant;
    }
}
