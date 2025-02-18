use std::time::Duration;

use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::{
    assets::levels::{CompletedLevels, Level, LevelCompletionStars},
    entities::{enemy::Enemy, tile::movement::TileMovement},
    game::{GameState, GameTilemap},
    player::Player,
    ui::UiState,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WaveState {
    NotStarted,
    Setup,
    InProgress,
    Completed,
}

#[derive(Resource)]
pub struct GameWave {
    total: usize,
    current: usize,
    state: WaveState,
}

impl Default for GameWave {
    fn default() -> Self {
        Self {
            total: 0,
            current: 0,
            state: WaveState::NotStarted,
        }
    }
}

impl GameWave {
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
        let last_index = self.total.saturating_sub(1);
        if self.current >= last_index {
            return;
        }
        self.current = self.current.saturating_add(1).min(last_index);
        self.state = WaveState::Setup;
    }
    pub fn is_last(&self) -> bool {
        let last_index = self.total.saturating_sub(1);
        self.current == last_index
    }
    pub fn is_next_wave_allowed(&self) -> bool {
        self.state == WaveState::NotStarted
            || self.state == WaveState::Completed && self.is_last() == false
    }
    pub fn is_fully_completed(&self) -> bool {
        self.state == WaveState::Completed && self.is_last() == true
    }
}

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWave>();

        app.add_systems(
            PreUpdate,
            update_wave.run_if(in_state(GameState::InGame).and(resource_changed::<GameWave>)),
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
    mut game_wave: ResMut<GameWave>,
    player: Res<Player>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if selected_level.error.is_some() {
        return;
    }
    if selected_level.waves.is_empty() {
        return;
    }
    if game_wave.get_state() != WaveState::Setup {
        if game_wave.is_fully_completed() == true {
            next_ui_state.set(UiState::GameOver);
            next_game_state.set(GameState::Pause);
            completed_levels
                .update(|levels| {
                    levels.add(
                        &selected_level.name,
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

    for wave in selected_level.waves[game_wave.get_current()].iter() {
        for index in 0..wave.count {
            commands.entity(tilemap_entity).with_child((
                Enemy::new(wave.enemy_variant),
                TileMovement::new(
                    selected_level.paths[wave.path_index].clone(),
                    Duration::from_secs_f32(wave.duration),
                    Some(Duration::from_secs_f32(
                        wave.spawn_interval * index as f32 + wave.spawn_delay,
                    )),
                ),
                Transform::from_scale(Vec3::ZERO),
            ));
        }
    }
    game_wave.set_state(WaveState::InProgress);
}

fn update_wave_state(enemies: Query<&Enemy>, mut game_wave: ResMut<GameWave>) {
    if enemies.is_empty() && game_wave.get_state() == WaveState::InProgress {
        game_wave.set_state(WaveState::Completed);
    }
}
