use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_persistent::Persistent;

use crate::{
    assets::{audio::game::GameAudioAssets, entities::tile::TileAssets},
    audio::GameAudioVolume,
    entities::projectile::Projectile,
    game::{GameState, MainTilemap},
};

use super::{
    tilemap::{movement::TileMovement, position::TilePosition},
    unit::Unit,
};

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(unused)]
pub enum StructureVariant {
    Tower,
    None,
}

#[derive(Component, Clone)]
#[require(TilePosition, Sprite)]
pub struct Structure {
    variant: StructureVariant,
    damage: u32,
    fire_radius: f32,
    fire_rate: Duration,
    cooldown: Duration,
}

impl Default for Structure {
    fn default() -> Self {
        Self {
            variant: StructureVariant::None,
            damage: 0,
            fire_radius: 0.0,
            fire_rate: Duration::ZERO,
            cooldown: Duration::ZERO,
        }
    }
}

#[allow(unused)]
impl Structure {
    pub fn new(
        variant: StructureVariant,
        damage: u32,
        fire_radius: f32,
        fire_rate: Duration,
    ) -> Self {
        Self {
            variant,
            damage,
            fire_radius,
            fire_rate,
            ..default()
        }
    }
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
    pub fn get_radius(&self) -> f32 {
        self.fire_radius
    }
}

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_structure.run_if(in_state(GameState::InGame)));
    }
}

fn update_structure(
    mut commands: Commands,
    mut structures: Query<(&mut Structure, &TilePosition, &mut Transform)>,
    main_tilemap: Query<Entity, With<MainTilemap>>,
    units: Query<(Entity, &TileMovement, &TilePosition), With<Unit>>,
    tile_assets: Res<TileAssets>,
    game_audio_assets: Res<GameAudioAssets>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    time: Res<Time>,
) {
    let mut sorted_units = units.iter().collect::<Vec<_>>();
    sorted_units.sort_by(
        |(_unit_a_entity, unit_a_movement, _unit_a_tile_position),
         (_unit_b_entity, unit_b_movement, _unit_b_tile_position)| {
            unit_b_movement
                .get_progress()
                .partial_cmp(&unit_a_movement.get_progress())
                .unwrap_or(std::cmp::Ordering::Equal)
        },
    );

    for (mut structure, structure_tile_position, mut structure_transform) in structures.iter_mut() {
        if structure.cooldown > Duration::ZERO {
            structure.cooldown = structure
                .cooldown
                .checked_sub(Duration::from_secs_f32(time.delta_secs()))
                .unwrap_or_default();
            continue;
        }

        for (unit_entity, unit_movement, unit_tile_position) in &sorted_units {
            if structure_tile_position
                .as_vec2()
                .distance(unit_tile_position.as_vec2())
                <= structure.fire_radius
            {
                let projectile_duration = Duration::from_secs_f32(0.1);

                let unit_progress_on_hit = unit_movement.get_progress()
                    + projectile_duration.as_secs_f32()
                        / unit_movement.get_duration().as_secs_f32();

                commands
                    .entity(main_tilemap.get_single().unwrap())
                    .with_child((
                        Projectile::new(*unit_entity, structure.damage),
                        TileMovement::new(
                            vec![
                                structure_tile_position.as_vec2(),
                                unit_movement.position_at_progress(unit_progress_on_hit),
                            ],
                            projectile_duration,
                        ),
                        Transform::from_scale(Vec3::ZERO),
                        Sprite {
                            image: tile_assets.forest_tilemap.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: tile_assets.forest_tilemap_atlas.clone(),
                                index: 191,
                            }),
                            ..default()
                        },
                    ));
                commands.spawn((
                    AudioPlayer::new(game_audio_assets.get_random_shoot().clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Once,
                        volume: Volume::new(game_audio_volume.get_sfx_volume()),
                        ..default()
                    },
                ));
                structure.cooldown = structure.fire_rate;

                let unit_direction = structure_tile_position.as_vec2()
                    - unit_movement.position_at_progress(unit_progress_on_hit);
                let scale_x = if unit_direction.x < 0.0 { 1.0 } else { -1.0 };
                structure_transform.scale.x = scale_x;

                break;
            }
        }
    }
}
