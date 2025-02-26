pub mod assets;
pub mod audio;
pub mod entities;
pub mod input;
pub mod meshes;
pub mod player;
pub mod ui;
pub mod waves;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_persistent::Persistent;

use crate::game::{
    assets::{audio::game::GameAudioAssets, levels::Level, GameAssetsPlugin},
    audio::{GameAudioPlugin, GameAudioVolume},
    entities::{tile::indicator::TileIndicator, tilemap::Tilemap, GameEntitiesPlugin},
    input::GameInputPlugin,
    player::{Player, PlayerPlugin},
    ui::{GameUiPlugin, UiState},
    waves::{GameWave, GameWavesPlugin},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameAssetsPlugin,
            GameAudioPlugin,
            GameEntitiesPlugin,
            GameUiPlugin,
            GameWavesPlugin,
            GameInputPlugin,
            PlayerPlugin,
        ));

        app.init_resource::<GameSpeed>();

        app.init_state::<GameState>();
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

#[derive(Resource, Default)]
pub enum GameSpeed {
    #[default]
    Normal,
    Double,
    Triple,
}

impl GameSpeed {
    pub fn as_f32(&self) -> f32 {
        match self {
            GameSpeed::Normal => 1.0,
            GameSpeed::Double => 2.0,
            GameSpeed::Triple => 3.0,
        }
    }
    pub fn set_default(&mut self) {
        *self = GameSpeed::default();
    }
    pub fn toggle(&mut self) {
        match self {
            GameSpeed::Normal => *self = GameSpeed::Double,
            GameSpeed::Double => *self = GameSpeed::Triple,
            GameSpeed::Triple => *self = GameSpeed::Normal,
        }
    }
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
    game_tilemap: Query<Entity, With<GameTilemap>>,
    selected_level: Res<Level>,
    mut player: ResMut<Player>,
    mut game_wave: ResMut<GameWave>,
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
            Tilemap::new(selected_level.get_size(), UVec2::new(16, 16)),
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
    game_wave.restart(selected_level.get_waves().len());
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
