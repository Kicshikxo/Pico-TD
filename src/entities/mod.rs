pub mod enemy;
pub mod soldier;
pub mod tile;
pub mod tilemap;

use bevy::prelude::*;
use enemy::EnemyPlugin;
use soldier::SoldierPlugin;
use tile::TilePligin;
use tilemap::TilemapPlugin;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TilemapPlugin, TilePligin, SoldierPlugin, EnemyPlugin));
    }
}
