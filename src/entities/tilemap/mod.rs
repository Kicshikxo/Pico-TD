pub mod movement;
pub mod position;
pub mod tile;

use std::collections::HashMap;

use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use position::TilePosition;

use crate::{assets::entities::tile::TileAssets, game::GameState};
use tile::{Tile, TileVariant};

#[derive(Component, Clone, Debug)]
#[component(on_add = Tilemap::on_add)]
#[require(Transform, InheritedVisibility)]
pub struct Tilemap {
    size: UVec2,
    tiles: HashMap<IVec2, Entity>,
    tile_size: UVec2,
    need_update: bool,
}

#[allow(unused)]
impl Tilemap {
    pub fn new(size: UVec2, tile_size: UVec2) -> Self {
        Self {
            size,
            tiles: HashMap::new(),
            tile_size,
            need_update: true,
        }
    }
    fn on_add(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
        let tiles = world.get::<Self>(entity).unwrap().tiles.clone();

        for tile in tiles.values() {
            world.commands().entity(entity).add_child(*tile);
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
    pub fn set_tile(&mut self, position: TilePosition, entity: Entity) {
        self.need_update = true;
        self.tiles.insert(position.as_ivec2(), entity);
    }
    pub fn get_tile(&self, position: TilePosition) -> Option<Entity> {
        self.tiles.get(&position.as_ivec2()).copied()
    }
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_tilemap.run_if(in_state(GameState::InGame)));
    }
}

fn update_tilemap(
    mut tilemaps: Query<&mut Tilemap>,
    mut tiles: Query<(&Tile, &mut Sprite, &mut Transform)>,
    tile_assets: Res<TileAssets>,
) {
    for mut tilemap in tilemaps.iter_mut() {
        if tilemap.need_update == false {
            continue;
        }

        for (tile_position, tile_entity) in &tilemap.tiles {
            let nearby_tile = |dx: i32, dy: i32| -> TileVariant {
                tilemap
                    .get_tile(TilePosition::from_ivec2(tile_position + IVec2::new(dx, dy)))
                    .and_then(|entity| tiles.get(entity).ok())
                    .map(|(tile, _, _)| tile.variant)
                    .unwrap_or(TileVariant::Unknown)
            };

            let tiles_around: [[TileVariant; 3]; 3] = [
                [nearby_tile(-1, 1), nearby_tile(0, 1), nearby_tile(1, 1)],
                [nearby_tile(-1, 0), nearby_tile(0, 0), nearby_tile(1, 0)],
                [nearby_tile(-1, -1), nearby_tile(0, -1), nearby_tile(1, -1)],
            ];

            let Ok((tile, mut tile_sprite, mut tile_transform)) = tiles.get_mut(*tile_entity)
            else {
                continue;
            };

            if let Some(texture_atlas) = tile_sprite.texture_atlas.as_mut() {
                texture_atlas.index = match tile.variant {
                    TileVariant::Ground => tile_assets.get_ground_tile_index(tiles_around) as usize,
                    TileVariant::Road => tile_assets.get_road_tile_index(tiles_around) as usize,
                    TileVariant::Water => tile_assets.get_water_tile_index(tiles_around) as usize,
                    TileVariant::Unknown => 0,
                };
            }

            tile_transform.translation = (tile_position * tilemap.tile_size.as_ivec2())
                .extend(-1)
                .as_vec3();
        }

        tilemap.need_update = false;
    }
}
