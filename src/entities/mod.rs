// pub mod entity_shadow;
pub mod projectile;
pub mod structure;
pub mod tilemap;
pub mod unit;

use bevy::prelude::*;
use projectile::ProjectilePlugin;
use structure::StructurePlugin;
use tilemap::{tile::TilePligin, TilemapPlugin};
use unit::UnitPlugin;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TilemapPlugin,
            TilePligin,
            StructurePlugin,
            ProjectilePlugin,
            UnitPlugin,
        ));
    }
}
