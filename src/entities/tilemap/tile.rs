use bevy::prelude::*;

use crate::game::{GameState, MainTilemap};

use super::{position::TilePosition, Tilemap};

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(unused)]
pub enum TileVariant {
    Ground,
    Road,
    Water,
    Unknown,
}

#[derive(Component, Clone, Copy, Debug)]
#[require(Sprite, Transform)]
pub struct Tile {
    pub variant: TileVariant,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            variant: TileVariant::Unknown,
        }
    }
}

impl From<char> for Tile {
    fn from(tile: char) -> Self {
        Self {
            variant: match tile {
                '#' => TileVariant::Ground,
                '.' => TileVariant::Road,
                '~' => TileVariant::Water,
                _ => TileVariant::Unknown,
            },
            ..default()
        }
    }
}

impl Tile {}

pub struct TilePligin;

impl Plugin for TilePligin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_tile_position.run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_tile_position(
    main_tilemap: Query<&Tilemap, With<MainTilemap>>,
    mut tile_positions: Query<(&mut TilePosition, &mut Transform)>,
) {
    let Ok(tilemap) = main_tilemap.get_single() else {
        return;
    };
    for (mut tile_position, mut position_transform) in tile_positions.iter_mut() {
        if tile_position.get_need_update() == false {
            continue;
        }

        position_transform.translation.x =
            tile_position.get_x() as f32 * tilemap.tile_size.x as f32;
        position_transform.translation.y =
            (tile_position.get_y() as f32 * tilemap.tile_size.y as f32) * -1.0
                + tilemap.get_size().y as f32 * tilemap.tile_size.y as f32
                - tilemap.tile_size.y as f32;

        tile_position.set_need_update(false);
    }
}
