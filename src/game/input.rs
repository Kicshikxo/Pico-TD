use bevy::{audio::PlaybackMode, input::touch::TouchPhase, prelude::*};
use bevy_persistent::Persistent;

use crate::game::{
    assets::audio::ui::UiAudioAssets,
    audio::{GameAudio, GameAudioVolume},
    camera::GameCamera,
    entities::{
        soldier::{Soldier, SoldierVariant},
        tile::position::TilePosition,
        tilemap::{
            Tilemap,
            tile::{TilemapTile, TilemapTileVariant},
        },
    },
    ui::UiState,
    waves::GameWaves,
    {GameState, GameTilemap},
};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedTile>();
        app.init_resource::<SelectedSoldier>();

        app.add_systems(
            Update,
            (update_selected_tile, update_selected_soldier)
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Resource, Default)]
pub struct SelectedTile {
    pub tile_position: TilePosition,
    pub previous_tile_position: TilePosition,
}

#[derive(Resource, Default)]
pub struct SelectedSoldier {
    pub variant: Option<SoldierVariant>,
    pub tile_position: TilePosition,
}

fn update_selected_tile(
    game_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    game_tilemap: Single<(&Tilemap, &Transform), With<GameTilemap>>,
    mut selected_tile: ResMut<SelectedTile>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut touch_events: EventReader<TouchInput>,
) {
    let cursor_moved = cursor_moved_events.is_empty() == false;
    let touch_moved = touch_events.is_empty() == false;

    if cursor_moved == false && touch_moved == false {
        return;
    }
    let Ok((camera, camera_transform)) = game_camera.single() else {
        return;
    };
    let (game_tilemap, game_tilemap_transform) = game_tilemap.into_inner();

    selected_tile.previous_tile_position = selected_tile.tile_position;

    let mut update_tile_position = |position: Vec2| {
        let Ok(cursor_position) = camera.viewport_to_world_2d(camera_transform, position) else {
            return;
        };

        let cursor_in_tilemap_position = game_tilemap_transform
            .compute_matrix()
            .inverse()
            .transform_point3(
                (cursor_position - game_tilemap.get_tile_size() as f32 / 2.0).extend(0.0),
            )
            .xy();

        let cursor_tile_position =
            TilePosition::from_tilemap_position(game_tilemap, cursor_in_tilemap_position);

        if selected_tile.tile_position.as_vec2() != cursor_tile_position.as_vec2() {
            selected_tile.tile_position = cursor_tile_position;
        }
    };

    for cursor_moved in cursor_moved_events.read() {
        update_tile_position(cursor_moved.position);
    }

    for touch in touch_events.read() {
        update_tile_position(touch.position);
    }
}

fn update_selected_soldier(
    mut commands: Commands,
    game_tilemap: Single<&Tilemap, With<GameTilemap>>,
    tiles: Query<&TilemapTile>,
    soldiers: Query<(&Soldier, &TilePosition)>,
    selected_tile: Res<SelectedTile>,
    mut selected_soldier: ResMut<SelectedSoldier>,
    game_waves: Res<GameWaves>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut touch_events: EventReader<TouchInput>,
    ui_interaction: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    game_audio: Single<Entity, With<GameAudio>>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    ui_audio_assets: Res<UiAudioAssets>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mouse_pressed = mouse_button_input.just_pressed(MouseButton::Left);
    let touch_started = touch_events
        .read()
        .any(|touch| touch.phase == TouchPhase::Started);

    if mouse_pressed == false && touch_started == false {
        return;
    }
    if selected_tile.tile_position.as_vec2() != selected_tile.previous_tile_position.as_vec2() {
        return;
    }
    if ui_interaction.is_empty() == false {
        return;
    }
    if game_waves.is_fully_completed() == true {
        return;
    }

    if let Some(selected_tile_entity) =
        game_tilemap.get_tile(selected_tile.tile_position.as_ivec2())
    {
        if let Ok(selected_tile) = tiles.get(selected_tile_entity) {
            if selected_tile.get_variant() != TilemapTileVariant::Ground {
                return;
            }
        } else {
            return;
        }
    } else {
        return;
    }

    if let Some((soldier, _soldier_tile_position)) =
        soldiers.iter().find(|(_soldier, soldier_tile_position)| {
            soldier_tile_position.as_vec2() == selected_tile.tile_position.as_vec2()
        })
    {
        selected_soldier.variant = Some(soldier.get_variant());
        next_ui_state.set(UiState::SoldierInfo);
    } else {
        selected_soldier.variant = None;
        next_ui_state.set(UiState::SoldierSelect);
    }

    selected_soldier.tile_position = selected_tile.tile_position;
    next_game_state.set(GameState::Pause);

    commands.entity(game_audio.entity()).with_child((
        AudioPlayer::new(ui_audio_assets.tilemap_click.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Remove,
            volume: game_audio_volume.get_sfx_volume(),
            ..default()
        },
    ));
}
