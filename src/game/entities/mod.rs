pub mod enemy;
pub mod soldier;
pub mod tile;
pub mod tilemap;

use bevy::prelude::*;

use crate::game::entities::{
    enemy::EnemyPlugin, soldier::SoldierPlugin, tile::TilePligin, tilemap::TilemapPlugin,
};

pub struct GameEntitiesPlugin;

impl Plugin for GameEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TilemapPlugin, TilePligin, SoldierPlugin, EnemyPlugin));
    }
}
