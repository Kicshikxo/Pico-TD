pub mod assets;
pub mod audio;
pub mod config;
pub mod entities;
pub mod input;
pub mod meshes;
pub mod player;
pub mod speed;
pub mod ui;
pub mod waves;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    winit::WinitWindows,
};
use bevy_persistent::Persistent;
use winit::window::Icon;

use crate::game::{
    assets::{audio::game::GameAudioAssets, levels::Level, utils::UtilsAssets, GameAssetsPlugin},
    audio::{GameAudioPlugin, GameAudioVolume},
    config::GameConfigPlugin,
    entities::{tile::indicator::TileIndicator, tilemap::Tilemap, GameEntitiesPlugin},
    input::GameInputPlugin,
    player::{Player, PlayerPlugin},
    speed::GameSpeed,
    ui::{GameUiPlugin, UiState},
    waves::{GameWaves, GameWavesPlugin},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameConfigPlugin,
            GameAssetsPlugin,
            GameAudioPlugin,
            GameEntitiesPlugin,
            GameUiPlugin,
            GameWavesPlugin,
            GameInputPlugin,
            PlayerPlugin,
        ));

        app.init_state::<GameState>();
        app.init_resource::<GameSpeed>();

        app.add_systems(OnEnter(GameState::Setup), setup)
            .add_systems(OnEnter(GameState::Start), start_game)
            .add_systems(OnEnter(GameState::Pause), pause_game)
            .add_systems(OnExit(GameState::Pause), resume_game);
    }
}

#[derive(Component)]
pub struct GameTilemap;

#[derive(Component)]
pub struct GameBackgroundAudio;

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
    mut windows: Query<(Entity, &mut Window)>,
    winit_windows: NonSend<WinitWindows>,
    utils_assets: Res<UtilsAssets>,
    asset_images: Res<Assets<Image>>,
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

    for (window_entity, mut window) in windows.iter_mut() {
        window.visible = true;

        let Some(winit_window) = winit_windows.get_window(window_entity) else {
            continue;
        };
        let Some(window_icon) = asset_images.get(&utils_assets.window_icon) else {
            continue;
        };

        winit_window.set_window_icon(
            Icon::from_rgba(
                window_icon.data.clone(),
                window_icon.width(),
                window_icon.height(),
            )
            .ok(),
        );
    }
}

fn start_game(
    mut commands: Commands,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    selected_level: Res<Level>,
    mut player: ResMut<Player>,
    mut game_waves: ResMut<GameWaves>,
    mut game_speed: ResMut<GameSpeed>,
    game_audio_assets: Res<GameAudioAssets>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if selected_level.get_error().is_some() {
        next_ui_state.set(UiState::LevelSelect);
        next_game_state.set(GameState::Pause);
        return;
    }

    if let Ok(tilemap_entity) = game_tilemap.get_single() {
        commands.entity(tilemap_entity).despawn_recursive();
    }

    commands
        .spawn((
            GameTilemap,
            Tilemap::new(selected_level.get_size(), selected_level.get_tile_size()),
        ))
        .with_child((
            GameBackgroundAudio,
            AudioPlayer::new(game_audio_assets.background.clone()),
            PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(game_audio_volume.get_music_volume()),
                ..default()
            },
        ))
        .with_child(TileIndicator);

    player.restart(
        selected_level.get_player_health(),
        selected_level.get_player_money(),
    );
    game_waves.restart(selected_level.get_waves().len().saturating_sub(1));
    game_speed.set_default();

    next_ui_state.set(UiState::InGame);
    next_game_state.set(GameState::InGame);
}

fn pause_game(
    mut background_audio: Query<&mut AudioSink, With<GameBackgroundAudio>>,
    ui_state: Res<State<UiState>>,
) {
    if matches!(
        ui_state.get(),
        UiState::SoldierSelect | UiState::SoldierInfo
    ) {
        return;
    }
    if let Ok(background_audio_sink) = background_audio.get_single_mut() {
        background_audio_sink.pause();
    }
}

fn resume_game(mut background_audio: Query<&mut AudioSink, With<GameBackgroundAudio>>) {
    if let Ok(background_audio_sink) = background_audio.get_single_mut() {
        background_audio_sink.play();
    }
}
