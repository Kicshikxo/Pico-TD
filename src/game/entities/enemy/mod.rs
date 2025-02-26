pub mod health;
pub mod health_bar;

use std::{
    f32::consts::{FRAC_PI_2, PI, TAU},
    time::Duration,
};

use bevy::prelude::*;
use serde::Deserialize;

use crate::game::{
    assets::sprites::entity::EnemySpriteVariant,
    entities::{
        enemy::{
            health::EnemyHealth,
            health_bar::{HealthBar, HealthBarPlugin},
        },
        tile::{
            movement::TileMovement,
            position::TilePosition,
            sprite::{TileSprite, TileSpriteVariant},
        },
    },
    player::Player,
    {GameSpeed, GameState, GameTilemap},
};

pub struct EnemyVariantConfig {
    health: u32,
    damage: u32,
    kill_reward: u32,
    sprite_variant: EnemySpriteVariant,
    sprite_scale: Vec3,
    z_position: f32,
}

impl EnemyVariantConfig {
    pub fn get_health(&self) -> u32 {
        self.health
    }
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
    pub fn get_kill_reward(&self) -> u32 {
        self.kill_reward
    }
    pub fn get_sprite_variant(&self) -> EnemySpriteVariant {
        self.sprite_variant
    }
    pub fn get_sprite_scale(&self) -> Vec3 {
        self.sprite_scale
    }
    pub fn get_z_position(&self) -> f32 {
        self.z_position
    }
}

#[derive(Clone, Copy, PartialEq, Deserialize)]
pub enum EnemyLevel {
    Mk1,
    Mk2,
    Mk3,
    Mk4,
    Mk5,
}

