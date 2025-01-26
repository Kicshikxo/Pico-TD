pub mod indicator;
pub mod movement;
pub mod position;
pub mod sprite;

use bevy::prelude::*;

use indicator::TileIndicatorPlugin;
use movement::TileMovementPlugin;
use position::TilePositionPlugin;
use sprite::TileSpritePlugin;

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
