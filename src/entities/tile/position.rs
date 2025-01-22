use bevy::prelude::*;

use crate::{
    entities::tilemap::Tilemap,
    game::{GameState, GameTilemap},
};

#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[require(Transform)]
pub struct TilePosition {
    x: f32,
    y: f32,
    update_required: bool,
}

impl Default for TilePosition {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            update_required: true,
        }
    }
}

#[allow(unused)]
impl TilePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, ..default() }
    }
    pub fn from_vec2(vec: Vec2) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            ..default()
        }
    }
    pub fn from_ivec2(ivec: IVec2) -> Self {
        Self {
            x: ivec.x as f32,
            y: ivec.y as f32,
            ..default()
        }
    }
    pub fn from_tilemap_position(tilemap: &Tilemap, position: Vec2) -> Self {
        let tile_size_x = tilemap.get_tile_size().x as f32;
        let tile_size_y = tilemap.get_tile_size().y as f32;
        let tilemap_size_y = tilemap.get_size().y as f32;

        Self {
            x: (position.x / tile_size_x).ceil(),
            y: ((tilemap_size_y * tile_size_y - position.y - tile_size_y) / tile_size_y).floor(),
            ..default()
        }
    }
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    pub fn as_ivec2(&self) -> IVec2 {
        IVec2::new(self.x as i32, self.y as i32)
    }
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.update_required = true;
    }
    pub fn set_from_vec2(&mut self, vec: Vec2) {
        self.x = vec.x;
        self.y = vec.y;
        self.update_required = true;
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        self.update_required = true;
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
        self.update_required = true;
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_tilemap_x(&self, tilemap: &Tilemap) -> f32 {
        self.x * tilemap.get_tile_size().x as f32
    }
    pub fn get_tilemap_y(&self, tilemap: &Tilemap) -> f32 {
        let tile_size_y = tilemap.get_tile_size().y as f32;
        let tilemap_size_y = tilemap.get_size().y as f32;

        (tilemap_size_y - self.y) * tile_size_y - tile_size_y
    }
    pub fn get_tilemap_position(&self, tilemap: &Tilemap) -> Vec2 {
        Vec2::new(self.get_tilemap_x(tilemap), self.get_tilemap_y(tilemap))
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
}

pub struct TilePositionPlugin;

impl Plugin for TilePositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_tile_position.run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_tile_position(
    game_tilemap: Query<&Tilemap, With<GameTilemap>>,
    mut tile_positions: Query<(&mut TilePosition, &mut Transform)>,
) {
    let Ok(tilemap) = game_tilemap.get_single() else {
        return;
    };
    for (mut tile_position, mut position_transform) in tile_positions.iter_mut() {
        if tile_position.get_update_required() == false {
            continue;
        }

        position_transform.translation.x = tile_position.get_tilemap_x(tilemap);
        position_transform.translation.y = tile_position.get_tilemap_y(tilemap);

        tile_position.set_update_required(false);
    }
}
