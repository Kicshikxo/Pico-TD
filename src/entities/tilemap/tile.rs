use bevy::prelude::*;

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
#[require(Transform)]
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

impl From<char> for TilemapTile {
    fn from(tile: char) -> Self {
        Self {
            variant: match tile {
                '#' => TilemapTileVariant::Ground,
                'F' => TilemapTileVariant::Flower,
                'T' => TilemapTileVariant::Tree,
                '.' => TilemapTileVariant::Road,
                '~' => TilemapTileVariant::Water,
                _ => TilemapTileVariant::Unknown,
            },
            ..default()
        }
    }
}

impl TilemapTile {
    pub fn get_variant(&self) -> TilemapTileVariant {
        self.variant
    }
    #[allow(unused)]
    pub fn set_variant(&mut self, variant: TilemapTileVariant) {
        self.variant = variant;
    }
}
