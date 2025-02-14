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

impl TilemapTileVariant {
    pub fn get_preview_color(&self) -> Color {
        match self {
            TilemapTileVariant::Ground => Color::srgb(132.0 / 255.0, 198.0 / 255.0, 105.0 / 255.0),
            TilemapTileVariant::Flower => Color::srgb(179.0 / 255.0, 195.0 / 255.0, 104.0 / 255.0),
            TilemapTileVariant::Tree => Color::srgb(67.0 / 255.0, 149.0 / 255.0, 69.0 / 255.0),
            TilemapTileVariant::Road => Color::srgb(82.0 / 255.0, 96.0 / 255.0, 124.0 / 255.0),
            TilemapTileVariant::Water => Color::srgb(117.0 / 255.0, 227.0 / 255.0, 255.0 / 255.0),
            _ => Color::srgba(0.0, 0.0, 0.0, 0.0),
        }
    }
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
