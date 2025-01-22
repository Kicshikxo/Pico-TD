use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_persistent::Persistent;

use crate::{
    assets::{audio::game::GameAudioAssets, levels::Level, AssetsPlugin},
    audio::{GameAudioPlugin, GameAudioVolume},
    entities::{
        structure::Structure,
        tile::{movement::TileMovement, position::TilePosition},
        tilemap::{
            tile::{TilemapTile, TilemapTileVariant},
            Tilemap,
        },
        unit::{Unit, UnitVariant},
        EntitiesPlugin,
    },
    ui::{GameUiPlugin, UiState},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AssetsPlugin, GameAudioPlugin, EntitiesPlugin, GameUiPlugin));

        app.init_resource::<SelectedTile>();
        app.init_resource::<SelectedStructure>();

        app.init_state::<GameState>();
        app.add_systems(OnEnter(GameState::Setup), setup)
            .add_systems(OnEnter(GameState::Start), start_game)
            .add_systems(OnEnter(GameState::Pause), pause_game)
            .add_systems(OnExit(GameState::Pause), resume_game)
            .add_systems(
                Update,
                (update_selected_tile, update_selected_structure)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct GameTilemap;

#[derive(Component)]
pub struct GameBackgroundSound;

#[derive(Resource, Default)]
pub struct SelectedTile {
    pub tile_position: TilePosition,
}

#[derive(Resource, Default)]
pub struct SelectedStructure {
    pub tile_position: TilePosition,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    AssetsLoading,
    Setup,
    Pause,
    Start,
    InGame,
}

fn setup(
    mut commands: Commands,
    mut window: Query<&mut Window>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Camera2d::default(),
        Msaa::Off,
        Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
    ));

    next_ui_state.set(UiState::Menu);
    next_game_state.set(GameState::Pause);

    if let Ok(mut window) = window.get_single_mut() {
        window.visible = true;
    };
}

fn start_game(
    mut commands: Commands,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    selected_level: Res<Level>,
    game_audio_assets: Res<GameAudioAssets>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if selected_level.error.is_some() {
        next_ui_state.set(UiState::LevelSelect);
        next_game_state.set(GameState::Pause);
        return;
    }

    if let Ok(tilemap_entity) = game_tilemap.get_single() {
        commands.entity(tilemap_entity).despawn_recursive();
    }

    let tilemap_entity = commands
        .spawn((
            GameTilemap,
            Tilemap::new(selected_level.size, UVec2::new(16, 16)),
        ))
        .id();

    for index in 0..10 {
        commands.entity(tilemap_entity).with_child((
            Unit::new(UnitVariant::Truck),
            TileMovement::new(
                selected_level.paths[0].clone(),
                Duration::from_secs(20),
                Some(Duration::from_secs_f32(0.33 * index as f32)),
            ),
        ));
    }
    for index in 0..10 {
        commands.entity(tilemap_entity).with_child((
            Unit::new(UnitVariant::Plane),
            TileMovement::new(
                selected_level.paths[0].clone(),
                Duration::from_secs(20),
                Some(Duration::from_secs_f32(0.33 * index as f32 + 0.33 * 12.0)),
            ),
        ));
    }
    for index in 0..5 {
        commands.entity(tilemap_entity).with_child((
            Unit::new(UnitVariant::Tank),
            TileMovement::new(
                selected_level.paths[0].clone(),
                Duration::from_secs(20),
                Some(Duration::from_secs_f32(0.33 * index as f32 + 0.33 * 22.0)),
            ),
        ));
    }

    commands.entity(tilemap_entity).with_child((
        GameBackgroundSound,
        AudioPlayer::new(game_audio_assets.background.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(game_audio_volume.get_music_volume()),
            ..default()
        },
    ));

    next_game_state.set(GameState::InGame);
}

fn pause_game(
    mut background_sound: Query<&mut AudioSink, With<GameBackgroundSound>>,
    ui_state: Res<State<UiState>>,
) {
    if matches!(
        ui_state.get(),
        UiState::StructureSelect | UiState::StructureInfo
    ) {
        return;
    }
    if let Ok(background_sound_sink) = background_sound.get_single_mut() {
        background_sound_sink.pause();
    }
}

fn resume_game(mut background_sound: Query<&mut AudioSink, With<GameBackgroundSound>>) {
    if let Ok(background_sound_sink) = background_sound.get_single_mut() {
        background_sound_sink.play();
    }
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
    let Ok((tilemap, tilemap_transform)) = game_tilemap.get_single() else {
        return;
    };

    for cursor_moved in cursor_moved_events.read() {
        let Ok(cursor_position) =
            camera.viewport_to_world_2d(camera_transform, cursor_moved.position)
        else {
            continue;
        };

        let cursor_in_tilemap_position = tilemap_transform
            .compute_matrix()
            .inverse()
            .transform_point3(
                (cursor_position - tilemap.get_tile_size().as_vec2() / 2.0).extend(0.0),
            )
            .xy();

        let cursor_tile_position =
            TilePosition::from_tilemap_position(tilemap, cursor_in_tilemap_position);

        selected_tile.tile_position = cursor_tile_position;
    }
}

fn update_selected_structure(
    game_tilemap: Query<&Tilemap, With<GameTilemap>>,
    tiles: Query<&TilemapTile>,
    structures: Query<&TilePosition, With<Structure>>,
    selected_tile: Res<SelectedTile>,
    mut selected_structure: ResMut<SelectedStructure>,
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
    // ! Refactor
    let game_tilemap = game_tilemap.single();
    if let Some(selected_tile_entity) = game_tilemap.get_tile(IVec2::new(
        selected_tile.tile_position.as_ivec2().x,
        game_tilemap.get_size().y as i32 - selected_tile.tile_position.as_ivec2().y - 1,
    )) {
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
