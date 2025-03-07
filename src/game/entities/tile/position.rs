use bevy::prelude::*;

use crate::game::{
    entities::tilemap::Tilemap,
    {GameState, GameTilemap},
};

#[derive(Component, Clone, Copy)]
#[require(Transform)]
pub struct TilePosition {
    position: Vec2,
    position_z: f32,
    update_required: bool,
}

impl Default for TilePosition {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            position_z: 0.0,
            update_required: true,
        }
    }
}

#[allow(unused)]
impl TilePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            ..default()
        }
    }
    pub fn with_z(mut self, z: f32) -> Self {
        self.position_z = z;
        self
    }
    pub fn from_vec2(vec: Vec2) -> Self {
        Self::new(vec.x, vec.y)
    }
    pub fn from_ivec2(ivec: IVec2) -> Self {
        Self::new(ivec.x as f32, ivec.y as f32)
    }
    pub fn from_tilemap_position(tilemap: &Tilemap, position: Vec2) -> Self {
        let tile_size_x = tilemap.get_tile_size().x as f32;
        let tile_size_y = tilemap.get_tile_size().y as f32;
        let tilemap_size_y = tilemap.get_size().y as f32;

        Self::new(
            (position.x / tile_size_x).ceil(),
            ((tilemap_size_y * tile_size_y - position.y - tile_size_y) / tile_size_y).floor(),
        )
    }
    pub fn as_vec2(&self) -> Vec2 {
        self.position
    }
    pub fn as_ivec2(&self) -> IVec2 {
        self.position.as_ivec2()
    }
    pub fn set(&mut self, x: f32, y: f32) {
        self.set_update_required(self.position != Vec2::new(x, y));
        self.position.x = x;
        self.position.y = y;
    }
    pub fn set_from_vec2(&mut self, vec: Vec2) {
        self.set_update_required(self.position != vec);
        self.position.x = vec.x;
        self.position.y = vec.y;
    }
    pub fn set_x(&mut self, x: f32) {
        self.set_update_required(self.position.x != x);
        self.position.x = x;
    }
    pub fn get_x(&self) -> f32 {
        self.position.x
    }
    pub fn set_y(&mut self, y: f32) {
        self.set_update_required(self.position.y != y);
        self.position.y = y;
    }
    pub fn get_y(&self) -> f32 {
        self.position.y
    }
    pub fn set_z(&mut self, z: f32) {
        self.set_update_required(self.position_z != z);
        self.position_z = z;
    }
    pub fn get_z(&self) -> f32 {
        self.position_z
    }
    pub fn get_tilemap_x(&self, tilemap: &Tilemap) -> f32 {
        self.position.x * tilemap.get_tile_size().x as f32
    }
    pub fn get_tilemap_y(&self, tilemap: &Tilemap) -> f32 {
        let tile_size_y = tilemap.get_tile_size().y as f32;
        let tilemap_size_y = tilemap.get_size().y as f32;

        (tilemap_size_y - self.position.y) * tile_size_y - tile_size_y
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

        position_transform.translation = tile_position
            .get_tilemap_position(tilemap)
            .extend(tile_position.get_z());

        tile_position.set_update_required(false);
    }
}