impl EnemyLevel {
    pub fn as_index(&self) -> u32 {
        match self {
            EnemyLevel::Mk1 => 0,
            EnemyLevel::Mk2 => 1,
            EnemyLevel::Mk3 => 2,
            EnemyLevel::Mk4 => 3,
            EnemyLevel::Mk5 => 4,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Deserialize)]
pub enum EnemyVariant {
    Dron(EnemyLevel),
    Truck(EnemyLevel),
    Tank(EnemyLevel),
    Plane(EnemyLevel),
    Helicopter(EnemyLevel),
    Boat(EnemyLevel),
    Submarine(EnemyLevel),
}

impl EnemyVariant {
    pub fn get_config(&self) -> EnemyVariantConfig {
        match self {
            EnemyVariant::Dron(level) => EnemyVariantConfig {
                health: 100 + 100 * level.as_index(),
                damage: 1 + 1 * level.as_index(),
                kill_reward: 1 + 1 * level.as_index(),
                sprite_variant: match level {
                    EnemyLevel::Mk1 => EnemySpriteVariant::DronGray,
                    EnemyLevel::Mk2 => EnemySpriteVariant::DronRed,
                    EnemyLevel::Mk3 => EnemySpriteVariant::DronGreen,
                    EnemyLevel::Mk4 => EnemySpriteVariant::DronBlue,
                    EnemyLevel::Mk5 => EnemySpriteVariant::DronYellow,
                },
                sprite_scale: Vec3::new(0.67, 0.67, 1.0),
                z_position: 0.03,
            },
            EnemyVariant::Truck(level) => EnemyVariantConfig {
                health: 100 + 100 * level.as_index(),
                damage: 5 + 1 * level.as_index(),
                kill_reward: 1 + 1 * level.as_index(),
                sprite_variant: match level {
                    EnemyLevel::Mk1 => EnemySpriteVariant::TruckGray,
                    EnemyLevel::Mk2 => EnemySpriteVariant::TruckRed,
                    EnemyLevel::Mk3 => EnemySpriteVariant::TruckGreen,
                    EnemyLevel::Mk4 => EnemySpriteVariant::TruckBlue,
                    EnemyLevel::Mk5 => EnemySpriteVariant::TruckYellow,
                },
                sprite_scale: Vec3::new(0.75, 0.75, 1.0),
                z_position: 0.01,
            },
            EnemyVariant::Tank(level) => EnemyVariantConfig {
                health: 1000 + 200 * level.as_index(),
                damage: 10 + 2 * level.as_index(),
                kill_reward: 10 + 10 * level.as_index(),
                sprite_variant: match level {
                    EnemyLevel::Mk1 => EnemySpriteVariant::TankGray,
                    EnemyLevel::Mk2 => EnemySpriteVariant::TankRed,
                    EnemyLevel::Mk3 => EnemySpriteVariant::TankGreen,
                    EnemyLevel::Mk4 => EnemySpriteVariant::TankBlue,
                    EnemyLevel::Mk5 => EnemySpriteVariant::TankYellow,
                },
                sprite_scale: Vec3::new(0.9, 0.9, 1.0),
                z_position: 0.02,
            },
            EnemyVariant::Plane(level) => EnemyVariantConfig {
                health: 500 + 100 * level.as_index(),
                damage: 5 + 1 * level.as_index(),
                kill_reward: 5 + 5 * level.as_index(),
                sprite_variant: match level {
                    EnemyLevel::Mk1 => EnemySpriteVariant::PlaneGray,
                    EnemyLevel::Mk2 => EnemySpriteVariant::PlaneRed,
                    EnemyLevel::Mk3 => EnemySpriteVariant::PlaneGreen,
                    EnemyLevel::Mk4 => EnemySpriteVariant::PlaneBlue,
                    EnemyLevel::Mk5 => EnemySpriteVariant::PlaneYellow,
                },
                sprite_scale: Vec3::new(1.0, 1.0, 1.0),
                z_position: 0.05,
            },
            EnemyVariant::Helicopter(level) => EnemyVariantConfig {
                health: 300 + 100 * level.as_index(),
                damage: 3 + 1 * level.as_index(),
                kill_reward: 3 + 3 * level.as_index(),
                sprite_variant: match level {
                    EnemyLevel::Mk1 => EnemySpriteVariant::HelicopterGray,
                    EnemyLevel::Mk2 => EnemySpriteVariant::HelicopterRed,
                    EnemyLevel::Mk3 => EnemySpriteVariant::HelicopterGreen,
                    EnemyLevel::Mk4 => EnemySpriteVariant::HelicopterBlue,
                    EnemyLevel::Mk5 => EnemySpriteVariant::HelicopterYellow,
                },
                sprite_scale: Vec3::new(1.0, 1.0, 1.0),
                z_position: 0.04,
            },
            EnemyVariant::Boat(level) => EnemyVariantConfig {
                health: 200 + 100 * level.as_index(),
                damage: 2 + 1 * level.as_index(),
                kill_reward: 5 + 5 * level.as_index(),
                sprite_variant: match level {
                    EnemyLevel::Mk1 => EnemySpriteVariant::BoatGray,
                    EnemyLevel::Mk2 => EnemySpriteVariant::BoatRed,
                    EnemyLevel::Mk3 => EnemySpriteVariant::BoatGreen,
                    EnemyLevel::Mk4 => EnemySpriteVariant::BoatBlue,
                    EnemyLevel::Mk5 => EnemySpriteVariant::BoatYellow,
                },
                sprite_scale: Vec3::new(0.75, 0.75, 1.0),
                z_position: 0.02,
            },
            EnemyVariant::Submarine(level) => EnemyVariantConfig {
                health: 500 + 100 * level.as_index(),
                damage: 5 + 1 * level.as_index(),
                kill_reward: 5 + 5 * level.as_index(),
                sprite_variant: match level {
                    EnemyLevel::Mk1 => EnemySpriteVariant::SubmarineGray,
                    EnemyLevel::Mk2 => EnemySpriteVariant::SubmarineRed,
                    EnemyLevel::Mk3 => EnemySpriteVariant::SubmarineGreen,
                    EnemyLevel::Mk4 => EnemySpriteVariant::SubmarineBlue,
                    EnemyLevel::Mk5 => EnemySpriteVariant::SubmarineYellow,
                },
                sprite_scale: Vec3::new(0.75, 0.75, 1.0),
                z_position: 0.01,
            },
        }
    }
}

#[derive(Component)]
#[require(EnemyHealth, TileMovement, TilePosition)]
pub struct Enemy {
    variant: EnemyVariant,
    update_required: bool,
}

#[allow(unused)]
impl Enemy {
    pub fn new(variant: EnemyVariant) -> Self {
        Self {
            variant,
            update_required: true,
        }
    }
    pub fn get_variant(&self) -> EnemyVariant {
        self.variant
    }
    pub fn set_variant(&mut self, variant: EnemyVariant) {
        self.variant = variant;
        self.update_required = true;
    }
    pub fn get_damage(&self) -> u32 {
        self.get_variant().get_config().get_damage()
    }
    pub fn get_kill_reward(&self) -> u32 {
        self.get_variant().get_config().get_kill_reward()
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HealthBarPlugin);

