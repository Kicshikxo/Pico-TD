pub mod structure;
pub mod tile;
pub mod tilemap;
pub mod unit;

use bevy::prelude::*;
use structure::StructurePlugin;
use tile::TilePligin;
use tilemap::TilemapPlugin;
use unit::UnitPlugin;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TilemapPlugin, TilePligin, StructurePlugin, UnitPlugin));
    }
}
