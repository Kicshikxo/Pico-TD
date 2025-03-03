use bevy::prelude::*;

use crate::game::{
    entities::{
        soldier::Soldier,
        tile::{
            position::TilePosition,
            sprite::{TileSprite, TileSpriteVariant, UtilVariant},
        },
        tilemap::{
            tile::{TilemapTile, TilemapTileVariant},
            Tilemap,
        },
    },
    input::SelectedTile,
    GameTilemap,
};

#[derive(Component)]
#[require(TilePosition)]
pub struct TileIndicator;

pub struct TileIndicatorPlugin;

impl Plugin for TileIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, init_tile_indicator);

        app.add_systems(
            Update,
            update_tile_indicator.run_if(resource_changed::<SelectedTile>),
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
            TilePosition::new(-1.0, -1.0).with_z(-1.0),
        ));
    }
}

fn update_tile_indicator(
    mut tile_indicator: Query<(&mut TilePosition, &mut Sprite), With<TileIndicator>>,
    game_tilemap: Query<&Tilemap, With<GameTilemap>>,
    tiles: Query<&TilemapTile>,
    soldiers: Query<&TilePosition, (With<Soldier>, Without<TileIndicator>)>,
    selected_tile: Res<SelectedTile>,
) {
    let Ok(game_tilemap) = game_tilemap.get_single() else {
        return;
    };

    for (mut tile_indicator_tile_position, mut tile_indicator_sprite) in tile_indicator.iter_mut() {
        tile_indicator_tile_position.set_from_vec2(selected_tile.tile_position.as_vec2());

        let soldier_found = soldiers.iter().any(|soldier_tile_position| {
            soldier_tile_position.as_vec2() == selected_tile.tile_position.as_vec2()
        });

        if let Some(selected_tile_entity) =
            game_tilemap.get_tile(selected_tile.tile_position.as_ivec2())
        {
            if let Ok(selected_tile) = tiles.get(selected_tile_entity) {
                if soldier_found {
                    tile_indicator_sprite.color = Color::srgba(1.0, 1.0, 0.0, 0.75);
                } else if selected_tile.get_variant() == TilemapTileVariant::Ground {
                    tile_indicator_sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.75);
                } else {
                    tile_indicator_sprite.color = Color::srgba(1.0, 0.0, 0.0, 0.75);
                }
            }
        } else {
            tile_indicator_sprite.color = Color::NONE;
        }
    }
}
