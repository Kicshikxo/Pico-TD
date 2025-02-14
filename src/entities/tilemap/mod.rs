pub mod tile;

use std::collections::HashMap;

use bevy::prelude::*;
use tile::{TilemapTile, TilemapTileVariant};

use crate::{
    assets::{levels::Level, sprites::tile::TileAssets},
    game::GameState,
};

use super::tile::{position::TilePosition, sprite::TileSprite};

#[derive(Component, Clone)]
#[require(Transform, InheritedVisibility)]
pub struct Tilemap {
    size: UVec2,
    tiles: HashMap<IVec2, Entity>,
    tile_size: UVec2,
    update_required: bool,
}

impl Default for Tilemap {
    fn default() -> Self {
        Self {
            size: UVec2::ZERO,
            tiles: HashMap::new(),
            tile_size: UVec2::ZERO,
            update_required: true,
        }
    }
}

impl Tilemap {
    pub fn new(size: UVec2, tile_size: UVec2) -> Self {
        Self {
            size,
            tile_size,
            ..default()
        }
    }
    pub fn get_size(&self) -> UVec2 {
        self.size
    }
    pub fn get_tiles(&self) -> &HashMap<IVec2, Entity> {
        &self.tiles
    }
    pub fn get_tile_size(&self) -> UVec2 {
        self.tile_size
    }
    pub fn set_tile(&mut self, position: IVec2, entity: Entity) {
        self.update_required = true;
        self.tiles.insert(position, entity);
    }
    pub fn get_tile(&self, position: IVec2) -> Option<Entity> {
        self.tiles.get(&position).copied()
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_tilemap);
        app.add_systems(Update, update_tilemap.run_if(in_state(GameState::InGame)));
    }
}

fn init_tilemap(
    mut commands: Commands,
    mut tilemaps: Query<(Entity, &mut Tilemap), Added<Tilemap>>,
    selected_level: Res<Level>,
) {
    for (tilemap_entity, mut tilemap) in tilemaps.iter_mut() {
        if selected_level.error.is_some() {
            return;
        }

        for x in 0..selected_level.size.x {
            for y in 0..selected_level.size.y {
                let tilemap_tile = selected_level.map[y as usize][x as usize];
                let tilemap_tile_entity = commands
                    .spawn((
                        TileSprite::new(tilemap_tile.get_variant().into()),
                        TilePosition::new(x as f32, y as f32).with_z(-2.0),
                        tilemap_tile,
                    ))
                    .id();

                commands
                    .entity(tilemap_entity)
                    .add_child(tilemap_tile_entity);

                tilemap.set_tile(IVec2::new(x as i32, y as i32), tilemap_tile_entity);
            }
        }

        commands
            .entity(tilemap_entity)
            .insert(Transform::from_translation(
                (tilemap.get_size() * tilemap.get_tile_size() - tilemap.get_tile_size())
                    .extend(0)
                    .as_vec3()
                    / -2.0,
            ));
    }
}

fn update_tilemap(
    mut tilemaps: Query<&mut Tilemap>,
    mut tiles: Query<(&TilemapTile, &mut Sprite)>,
    tile_assets: Res<TileAssets>,
) {
    for mut tilemap in tilemaps.iter_mut() {
        if tilemap.get_update_required() == false {
            continue;
        }

        for (tile_position, tile_entity) in tilemap.get_tiles() {
            let nearby_tile = |dx: i32, dy: i32| -> TilemapTileVariant {
                tilemap
                    .get_tile(tile_position + IVec2::new(dx, dy))
                    .and_then(|entity| tiles.get(entity).ok())
                    .map(|(tile, _tile_sprite)| tile.get_variant())
                    .unwrap_or(TilemapTileVariant::Unknown)
            };

            let tiles_around: [[TilemapTileVariant; 3]; 3] = [
                [nearby_tile(-1, -1), nearby_tile(0, -1), nearby_tile(1, -1)],
                [nearby_tile(-1, 0), nearby_tile(0, 0), nearby_tile(1, 0)],
                [nearby_tile(-1, 1), nearby_tile(0, 1), nearby_tile(1, 1)],
            ];

            if let Ok((tile, mut tile_sprite)) = tiles.get_mut(*tile_entity) {
                if let Some(texture_atlas) = tile_sprite.texture_atlas.as_mut() {
                    texture_atlas.index =
                        tile_assets.get_tile_index(tile.get_variant(), tiles_around)
                }
            }
        }

        tilemap.set_update_required(false);
    }
}
