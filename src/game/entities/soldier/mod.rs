pub mod config;
pub mod cooldown_indicator;
pub mod fire_radius;
pub mod projectile;
pub mod projectile_blast;

use std::{ops::Deref, time::Duration};

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_persistent::Persistent;
use serde::{Deserialize, Serialize};

use crate::game::{
    assets::audio::game::GameAudioAssets,
    audio::{GameAudio, GameAudioVolume},
    entities::{
        enemy::{health::EnemyHealth, Enemy},
        soldier::{
            config::{SoldierConfig, ROCKET_LAUNCHER_LEVELS, SNIPER_LEVELS, SOLDIER_LEVELS},
            cooldown_indicator::{CooldownIndicator, CooldownIndicatorPlugin},
            fire_radius::{FireRadius, FireRadiusPlugin},
            projectile::{Projectile, ProjectilePlugin, ProjectileVariant},
            projectile_blast::ProjectileBlastPlugin,
        },
        tile::{
            movement::TileMovement,
            position::TilePosition,
            sprite::{TileSprite, TileSpriteVariant},
        },
    },
    speed::GameSpeed,
    {GameState, GameTilemap},
};

#[derive(Clone, Copy, PartialEq)]
pub enum SoldierVariant {
    Soldier { level: usize },
    RocketLauncher { level: usize },
    Sniper { level: usize },
}

impl SoldierVariant {
    pub fn to_str(&self) -> &'static str {
        match self {
            SoldierVariant::Soldier { .. } => "soldier.variant.soldier",
            SoldierVariant::RocketLauncher { .. } => "soldier.variant.rocket_launcher",
            SoldierVariant::Sniper { .. } => "soldier.variant.sniper",
        }
    }
    pub fn get_levels(&self) -> &'static [SoldierConfig] {
        match self {
            Self::Soldier { .. } => &SOLDIER_LEVELS,
            Self::RocketLauncher { .. } => &ROCKET_LAUNCHER_LEVELS,
            Self::Sniper { .. } => &SNIPER_LEVELS,
        }
    }
    pub fn get_level(&self) -> usize {
        match self {
            SoldierVariant::Soldier { level }
            | SoldierVariant::RocketLauncher { level }
            | SoldierVariant::Sniper { level } => *level,
        }
    }
    fn set_level(&mut self, new_level: usize) {
        match self {
            SoldierVariant::Soldier { level }
            | SoldierVariant::RocketLauncher { level }
            | SoldierVariant::Sniper { level } => *level = new_level,
        }
    }
    pub fn get_max_level(&self) -> usize {
        self.get_levels().len().saturating_sub(1)
    }
    pub fn is_next_level_allowed(&self) -> bool {
        self.get_level() < self.get_max_level()
    }
    pub fn get_next_level(&self) -> usize {
        self.get_level().saturating_add(1).min(self.get_max_level())
    }
    pub fn set_next_level(&mut self) {
        self.set_level(self.get_next_level());
    }
    pub fn get_config(&self) -> &SoldierConfig {
        self.get_levels().get(self.get_level()).unwrap()
    }
    pub fn get_next_level_config(&self) -> &SoldierConfig {
        self.get_levels().get(self.get_next_level()).unwrap()
    }
}

#[derive(Clone, Copy, Default)]
pub enum SoldierTargetPriority {
    #[default]
    First,
    Last,
    Strongest,
    Weakest,
    Nearest,
}

impl SoldierTargetPriority {
    pub fn to_str(&self) -> &'static str {
        match self {
            SoldierTargetPriority::First => "soldier.target_priority.first",
            SoldierTargetPriority::Last => "soldier.target_priority.last",
            SoldierTargetPriority::Strongest => "soldier.target_priority.strongest",
            SoldierTargetPriority::Weakest => "soldier.target_priority.weakest",
            SoldierTargetPriority::Nearest => "soldier.target_priority.nearest",
        }
    }
    pub fn as_index(&self) -> usize {
        match self {
            SoldierTargetPriority::First => 0,
            SoldierTargetPriority::Last => 1,
            SoldierTargetPriority::Strongest => 2,
            SoldierTargetPriority::Weakest => 3,
            SoldierTargetPriority::Nearest => 4,
        }
    }
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => SoldierTargetPriority::First,
            1 => SoldierTargetPriority::Last,
            2 => SoldierTargetPriority::Strongest,
            3 => SoldierTargetPriority::Weakest,
            4 => SoldierTargetPriority::Nearest,
            _ => SoldierTargetPriority::default(),
        }
    }
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub enum SoldierPlacement {
    #[default]
    WithConfirmation,
    WithoutConfirmation,
}

impl SoldierPlacement {
    pub fn to_str(&self) -> &'static str {
        match self {
            SoldierPlacement::WithConfirmation => "soldier.placement.with_confirmation",
            SoldierPlacement::WithoutConfirmation => "soldier.placement.without_confirmation",
        }
    }
    pub fn as_index(&self) -> usize {
        match self {
            SoldierPlacement::WithConfirmation => 0,
            SoldierPlacement::WithoutConfirmation => 1,
        }
    }
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => SoldierPlacement::WithConfirmation,
            1 => SoldierPlacement::WithoutConfirmation,
            _ => SoldierPlacement::default(),
        }
    }
}

#[derive(Component, Clone)]
#[require(TilePosition)]
pub struct Soldier {
    variant: SoldierVariant,
    cooldown: Duration,
    target_priority: SoldierTargetPriority,
    update_required: bool,
}

