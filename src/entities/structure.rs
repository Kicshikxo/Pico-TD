use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_persistent::Persistent;

use crate::{
    assets::audio::game::GameAudioAssets,
    audio::{GameAudio, GameAudioVolume},
    entities::projectile::Projectile,
    game::{GameSpeed, GameState, GameTilemap},
};

use super::{
    projectile::ProjectileVariant,
    tile::{
        movement::TileMovement,
        position::TilePosition,
        sprite::{TileSprite, TileSpriteVariant},
    },
    unit::Unit,
};

pub struct StructureVariantConfig {
    damage: u32,
    fire_radius: f32,
    fire_rate: Duration,
    projectile_variant: ProjectileVariant,
}

impl StructureVariantConfig {
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
    pub fn get_fire_radius(&self) -> f32 {
        self.fire_radius
    }
    pub fn get_fire_rate(&self) -> Duration {
        self.fire_rate
    }
    pub fn get_projectile_variant(&self) -> ProjectileVariant {
        self.projectile_variant
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum StructureVariant {
    Soldier,
    SoldierFast,
    SoldierStrong,
    RocketLauncher,
}

impl StructureVariant {
    pub fn to_string(&self) -> String {
        match self {
            StructureVariant::Soldier => "ui.structure.soldier".to_string(),
            StructureVariant::SoldierFast => "ui.structure.soldier_fast".to_string(),
            StructureVariant::SoldierStrong => "ui.structure.soldier_strong".to_string(),
            StructureVariant::RocketLauncher => "ui.structure.rocket_launcher".to_string(),
        }
    }
    pub fn get_config(&self) -> StructureVariantConfig {
        match self {
            StructureVariant::Soldier => StructureVariantConfig {
                damage: 25,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(0.5),
                projectile_variant: ProjectileVariant::Bullet,
            },
            StructureVariant::SoldierFast => StructureVariantConfig {
                damage: 10,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(0.2),
                projectile_variant: ProjectileVariant::Bullet,
            },
            StructureVariant::SoldierStrong => StructureVariantConfig {
                damage: 50,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(1.0),
                projectile_variant: ProjectileVariant::Bullet,
            },
            StructureVariant::RocketLauncher => StructureVariantConfig {
                damage: 100,
                fire_radius: 5.0,
                fire_rate: Duration::from_secs_f32(2.0),
                projectile_variant: ProjectileVariant::Rocket,
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

impl Structure {
    pub fn new(variant: StructureVariant) -> Self {
        let config = variant.get_config();

        Self {
            variant,
            damage: config.damage,
            fire_radius: config.fire_radius,
            fire_rate: config.fire_rate,
            cooldown: Duration::ZERO,
            update_required: false,
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
    pub fn decrease_cooldown(&mut self, delta_time: Duration) {
        self.cooldown = self.cooldown.checked_sub(delta_time).unwrap_or_default();
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
        app.add_systems(
            Update,
            (init_structure, update_structure).run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_structure(
    mut commands: Commands,
    structures: Query<(Entity, &Structure), Added<Structure>>,
) {
    for (structure_entity, structure) in structures.iter() {
        commands
            .entity(structure_entity)
            .insert(TileSprite::new(structure.get_variant().into()));
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
    game_audio: Query<Entity, With<GameAudio>>,
    game_audio_assets: Res<GameAudioAssets>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    let mut sorted_units = units.iter().collect::<Vec<_>>();
    sorted_units.sort_by(|(_, unit_a_movement, _), (_, unit_b_movement, _)| {
        unit_b_movement
            .get_progress()
            .partial_cmp(&unit_a_movement.get_progress())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for (
        mut structure,
        structure_tile_position,
        mut structure_tile_sprite,
        mut structure_transform,
    ) in structures.iter_mut()
    {
        if structure.get_update_required() == true {
            structure_tile_sprite
                .set_variant(TileSpriteVariant::Structure(structure.get_variant().into()));
            let config = structure.get_variant().get_config();
            structure.set_damage(config.get_damage());
            structure.set_fire_radius(config.get_fire_radius());
            structure.set_fire_rate(config.get_fire_rate());
            structure.set_update_required(false);
        }

        if structure.get_cooldown() > Duration::ZERO {
            structure.decrease_cooldown(Duration::from_secs_f32(
                time.delta_secs() * game_speed.as_f32(),
            ));
            continue;
        }

        for (unit_entity, unit_movement, unit_tile_position) in &sorted_units {
            if structure_tile_position
                .as_vec2()
                .distance(unit_tile_position.as_vec2())
                <= structure.get_fire_radius()
            {
                let projectile_variant = structure
                    .get_variant()
                    .get_config()
                    .get_projectile_variant();
                let projectile_duration = projectile_variant.get_config().get_duration();

                let unit_progress_on_hit = unit_movement.get_progress()
                    + projectile_duration.as_secs_f32()
                        / unit_movement.get_duration().as_secs_f32();

                commands.entity(game_tilemap.single()).with_child((
                    Projectile::new(projectile_variant, *unit_entity, structure.get_damage()),
                    TileMovement::new(
                        vec![
                            structure_tile_position.as_vec2(),
                            unit_movement.position_at_progress(unit_progress_on_hit),
                        ],
                        projectile_duration,
                        None,
                    ),
                ));
                commands.entity(game_audio.single()).with_child((
                    AudioPlayer::new(game_audio_assets.get_random_shoot().clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Remove,
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
