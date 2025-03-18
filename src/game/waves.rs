use std::time::Duration;

use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::game::{
    assets::levels::{CompletedLevels, Level, LevelCompletionStars},
    entities::{enemy::Enemy, tile::movement::TileMovement},
    player::Player,
    ui::UiState,
    {GameState, GameTilemap},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WaveState {
    NotStarted,
    Setup,
    InProgress,
    Completed,
}

#[derive(Resource)]
pub struct GameWaves {
    total: usize,
    current: usize,
    state: WaveState,
}

impl Default for GameWaves {
    fn default() -> Self {
        Self {
            total: 0,
            current: 0,
            state: WaveState::NotStarted,
        }
    }
}

impl GameWaves {
    pub fn restart(&mut self, total: usize) {
        self.total = total;
        self.current = 0;
        self.state = WaveState::NotStarted;
    }
    pub fn get_total(&self) -> usize {
        self.total
    }
    pub fn get_current(&self) -> usize {
        self.current
    }
    pub fn get_state(&self) -> WaveState {
        self.state
    }
    pub fn set_state(&mut self, state: WaveState) {
        self.state = state;
    }
    pub fn next_wave(&mut self) {
        if self.state == WaveState::NotStarted {
            self.state = WaveState::Setup;
            return;
        }
        if self.current >= self.total {
            return;
        }
        self.current = self.current.saturating_add(1).min(self.total);
        self.state = WaveState::Setup;
    }
    pub fn is_last(&self) -> bool {
        self.current == self.total
    }
    pub fn is_next_wave_allowed(&self) -> bool {
        self.state == WaveState::NotStarted
            || self.state == WaveState::Completed && self.is_last() == false
    }
    pub fn is_fully_completed(&self) -> bool {
        self.state == WaveState::Completed && self.is_last() == true
    }
}

pub struct GameWavesPlugin;

impl Plugin for GameWavesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWaves>();

        app.add_systems(
            PreUpdate,
            update_wave.run_if(in_state(GameState::InGame).and(resource_changed::<GameWaves>)),
        );
        app.add_systems(
            Update,
            update_wave_state.run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_wave(
    mut commands: Commands,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    selected_level: Res<Level>,
    mut completed_levels: ResMut<Persistent<CompletedLevels>>,
    mut game_waves: ResMut<GameWaves>,
    player: Res<Player>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if selected_level.get_error().is_some() {
        return;
    }
    if selected_level.get_waves().is_empty() {
        return;
    }
    if game_waves.get_state() != WaveState::Setup {
        if game_waves.is_fully_completed() == true {
            next_ui_state.set(UiState::GameOver);
            next_game_state.set(GameState::Pause);
            completed_levels
                .update(|levels| {
                    levels.add(
                        &selected_level.get_name(),
                        LevelCompletionStars::from_player_health(player.get_health()),
                    )
                })
                .unwrap();
        }
        return;
    }
    let Ok(tilemap_entity) = game_tilemap.get_single() else {
        return;
    };
    let Some(wave) = selected_level.get_wave(game_waves.get_current()) else {
        return;
    };

    for wave_enemies in wave.get_enemies().iter() {
        for index in 0..wave_enemies.get_count() {
            commands.entity(tilemap_entity).with_child((
                Enemy::new(wave_enemies.get_enemy_variant()),
                TileMovement::new(
                    selected_level
                        .get_path(wave_enemies.get_path_index())
                        .get_points()
                        .clone(),
                    Duration::from_secs_f32(wave_enemies.get_duration()),
                    Some(Duration::from_secs_f32(
                        wave_enemies.get_spawn_interval() * index as f32
                            + wave_enemies.get_spawn_delay(),
                    )),
                ),
                Transform::from_scale(Vec3::ZERO),
            ));
        }
    }
    game_waves.set_state(WaveState::InProgress);
}

fn update_wave_state(
    enemies: Query<&Enemy>,
    selected_level: Res<Level>,
    mut game_waves: ResMut<GameWaves>,
    mut player: ResMut<Player>,
) {
    if enemies.is_empty() && game_waves.get_state() == WaveState::InProgress {
        game_waves.set_state(WaveState::Completed);
        if let Some(wave) = selected_level.get_wave(game_waves.get_current()) {
            player.get_money_mut().increase(wave.get_reward());
        }
    }
}
