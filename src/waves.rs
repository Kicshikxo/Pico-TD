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
pub struct Wave {
    total: usize,
    current: usize,
    state: WaveState,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            total: 0,
            current: 0,
            state: WaveState::Setup,
        }
    }
}

impl Wave {
    pub fn restart(&mut self, total: usize) {
        self.total = total;
        self.current = 0;
        self.state = WaveState::Setup;
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
    pub fn is_fully_completed(&self) -> bool {
        self.state == WaveState::Completed && self.is_last() == true
    }
}

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Wave>();

        app.add_systems(
            PreUpdate,
            update_wave.run_if(in_state(GameState::InGame).and(resource_changed::<Wave>)),
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
    mut wave: ResMut<Wave>,
) {
    if selected_level.error.is_some() {
        return;
    }
    if selected_level.waves.is_empty() {
        return;
    }
    if wave.get_state() != WaveState::Setup {
        return;
    }
    let Ok(tilemap_entity) = game_tilemap.get_single() else {
        return;
    };

    for wave in selected_level.waves[wave.get_current()].iter() {
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
    wave.set_state(WaveState::InProgress);
}

fn update_wave_state(units: Query<&Unit>, mut wave: ResMut<Wave>) {
    if units.is_empty() && wave.get_state() == WaveState::InProgress {
        wave.set_state(WaveState::Completed);
    }
}
