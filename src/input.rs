use bevy::prelude::*;

use crate::{
    entities::{
        structure::Structure,
        tile::position::TilePosition,
        tilemap::{
            tile::{TilemapTile, TilemapTileVariant},
            Tilemap,
        },
    },
    game::{GameState, GameTilemap},
    ui::UiState,
    waves::Wave,
};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedTile>();
        app.init_resource::<SelectedStructure>();

        app.add_systems(
            Update,
            (update_selected_tile, update_selected_structure).run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Resource, Default)]
pub struct SelectedTile {
    pub tile_position: TilePosition,
}

#[derive(Resource, Default)]
pub struct SelectedStructure {
    pub tile_position: TilePosition,
}

fn update_selected_tile(
    main_camera: Query<(&Camera, &GlobalTransform)>,
    game_tilemap: Query<(&Tilemap, &Transform), With<GameTilemap>>,
    mut selected_tile: ResMut<SelectedTile>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    if cursor_moved_events.is_empty() {
        return;
    }
    let Ok((camera, camera_transform)) = main_camera.get_single() else {
        return;
    };
    let Ok((game_tilemap, game_tilemap_transform)) = game_tilemap.get_single() else {
        return;
    };

    for cursor_moved in cursor_moved_events.read() {
        let Ok(cursor_position) =
            camera.viewport_to_world_2d(camera_transform, cursor_moved.position)
        else {
            continue;
        };

        let cursor_in_tilemap_position = game_tilemap_transform
            .compute_matrix()
            .inverse()
            .transform_point3(
                (cursor_position - game_tilemap.get_tile_size().as_vec2() / 2.0).extend(0.0),
            )
            .xy();

        let cursor_tile_position =
            TilePosition::from_tilemap_position(game_tilemap, cursor_in_tilemap_position);

        selected_tile.tile_position = cursor_tile_position;
    }
}

fn update_selected_structure(
    game_tilemap: Query<&Tilemap, With<GameTilemap>>,
    tiles: Query<&TilemapTile>,
    structures: Query<&TilePosition, With<Structure>>,
    selected_tile: Res<SelectedTile>,
    mut selected_structure: ResMut<SelectedStructure>,
    wave: Res<Wave>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    ui_interaction: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) == false {
        return;
    }
    if ui_interaction.is_empty() == false {
        return;
    }
    if wave.is_fully_completed() == true {
        return;
    }
    let Ok(game_tilemap) = game_tilemap.get_single() else {
        return;
    };
    if let Some(selected_tile_entity) =
        game_tilemap.get_tile_from_tile_position(selected_tile.tile_position)
    {
        if let Ok(selected_tile) = tiles.get(selected_tile_entity) {
            if selected_tile.get_variant() != TilemapTileVariant::Ground {
                return;
            }
        }
    }

    let structure_found = structures.iter().any(|structure_tile_position| {
        structure_tile_position.as_vec2() == selected_tile.tile_position.as_vec2()
    });

    selected_structure.tile_position = selected_tile.tile_position;
    next_ui_state.set(if structure_found {
        UiState::StructureInfo
    } else {
        UiState::StructureSelect
    });
    next_game_state.set(GameState::Pause);
}