impl Soldier {
    pub fn new(variant: SoldierVariant) -> Self {
        Self {
            variant,
            cooldown: Duration::ZERO,
            target_priority: SoldierTargetPriority::default(),
            update_required: false,
        }
    }
    pub fn get_variant(&self) -> SoldierVariant {
        self.variant
    }
    pub fn get_variant_mut(&mut self) -> &mut SoldierVariant {
        self.set_update_required(true);
        &mut self.variant
    }
    #[allow(unused)]
    pub fn set_variant(&mut self, variant: SoldierVariant) {
        self.set_update_required(self.variant != variant);
        self.variant = variant;
    }
    pub fn get_damage(&self) -> u32 {
        self.get_config().get_damage()
    }
    pub fn get_fire_radius(&self) -> f32 {
        self.get_config().get_fire_radius()
    }
    pub fn get_fire_rate(&self) -> Duration {
        self.get_config().get_fire_rate()
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
        self.cooldown = self.get_fire_rate();
    }
    pub fn get_target_priority(&self) -> SoldierTargetPriority {
        self.target_priority
    }
    pub fn set_target_priority(&mut self, priority: SoldierTargetPriority) {
        self.target_priority = priority;
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
}

impl Deref for Soldier {
    type Target = SoldierVariant;

    fn deref(&self) -> &Self::Target {
        &self.variant
    }
}

pub struct SoldierPlugin;

impl Plugin for SoldierPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CooldownIndicatorPlugin,
            FireRadiusPlugin,
            ProjectilePlugin,
            ProjectileBlastPlugin,
        ));

        app.add_systems(PreUpdate, init_soldier);

        app.add_systems(Update, update_soldier.run_if(in_state(GameState::InGame)));
        app.add_systems(
            PostUpdate,
            update_soldier_cooldown.run_if(in_state(GameState::InGame)),
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
        soldier_tile_position.set_z(2.0);

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
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    game_audio_assets: Res<GameAudioAssets>,
) {
    let mut projectiles = projectiles.iter().cloned().collect::<Vec<Projectile>>();

    for (mut soldier, soldier_tile_position, mut soldier_tile_sprite, mut soldier_transform) in
        soldiers.iter_mut()
    {
        if soldier.get_update_required() == true {
            let variant = soldier.get_variant();
            soldier_tile_sprite.set_variant(TileSpriteVariant::Soldier(variant.into()));
            soldier.set_update_required(false);
        }

        if soldier.get_cooldown() > Duration::ZERO {
            continue;
        }

        let mut sorted_enemies = enemies
            .iter()
            .filter(|(_, _, enemy_movement, _)| enemy_movement.get_progress() > 0.0)
            .collect::<Vec<_>>();

        sorted_enemies.sort_unstable_by(
            |(_, enemy_a_health, enemy_a_movement, enemy_a_tile_position),
             (_, enemy_b_health, enemy_b_movement, enemy_b_tile_position)| {
                match soldier.get_target_priority() {
                    SoldierTargetPriority::First => enemy_b_movement
                        .get_progress()
                        .total_cmp(&enemy_a_movement.get_progress()),
                    SoldierTargetPriority::Last => enemy_a_movement
                        .get_progress()
                        .total_cmp(&enemy_b_movement.get_progress()),
                    SoldierTargetPriority::Nearest => enemy_a_tile_position
                        .as_vec2()
                        .distance(soldier_tile_position.as_vec2())
                        .total_cmp(
                            &enemy_b_tile_position
                                .as_vec2()
                                .distance(soldier_tile_position.as_vec2()),
                        )
                        .then_with(|| {
                            enemy_b_movement
                                .get_progress()
                                .total_cmp(&enemy_a_movement.get_progress())
                        }),
                    SoldierTargetPriority::Strongest => enemy_b_health
                        .get_current()
                        .cmp(&enemy_a_health.get_current())
                        .then_with(|| {
                            enemy_b_movement
                                .get_progress()
                                .total_cmp(&enemy_a_movement.get_progress())
                        }),
                    SoldierTargetPriority::Weakest => enemy_a_health
                        .get_current()
                        .cmp(&enemy_b_health.get_current())
                        .then_with(|| {
                            enemy_b_movement
                                .get_progress()
                                .total_cmp(&enemy_a_movement.get_progress())
                        }),
                }
            },
        );

        for (enemy_entity, enemy_health, enemy_movement, enemy_tile_position) in
            sorted_enemies.iter()
        {
            if soldier_tile_position
                .as_vec2()
                .distance(enemy_tile_position.as_vec2())
                > soldier.get_fire_radius()
            {
                continue;
            }
            if enemy_health.get_current()
                <= projectiles
                    .iter()
                    .filter(|projectile| projectile.get_target() == *enemy_entity)
                    .map(|projectile| projectile.get_damage())
                    .sum()
            {
                continue;
            }

            let projectile_variant = soldier.get_config().get_projectile_variant();
            let projectile_duration = projectile_variant.get_config().get_duration();

            let enemy_progress_on_hit = enemy_movement.get_progress()
                + projectile_duration.as_secs_f32() / enemy_movement.get_duration().as_secs_f32();

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
                AudioPlayer::new(match projectile_variant {
                    ProjectileVariant::Bullet => game_audio_assets.get_random_bullet_shoot(),
                    ProjectileVariant::Rocket { .. } => game_audio_assets.get_random_rocket_shoot(),
                }),
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

fn update_soldier_cooldown(
    mut soldiers: Query<(Entity, &mut Soldier)>,
    mut cooldown_indicators: Query<&mut CooldownIndicator>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    for (soldier_entity, mut soldier) in soldiers.iter_mut() {
        if soldier.get_cooldown() == Duration::ZERO {
            continue;
        }

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