        app.add_systems(PreUpdate, init_enemy);

        app.add_systems(
            Update,
            update_enemy_movement.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            PostUpdate,
            update_enemy_health.run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_enemy(
    mut commands: Commands,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    enemies: Query<(Entity, &Enemy), Added<Enemy>>,
) {
    for (enemy_entity, enemy) in enemies.iter() {
        commands.entity(enemy_entity).insert((
            EnemyHealth::new(enemy.get_variant().get_config().get_health()),
            TileSprite::new(enemy.get_variant().into()),
        ));

        commands
            .entity(game_tilemap.single())
            .with_child((HealthBar::new(enemy_entity),));
    }
}

fn update_enemy_movement(
    mut commands: Commands,
    mut enemies: Query<
        (
            Entity,
            &mut Enemy,
            &mut EnemyHealth,
            &TileMovement,
            &mut TilePosition,
            &mut TileSprite,
            &mut Transform,
        ),
        With<Enemy>,
    >,
    mut player: ResMut<Player>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    for (
        enemy_entity,
        mut enemy,
        mut enemy_health,
        enemy_movement,
        mut enemy_tile_position,
        mut enemy_tile_sprite,
        mut enemy_transform,
    ) in enemies.iter_mut()
    {
        if enemy_movement.get_progress() >= 1.0 {
            commands.entity(enemy_entity).despawn_recursive();
            player.get_health_mut().damage(enemy.get_damage());
            continue;
        }
        if enemy.get_update_required() == true {
            enemy_tile_sprite.set_variant(TileSpriteVariant::Enemy(enemy.get_variant().into()));
            let config = enemy.get_variant().get_config();
            enemy_health.set_max(config.get_health());
            enemy_health.heal(config.get_health());
            enemy_tile_position.set_z(config.get_z_position());
            enemy_transform.scale = config.get_sprite_scale();
            enemy.set_update_required(false);
        }
        enemy_tile_position.set_from_vec2(enemy_movement.get_position());

        let (current_z, _current_y, _current_x) = enemy_transform.rotation.to_euler(EulerRot::ZYX);
        let direction = (enemy_movement.get_position() - enemy_movement.get_previous_position())
            .normalize_or_zero();

        let target_z = direction.x.atan2(direction.y) - FRAC_PI_2;
        let rotation_z = current_z
            + ((target_z - current_z + PI).rem_euclid(TAU) - PI)
                * (time.delta_secs() * game_speed.as_f32() * enemy_movement.get_speed() * PI);
        enemy_transform.rotation = Quat::from_rotation_z(rotation_z);
    }
}

fn update_enemy_health(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Enemy, &mut EnemyHealth, &mut Sprite, &Transform), With<Enemy>>,
    mut health_bars: Query<&mut HealthBar>,
    mut player: ResMut<Player>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    for (enemy_entity, enemy, mut enemy_health, mut enemy_sprite, _enemy_transform) in
        enemies.iter_mut()
    {
        if enemy_health.get_current() == 0 {
            commands.entity(enemy_entity).despawn_recursive();
            player.get_money_mut().increase(enemy.get_kill_reward());
            continue;
        }

        let target_enemy_sprite_color = LinearRgba::WHITE.to_vec3();
        let current_enemy_sprite_color = enemy_sprite.color.to_linear().to_vec3();

        if current_enemy_sprite_color != target_enemy_sprite_color {
            if (current_enemy_sprite_color - target_enemy_sprite_color).length() > 1e-3 {
                enemy_sprite.color = LinearRgba::from_vec3(
                    current_enemy_sprite_color.lerp(
                        target_enemy_sprite_color,
                        (time.delta_secs() * game_speed.as_f32()
                            / Duration::from_millis(250).as_secs_f32())
                        .clamp(0.0, 1.0),
                    ),
                )
                .into();
            } else {
                enemy_sprite.color = LinearRgba::from_vec3(target_enemy_sprite_color).into();
            }
        }

        if enemy_health.get_update_required() == true {
            enemy_sprite.color = Color::srgb(1.0, 0.0, 0.0);

            for mut health_bar in health_bars.iter_mut() {
                if health_bar.get_enemy_entity() == enemy_entity {
                    health_bar.set_update_required(true);
                }
            }

            enemy_health.set_update_required(false);
        }
    }
}
