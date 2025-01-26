use bevy::prelude::*;

use crate::{
    entities::{
        structure::Structure,
        tilemap::{
            tile::{TilemapTile, TilemapTileVariant},
            Tilemap,
        },
    },
    game::{GameState, GameTilemap},
    input::SelectedTile,
};

use super::{
    position::TilePosition,
    sprite::{TileSprite, TileSpriteVariant, UtilVariant},
};

#[derive(Component)]
#[require(TilePosition)]
pub struct TileIndicator;

pub struct TileIndicatorPlugin;

impl Plugin for TileIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_tile_indicator);
        app.add_systems(
            Update,
            update_tile_indicator
                .run_if(in_state(GameState::InGame).and(resource_changed::<SelectedTile>)),
        );
    }
}

fn init_tile_indicator(
    mut commands: Commands,
    tile_indicators: Query<Entity, Added<TileIndicator>>,
) {
    for tile_indicator_entity in tile_indicators.iter() {
        commands.entity(tile_indicator_entity).insert((
            TileSprite::new(TileSpriteVariant::Util(UtilVariant::TileIndicator)),
            Transform::from_translation(Vec3::new(0.0, 0.0, -0.5)),
        ));
    }
}

fn update_tile_indicator(
    mut tile_indicator: Query<(&mut TilePosition, &mut Sprite), With<TileIndicator>>,
    game_tilemap: Query<&Tilemap, With<GameTilemap>>,
    tiles: Query<&TilemapTile>,
    structures: Query<&TilePosition, (With<Structure>, Without<TileIndicator>)>,
    selected_tile: Res<SelectedTile>,
) {
    let Ok(game_tilemap) = game_tilemap.get_single() else {
        return;
    };

    for (mut tile_indicator_tile_position, mut tile_indicator_sprite) in tile_indicator.iter_mut() {
        tile_indicator_tile_position.set_from_vec2(selected_tile.tile_position.as_vec2());

        let structure_found = structures.iter().any(|structure_tile_position| {
            structure_tile_position.as_vec2() == selected_tile.tile_position.as_vec2()
        });

        if let Some(selected_tile_entity) =
            game_tilemap.get_tile_from_tile_position(selected_tile.tile_position)
        {
            if let Ok(selected_tile) = tiles.get(selected_tile_entity) {
                if structure_found {
                    tile_indicator_sprite.color = Color::srgba(1.0, 1.0, 0.0, 0.75);
                } else if selected_tile.get_variant() == TilemapTileVariant::Ground {
                    tile_indicator_sprite.color = Color::srgba(0.0, 1.0, 0.0, 0.75);
                } else {
                    tile_indicator_sprite.color = Color::srgba(1.0, 0.0, 0.0, 0.75);
                }
            }
        }
    }
}
