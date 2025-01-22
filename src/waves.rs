use std::time::Duration;

use bevy::prelude::*;

use crate::{
    assets::levels::Level,
    entities::{tile::movement::TileMovement, unit::Unit},
    game::{GameState, GameTilemap},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaveState {
    Setup,
    InProgress,
    Completed,
}

#[derive(Resource)]
pub struct CurrentWave {
    total: usize,
    index: usize,
    state: WaveState,
}

impl Default for CurrentWave {
    fn default() -> Self {
        Self {
            total: 0,
            index: 0,
            state: WaveState::Setup,
        }
    }
}

impl CurrentWave {
    pub fn restart(&mut self, total: usize) {
        self.total = total;
        self.index = 0;
        self.state = WaveState::Setup;
    }
    pub fn get_index(&self) -> usize {
        self.index
    }
    pub fn get_state(&self) -> WaveState {
        self.state
    }
    pub fn set_state(&mut self, state: WaveState) {
        self.state = state;
    }
    pub fn next_wave(&mut self) {
        let last_index = self.total.saturating_sub(1);
        if self.index >= last_index {
            return;
        }
        self.index = self.index.saturating_add(1).min(last_index);
        self.state = WaveState::Setup;
    }
}

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentWave>();

        app.add_systems(
            Update,
            update_current_wave
                .run_if(in_state(GameState::InGame).and(resource_changed::<CurrentWave>)),
        );
        app.add_systems(
            Update,
            update_current_wave_state.run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_current_wave(
    mut commands: Commands,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    selected_level: Res<Level>,
    mut current_wave: ResMut<CurrentWave>,
) {
    if selected_level.error.is_some() {
        return;
    }
    if current_wave.get_state() != WaveState::Setup {
        return;
    }
    let Ok(tilemap_entity) = game_tilemap.get_single() else {
        return;
    };

    for wave in selected_level.waves[current_wave.get_index()].iter() {
        for index in 0..wave.count {
            commands.entity(tilemap_entity).with_child((
                Unit::new(wave.unit_variant),
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
    current_wave.set_state(WaveState::InProgress);
}

fn update_current_wave_state(units: Query<&Unit>, mut current_wave: ResMut<CurrentWave>) {
    if units.is_empty() && current_wave.get_state() == WaveState::InProgress {
        current_wave.set_state(WaveState::Completed);
    }
}
