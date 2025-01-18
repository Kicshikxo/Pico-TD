use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(unused)]
pub enum TilemapTileVariant {
    Ground,
    Road,
    Water,
    Unknown,
}

#[derive(Component, Clone, Copy, Debug)]
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
                '.' => TilemapTileVariant::Road,
                '~' => TilemapTileVariant::Water,
                _ => TilemapTileVariant::Unknown,
            },
            ..default()
        }
    }
}

#[allow(unused)]
impl TilemapTile {
    pub fn get_variant(&self) -> TilemapTileVariant {
        self.variant
    }
    pub fn set_variant(&mut self, variant: TilemapTileVariant) {
        self.variant = variant;
    }
}
