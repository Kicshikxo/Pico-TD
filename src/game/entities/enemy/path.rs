use std::time::Duration;

use bevy::{prelude::*, sprite::AlphaMode2d};
use bevy_persistent::Persistent;
use serde::{Deserialize, Serialize};

use crate::game::{
    assets::levels::Level,
    config::GameConfig,
    entities::{tile::position::TilePosition, tilemap::Tilemap},
    meshes::rounded_rectangle::RoundedRectangle,
    waves::{GameWaves, WaveState},
    GameState, GameTilemap,
};

#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EnemyPathVisibility {
    #[default]
    PreWaveVisible,
    AlwaysVisible,
    NeverVisible,
}

impl EnemyPathVisibility {
    pub fn to_str(&self) -> &'static str {
        match self {
            EnemyPathVisibility::PreWaveVisible => "enemy.path_visibility.pre_wave_visible",
            EnemyPathVisibility::AlwaysVisible => "enemy.path_visibility.always_visible",
            EnemyPathVisibility::NeverVisible => "enemy.path_visibility.never_visible",
        }
    }
    pub fn as_index(&self) -> usize {
        match self {
            EnemyPathVisibility::PreWaveVisible => 0,
            EnemyPathVisibility::AlwaysVisible => 1,
            EnemyPathVisibility::NeverVisible => 2,
        }
    }
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => EnemyPathVisibility::PreWaveVisible,
            1 => EnemyPathVisibility::AlwaysVisible,
            2 => EnemyPathVisibility::NeverVisible,
            _ => EnemyPathVisibility::default(),
        }
    }
}

#[derive(Component)]
struct EnemyPath {
    path_index: usize,
    visible: bool,
}

impl EnemyPath {
    fn new(path_index: usize) -> Self {
        Self {
            path_index,
            visible: false,
        }
    }
    pub fn get_path_index(&self) -> usize {
        self.path_index
    }
    fn get_visible(&self) -> bool {
        self.visible
    }
    fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }
}

pub struct EnemyPathPlugin;

impl Plugin for EnemyPathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Start), init_enemy_paths);

        app.add_systems(
            Update,
            update_enemy_paths.run_if(
                in_state(GameState::InGame).and(
                    resource_changed::<GameWaves>.or(resource_changed::<Persistent<GameConfig>>),
                ),
            ),
        );
        app.add_systems(
            PostUpdate,
            update_enemy_paths_alpha.run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_enemy_paths(
    mut commands: Commands,
    mut enemy_paths: Query<Entity, With<EnemyPath>>,
    game_tilemap: Query<(Entity, &Tilemap), With<GameTilemap>>,
    selected_level: Res<Level>,
    game_config: Res<Persistent<GameConfig>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for enemy_path_entity in enemy_paths.iter_mut() {
        commands.entity(enemy_path_entity).despawn_recursive();
    }

    let Ok((game_tilemap_entity, game_tilemap)) = game_tilemap.get_single() else {
        return;
    };

    for (path_index, path) in selected_level.get_paths().iter().enumerate() {
        let path_visible = selected_level.get_wave(0).map_or(false, |wave| {
            wave.get_enemies()
                .iter()
                .any(|wave_enemies| wave_enemies.get_path_index() == path_index)
        }) && game_config.get_enemy_path_visibility()
            != EnemyPathVisibility::NeverVisible;
        let path_color = Color::hsl(path_index as f32 * 60.0, 1.0, 0.67)
            .with_alpha(if path_visible { 0.5 } else { 0.0 });

        for segment in path.windows(2) {
            let (start_position, end_position) = (segment[0], segment[1]);

            let middle_position = (start_position + end_position) / 2.0;
            let segment_length = start_position.distance(end_position);
            let segment_angle = (end_position - start_position).angle_to(Vec2::X);

            commands.entity(game_tilemap_entity).with_child((
                EnemyPath::new(path_index),
                Mesh2d(meshes.add(RoundedRectangle::new(
                    segment_length * game_tilemap.get_tile_size() as f32,
                    2.0,
                    1.0,
                ))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: path_color,
                    alpha_mode: AlphaMode2d::Blend,
                    ..default()
                })),
                TilePosition::from_vec2(middle_position).with_z(path_index as f32 * 1e-6),
                Transform::from_rotation(Quat::from_rotation_z(segment_angle)),
            ));
        }
    }
}

fn update_enemy_paths(
    mut enemy_paths: Query<&mut EnemyPath>,
    selected_level: Res<Level>,
    game_waves: Res<GameWaves>,
    game_config: Res<Persistent<GameConfig>>,
) {
    let Some(wave) = selected_level.get_wave(if game_waves.get_state() == WaveState::Completed {
        game_waves.get_current().saturating_add(1)
    } else {
        game_waves.get_current()
    }) else {
        return;
    };

    let suitable_wave_state = match game_config.get_enemy_path_visibility() {
        EnemyPathVisibility::PreWaveVisible => {
            matches!(
                game_waves.get_state(),
                WaveState::NotStarted | WaveState::Completed
            )
        }
        EnemyPathVisibility::AlwaysVisible => true,
        EnemyPathVisibility::NeverVisible => false,
    };

    for mut enemy_path in enemy_paths.iter_mut() {
        let path_used = wave
            .get_enemies()
            .iter()
            .any(|wave_enemies| wave_enemies.get_path_index() == enemy_path.get_path_index());

        let path_visible = path_used && suitable_wave_state;

        enemy_path.set_visible(path_visible);
    }
}

fn update_enemy_paths_alpha(
    enemy_paths: Query<(&EnemyPath, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (enemy_path, enemy_path_mesh_material_2d) in enemy_paths.iter() {
        if let Some(enemy_path_color_material) = materials.get_mut(&enemy_path_mesh_material_2d.0) {
            let target_alpha = if enemy_path.get_visible() { 0.5 } else { 0.0 };
            let current_alpha = enemy_path_color_material.color.alpha();

            if current_alpha == target_alpha {
                continue;
            }
            if (current_alpha - target_alpha).abs() > 1e-3 {
                let new_alpha = current_alpha.lerp(
                    target_alpha,
                    (time.delta_secs() / Duration::from_millis(50).as_secs_f32()).clamp(0.0, 1.0),
                );
                enemy_path_color_material.color.set_alpha(new_alpha);
            } else {
                enemy_path_color_material.color.set_alpha(target_alpha);
            }
        }
    }
}
