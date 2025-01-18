use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_persistent::Persistent;

use crate::{
    assets::audio::game::GameAudioAssets,
    audio::GameAudioVolume,
    entities::projectile::Projectile,
    game::{GameState, GameTilemap},
};

use super::{
    tile::{
        movement::TileMovement,
        position::TilePosition,
        sprite::{ProjectileTileSpriteVariant, TileSprite, TileSpriteVariant},
    },
    unit::Unit,
};

pub struct StructureVariantConfig {
    damage: u32,
    fire_radius: f32,
    fire_rate: Duration,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(unused)]
pub enum StructureVariant {
    Soldier,
    SoldierFast,
    SoldierStrong,
    Empty,
}

impl StructureVariant {
    pub fn to_string(&self) -> String {
        match self {
            StructureVariant::Soldier => "soldier".to_string(),
            StructureVariant::SoldierFast => "soldier fast".to_string(),
            StructureVariant::SoldierStrong => "soldier strong".to_string(),
            StructureVariant::Empty => "empty".to_string(),
        }
    }
    pub fn get_config(&self) -> StructureVariantConfig {
        match self {
            StructureVariant::Soldier => StructureVariantConfig {
                damage: 10,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(0.5),
            },
            StructureVariant::SoldierFast => StructureVariantConfig {
                damage: 10,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(0.25),
            },
            StructureVariant::SoldierStrong => StructureVariantConfig {
                damage: 50,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(1.5),
            },
            StructureVariant::Empty => StructureVariantConfig {
                damage: 0,
                fire_radius: 0.0,
                fire_rate: Duration::ZERO,
            },
        }
    }
}

#[derive(Component, Clone)]
#[require(TilePosition)]
pub struct Structure {
    variant: StructureVariant,
    damage: u32,
    fire_radius: f32,
    fire_rate: Duration,
    cooldown: Duration,
    update_required: bool,
}

impl Default for Structure {
    fn default() -> Self {
        Self {
            variant: StructureVariant::Empty,
            damage: 0,
            fire_radius: 0.0,
            fire_rate: Duration::ZERO,
            cooldown: Duration::ZERO,
            update_required: true,
        }
    }
}

#[allow(unused)]
impl Structure {
    pub fn new(variant: StructureVariant) -> Self {
        Self {
            variant,
            ..default()
        }
    }
    pub fn get_variant(&self) -> StructureVariant {
        self.variant
    }
    pub fn set_variant(&mut self, variant: StructureVariant) {
        self.variant = variant;
        self.update_required = true;
    }
    pub fn set_damage(&mut self, damage: u32) {
        self.damage = damage;
    }
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
    pub fn set_fire_radius(&mut self, fire_radius: f32) {
        self.fire_radius = fire_radius;
    }
    pub fn get_fire_radius(&self) -> f32 {
        self.fire_radius
    }
    pub fn set_fire_rate(&mut self, fire_rate: Duration) {
        self.fire_rate = fire_rate;
    }
    pub fn get_fire_rate(&self) -> Duration {
        self.fire_rate
    }
    pub fn get_cooldown(&self) -> Duration {
        self.cooldown
    }
    pub fn set_cooldown(&mut self, cooldown: Duration) {
        self.cooldown = cooldown;
    }
    pub fn update_cooldown(&mut self) {
        self.cooldown = self.fire_rate;
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
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
    mut structures: Query<(
        &mut Structure,
        &TilePosition,
        &mut TileSprite,
        &mut Transform,
    )>,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    units: Query<(Entity, &TileMovement, &TilePosition), With<Unit>>,
    game_audio_assets: Res<GameAudioAssets>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    time: Res<Time>,
) {
    let mut sorted_units = units.iter().collect::<Vec<_>>();
    sorted_units.sort_by(|(_, unit_a_movement, _), (_, unit_b_movement, _)| {
        unit_b_movement
            .get_progress()
            .partial_cmp(&unit_a_movement.get_progress())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for (mut structure, structure_tile_position, mut tile_sprite, mut structure_transform) in
        structures.iter_mut()
    {
        if structure.get_update_required() == true {
            tile_sprite.set_variant(TileSpriteVariant::Structure(structure.get_variant().into()));
            let config = structure.get_variant().get_config();
            structure.set_damage(config.damage);
            structure.set_fire_radius(config.fire_radius);
            structure.set_fire_rate(config.fire_rate);
            structure.set_update_required(false);
        }

        if structure.get_variant() == StructureVariant::Empty {
            continue;
        }
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
                <= structure.get_fire_radius()
            {
                let projectile_duration = Duration::from_secs_f32(0.1);

                let unit_progress_on_hit = unit_movement.get_progress()
                    + projectile_duration.as_secs_f32()
                        / unit_movement.get_duration().as_secs_f32();

                commands
                    .entity(game_tilemap.get_single().unwrap())
                    .with_child((
                        Projectile::new(*unit_entity, structure.get_damage()),
                        TileMovement::new(
                            vec![
                                structure_tile_position.as_vec2(),
                                unit_movement.position_at_progress(unit_progress_on_hit),
                            ],
                            projectile_duration,
                        ),
                        TileSprite::new(TileSpriteVariant::Projectile(
                            ProjectileTileSpriteVariant::Bullet,
                        )),
                        Transform::from_scale(Vec3::ZERO),
                    ));
                commands.spawn((
                    AudioPlayer::new(game_audio_assets.get_random_shoot().clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Once,
                        volume: Volume::new(game_audio_volume.get_sfx_volume()),
                        ..default()
                    },
                ));

                structure.update_cooldown();

                let unit_direction = structure_tile_position.as_vec2()
                    - unit_movement.position_at_progress(unit_progress_on_hit);
                let scale_x = if unit_direction.x < 0.0 { 1.0 } else { -1.0 };
                structure_transform.scale.x = scale_x;

                break;
            }
        }
    }
}
