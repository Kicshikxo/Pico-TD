pub mod indicator;
pub mod movement;
pub mod position;
pub mod sprite;

use bevy::prelude::*;

use crate::game::entities::tile::{
    indicator::TileIndicatorPlugin, movement::TileMovementPlugin, position::TilePositionPlugin,
    sprite::TileSpritePlugin,
};

pub struct TilePligin;

impl Plugin for TilePligin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TilePositionPlugin,
            TileSpritePlugin,
            TileMovementPlugin,
            TileIndicatorPlugin,
        ));
    }
}
