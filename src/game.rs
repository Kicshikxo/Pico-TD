use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_persistent::Persistent;

use crate::{
    assets::{
        audio::game::GameAudioAssets,
        entities::tile::TilemapTileAssets,
        levels::{Level, LevelsAssets},
        AssetsPlugin,
    },
    audio::{GameAudioPlugin, GameAudioVolume},
    entities::{
        structure::{Structure, StructureVariant},
        tile::{
            movement::TileMovement,
            position::TilePosition,
            sprite::{
                StructureTileSpriteVariant, TileSprite, TileSpriteVariant, UnitTileSpriteVariant,
            },
        },
        tilemap::Tilemap,
        unit::{
            health::{UnitHealth, UnitHealthBar},
            Unit, UnitVariant,
        },
        EntitiesPlugin,
    },
    ui::{GameUiPlugin, UiState},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AssetsPlugin, GameAudioPlugin, EntitiesPlugin, GameUiPlugin));

        app.init_state::<GameState>();
        app.add_systems(OnEnter(GameState::Setup), setup)
            .add_systems(OnEnter(GameState::Start), start_game)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::InGame)),
            )
            .add_systems(Update, main_update.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct GameTilemap;

#[derive(Component)]
pub struct BackgroundSound;

#[derive(Resource)]
pub struct SelectedStructure {
    pub position: TilePosition,
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
    mut window: Single<&mut Window>,
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

    window.visible = true;
}

fn start_game(
    mut commands: Commands,
    seleted_level: Res<Level>,
    game_audio_assets: Res<GameAudioAssets>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if seleted_level.error.is_some() {
        next_ui_state.set(UiState::LevelSelect);
        next_game_state.set(GameState::Pause);
        return;
    }

    let mut tilemap = Tilemap::new(
        UVec2::new(seleted_level.size.x, seleted_level.size.y),
        UVec2::new(16, 16),
    );
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..seleted_level.size.x {
        for y in 0..seleted_level.size.y {
            let tilemap_tile = seleted_level.map[y as usize][x as usize];
            tilemap.set_tile(
                TilePosition::new(x as f32, y as f32),
                commands
                    .spawn((
                        TileSprite::new(TileSpriteVariant::Tilemap(
                            tilemap_tile.get_variant().into(),
                        )),
                        tilemap_tile,
                    ))
                    .id(),
            );
        }
    }

    commands.entity(tilemap_entity).insert((
        GameTilemap,
        Transform::from_translation(
            (tilemap.get_size() * tilemap.get_tile_size() - tilemap.get_tile_size())
                .extend(0)
                .as_vec3()
                / -2.0,
        ),
        tilemap,
    ));

    for i in 0..8 {
        commands.entity(tilemap_entity).with_children(|parent| {
            parent
                .spawn((
                    Unit::new(UnitVariant::Truck),
                    UnitHealth::new(100),
                    TileMovement::new(
                        seleted_level.paths[0].clone(),
                        Duration::from_secs(10 + 3 * i),
                    ),
                    TileSprite::new(TileSpriteVariant::Unit(UnitTileSpriteVariant::Truck)),
                ))
                .with_child(UnitHealthBar);
        });
    }

    for structure_position in seleted_level.structure_points.iter() {
        commands.entity(tilemap_entity).with_child((
            Structure::new(StructureVariant::Empty),
            TilePosition::new(structure_position.x, structure_position.y),
            TileSprite::new(TileSpriteVariant::Structure(
                StructureTileSpriteVariant::Empty,
            )),
        ));
    }

    commands.entity(tilemap_entity).with_child((
        BackgroundSound,
        AudioPlayer::new(game_audio_assets.background.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(game_audio_volume.get_music_volume()),
            ..default()
        },
    ));

    next_game_state.set(GameState::InGame);
}

fn update_cursor_position(
    mut commands: Commands,
    window: Single<&Window>,
    main_camera: Query<(&Camera, &GlobalTransform)>,
    game_tilemap: Query<(&Tilemap, &Transform), With<GameTilemap>>,
    structures: Query<(&Structure, &TilePosition)>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if mouse_button_events.is_empty() {
        return;
    }

    let Ok((camera, camera_transform)) = main_camera.get_single() else {
        return;
    };
    let Ok((tilemap, tilemap_transform)) = game_tilemap.get_single() else {
        return;
    };

    for mouse_click in mouse_button_events.read() {
        if mouse_click.button == MouseButton::Left && mouse_click.state == ButtonState::Pressed {
            let Ok(cursor_position) = camera.viewport_to_world_2d(
                camera_transform,
                window.cursor_position().unwrap_or(Vec2::ZERO),
            ) else {
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

            for (structure, structure_tile_position) in structures.iter() {
                if structure_tile_position.as_vec2() == cursor_tile_position.as_vec2().ceil() {
                    commands.insert_resource(SelectedStructure {
                        position: *structure_tile_position,
                    });
                    if structure.get_variant() == StructureVariant::Empty {
                        next_ui_state.set(UiState::StructureSelect);
                    } else {
                        next_ui_state.set(UiState::StructureInfo);
                    }
                    next_game_state.set(GameState::Pause);

                    break;
                }
            }
        }
    }
}

#[allow(unused)]
fn main_update(
    mut commands: Commands,
    mut gizmos: Gizmos,
    structures: Query<(&Structure, &Transform)>,
    levels_assets: Res<LevelsAssets>,
    levels_assets_loader: Res<Assets<Level>>,
    time: Res<Time>,
) {
    // let level: &Level = levels_assets_loader.get(&levels_assets.compain[0]).unwrap();

    // for (structure, structure_transform) in structures.iter() {
    //     gizmos.circle_2d(
    //         structure_transform.translation.xy(),
    //         structure.get_radius() * 16.0,
    //         Color::srgb(1.0, 0.0, 0.0),
    //     );
    // }

    // for structure_point in level.structure_points.iter() {
    //     gizmos.circle_2d(
    //         Vec2::new(
    //             structure_point.x as f32 * 16.0 - 10.0 * 16.0 + 16.0 / 2.0,
    //             (structure_point.y as f32 * 16.0 - 10.0 * 16.0 + 16.0 / 2.0) * -1.0,
    //         ),
    //         8.0,
    //         Color::srgb(1.0, 0.0, 0.0),
    //     );
    // }

    // gizmos
    //     .grid_2d(
    //         Isometry2d::IDENTITY,
    //         UVec2::new(20, 20),
    //         Vec2::new(16.0, 16.0),
    //         Color::srgb(0.0, 0.0, 0.0).with_alpha(0.5),
    //     )
    //     .outer_edges();

    // for path in level.paths.windows(2) {
    //     let from = Vec2::new(
    //         path[0].x * 16.0 - 10.0 * 16.0 + 16.0 / 2.0,
    //         (path[0].y * 16.0 - 10.0 * 16.0 + 16.0 / 2.0) * -1.0,
    //     );
    //     let to = Vec2::new(
    //         path[1].x * 16.0 - 10.0 * 16.0 + 16.0 / 2.0,
    //         (path[1].y * 16.0 - 10.0 * 16.0 + 16.0 / 2.0) * -1.0,
    //     );
    //     gizmos
    //         .arrow_2d(from, to, Color::srgb(1.0, 0.0, 0.0))
    //         .with_tip_length(4.0);
    // }

    // for progress in (0..=100).step_by(1) {
    //     let enemy_position = enemy.movement.position_at_progress(progress as f32 / 100.0);
    //     let gizmos_position = Vec2::new(
    //         enemy_position.x * 16.0 - 10.0 * 16.0 + 16.0 / 2.0,
    //         (enemy_position.y * 16.0 - 10.0 * 16.0 + 16.0 / 2.0) * -1.0,
    //     );

    //     gizmos.circle_2d(
    //         gizmos_position,
    //         4.0,
    //         Color::srgb(1.0, progress as f32 / 100.0, 0.0),
    //     );
    // }
}
