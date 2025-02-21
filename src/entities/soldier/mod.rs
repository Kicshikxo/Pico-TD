pub mod cooldown_indicator;
pub mod fire_radius;
pub mod projectile;

use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_persistent::Persistent;
use cooldown_indicator::{CooldownIndicator, CooldownIndicatorPlugin};
use fire_radius::{FireRadius, FireRadiusPlugin};
use projectile::{Projectile, ProjectilePlugin, ProjectileVariant};

use crate::{
    assets::audio::game::GameAudioAssets,
    audio::{GameAudio, GameAudioVolume},
    game::{GameSpeed, GameState, GameTilemap},
};

use super::{
    enemy::{health::EnemyHealth, Enemy},
    tile::{
        movement::TileMovement,
        position::TilePosition,
        sprite::{TileSprite, TileSpriteVariant},
    },
};

pub struct SoldierVariantConfig {
    price: u32,
    sell_price: u32,
    damage: u32,
    fire_radius: f32,
    fire_rate: Duration,
    projectile_variant: ProjectileVariant,
}

impl SoldierVariantConfig {
    pub fn get_price(&self) -> u32 {
        self.price
    }
    pub fn get_sell_price(&self) -> u32 {
        self.sell_price
    }
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
pub enum SoldierVariant {
    Soldier,
    SoldierFast,
    SoldierStrong,
    SoldierSniper,
    RocketLauncher,
}

impl SoldierVariant {
    pub fn to_string(&self) -> String {
        match self {
            SoldierVariant::Soldier => "ui.soldier.soldier".to_string(),
            SoldierVariant::SoldierFast => "ui.soldier.soldier_fast".to_string(),
            SoldierVariant::SoldierStrong => "ui.soldier.soldier_strong".to_string(),
            SoldierVariant::SoldierSniper => "ui.soldier.soldier_sniper".to_string(),
            SoldierVariant::RocketLauncher => "ui.soldier.rocket_launcher".to_string(),
        }
    }
    pub fn get_config(&self) -> SoldierVariantConfig {
        match self {
            SoldierVariant::Soldier => SoldierVariantConfig {
                price: 50,
                sell_price: 35,
                damage: 25,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(0.5),
                projectile_variant: ProjectileVariant::Bullet,
            },
            SoldierVariant::SoldierFast => SoldierVariantConfig {
                price: 100,
                sell_price: 70,
                damage: 10,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(0.2),
                projectile_variant: ProjectileVariant::Bullet,
            },
            SoldierVariant::SoldierStrong => SoldierVariantConfig {
                price: 150,
                sell_price: 105,
                damage: 50,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(1.0),
                projectile_variant: ProjectileVariant::Bullet,
            },
            SoldierVariant::SoldierSniper => SoldierVariantConfig {
                price: 200,
                sell_price: 140,
                damage: 150,
                fire_radius: 7.0,
                fire_rate: Duration::from_secs_f32(5.0),
                projectile_variant: ProjectileVariant::Bullet,
            },
            SoldierVariant::RocketLauncher => SoldierVariantConfig {
                price: 250,
                sell_price: 175,
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
pub struct Soldier {
    variant: SoldierVariant,
    damage: u32,
    fire_radius: f32,
    fire_rate: Duration,
    cooldown: Duration,
    update_required: bool,
}

#[allow(unused)]
impl Soldier {
    pub fn new(variant: SoldierVariant) -> Self {
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
    pub fn get_variant(&self) -> SoldierVariant {
        self.variant
    }
    pub fn set_variant(&mut self, variant: SoldierVariant) {
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
    pub fn get_cooldown_percentage(&self) -> f32 {
        (self.get_cooldown().as_secs_f32() / self.get_fire_rate().as_secs_f32()).clamp(0.0, 1.0)
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

pub struct SoldierPlugin;

impl Plugin for SoldierPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CooldownIndicatorPlugin, FireRadiusPlugin, ProjectilePlugin));

        app.add_systems(Update, init_soldier);
        app.add_systems(
            Update,
            (update_soldier, update_soldier_cooldown).run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_soldier(
    mut commands: Commands,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    mut soldiers: Query<(Entity, &Soldier, &mut TilePosition), Added<Soldier>>,
) {
    for (soldier_entity, soldier, mut soldier_tile_position) in soldiers.iter_mut() {
        commands
            .entity(soldier_entity)
            .insert(TileSprite::new(soldier.get_variant().into()));
        soldier_tile_position.set_z(1.0);

        commands
            .entity(game_tilemap.single())
            .with_child(FireRadius::new(soldier_entity));

        commands
            .entity(game_tilemap.single())
            .with_child(CooldownIndicator::new(soldier_entity));
    }
}

fn update_soldier(
    mut commands: Commands,
    mut soldiers: Query<(&mut Soldier, &TilePosition, &mut TileSprite, &mut Transform)>,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    enemies: Query<(Entity, &EnemyHealth, &TileMovement, &TilePosition), With<Enemy>>,
    projectiles: Query<&Projectile>,
    game_audio: Query<Entity, With<GameAudio>>,
    game_audio_assets: Res<GameAudioAssets>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
) {
    let mut sorted_enemies = enemies.iter().collect::<Vec<_>>();
    sorted_enemies.sort_by(|(_, _, enemy_a_movement, _), (_, _, enemy_b_movement, _)| {
        enemy_b_movement
            .get_progress()
            .partial_cmp(&enemy_a_movement.get_progress())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut projectiles = projectiles.iter().cloned().collect::<Vec<Projectile>>();

    for (mut soldier, soldier_tile_position, mut soldier_tile_sprite, mut soldier_transform) in
        soldiers.iter_mut()
    {
        if soldier.get_update_required() == true {
            soldier_tile_sprite
                .set_variant(TileSpriteVariant::Soldier(soldier.get_variant().into()));
            let config = soldier.get_variant().get_config();
            soldier.set_damage(config.get_damage());
            soldier.set_fire_radius(config.get_fire_radius());
            soldier.set_fire_rate(config.get_fire_rate());
            soldier.set_update_required(false);
        }

        if soldier.get_cooldown() > Duration::ZERO {
            continue;
        }

        for (enemy_entity, enemy_health, enemy_movement, enemy_tile_position) in &sorted_enemies {
            if soldier_tile_position
                .as_vec2()
                .distance(enemy_tile_position.as_vec2())
                <= soldier.get_fire_radius()
            {
                if enemy_health.get_current().saturating_sub(
                    projectiles
                        .iter()
                        .filter(|projectile| projectile.get_target() == *enemy_entity)
                        .map(|projectile| projectile.get_damage())
                        .sum(),
                ) == 0
                {
                    continue;
                }

                let projectile_variant =
                    soldier.get_variant().get_config().get_projectile_variant();
                let projectile_duration = projectile_variant.get_config().get_duration();

                let enemy_progress_on_hit = enemy_movement.get_progress()
                    + projectile_duration.as_secs_f32()
                        / enemy_movement.get_duration().as_secs_f32();

                let projectile =
                    Projectile::new(projectile_variant, *enemy_entity, soldier.get_damage());
                commands.entity(game_tilemap.single()).with_child((
                    projectile,
                    TileMovement::new(
                        vec![
                            soldier_tile_position.as_vec2(),
                            enemy_movement.position_at_progress(enemy_progress_on_hit),
                        ],
                        projectile_duration,
                        None,
                    ),
                ));
                projectiles.push(projectile);

                commands.entity(game_audio.single()).with_child((
                    AudioPlayer::new(game_audio_assets.get_random_shoot().clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Remove,
                        volume: Volume::new(game_audio_volume.get_sfx_volume()),
                        ..default()
                    },
                ));

                soldier.update_cooldown();

                let enemy_direction = soldier_tile_position.as_vec2()
                    - enemy_movement.position_at_progress(enemy_progress_on_hit);
                let scale_x = if enemy_direction.x < 0.0 { 1.0 } else { -1.0 };
                soldier_transform.scale.x = scale_x;

                break;
            }
        }
    }
}

fn update_soldier_cooldown(
    mut soldiers: Query<(Entity, &mut Soldier)>,
    mut cooldown_indicators: Query<&mut CooldownIndicator>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    for (soldier_entity, mut soldier) in soldiers.iter_mut() {
        if soldier.get_cooldown() > Duration::ZERO {
            soldier.decrease_cooldown(Duration::from_secs_f32(
                time.delta_secs() * game_speed.as_f32(),
            ));

            for mut cooldown_indicator in cooldown_indicators.iter_mut() {
                if cooldown_indicator.get_soldier_entity() == soldier_entity {
                    cooldown_indicator.set_update_required(true);
                    break;
                }
            }
        }
    }
}
