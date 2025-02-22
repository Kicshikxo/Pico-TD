pub mod health;
pub mod health_bar;

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::prelude::*;
use health::EnemyHealth;
use health_bar::{HealthBar, HealthBarPlugin};
use serde::Deserialize;

use crate::{
    game::{GameSpeed, GameState, GameTilemap},
    player::Player,
};

use super::tile::{
    movement::TileMovement,
    position::TilePosition,
    sprite::{TileSprite, TileSpriteVariant},
};

pub struct EnemyVariantConfig {
    health: u32,
    damage: u32,
    kill_reward: u32,
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
    pub fn get_sprite_scale(&self) -> Vec3 {
        self.sprite_scale
    }
    pub fn get_z_position(&self) -> f32 {
        self.z_position
    }
}

#[derive(Clone, Copy, PartialEq, Deserialize)]
pub enum EnemyVariant {
    Dron,
    Truck,
    Tank,
    Plane,
    Helicopter,
    Boat,
    Submarine,
}

impl EnemyVariant {
    pub fn get_config(&self) -> EnemyVariantConfig {
        match self {
            EnemyVariant::Dron => EnemyVariantConfig {
                health: 25,
                damage: 5,
                kill_reward: 5,
                sprite_scale: Vec3::new(0.67, 0.67, 1.0),
                z_position: 0.03,
            },
            EnemyVariant::Truck => EnemyVariantConfig {
                health: 100,
                damage: 5,
                kill_reward: 20,
                sprite_scale: Vec3::new(0.75, 0.75, 1.0),
                z_position: 0.01,
            },
            EnemyVariant::Tank => EnemyVariantConfig {
                health: 300,
                damage: 5,
                kill_reward: 60,
                sprite_scale: Vec3::new(0.9, 0.9, 1.0),
                z_position: 0.02,
            },
            EnemyVariant::Plane => EnemyVariantConfig {
                health: 150,
                damage: 5,
                kill_reward: 30,
                sprite_scale: Vec3::new(1.0, 1.0, 1.0),
                z_position: 0.04,
            },
            EnemyVariant::Helicopter => EnemyVariantConfig {
                health: 100,
                damage: 5,
                kill_reward: 20,
                sprite_scale: Vec3::new(1.0, 1.0, 1.0),
                z_position: 0.05,
            },
            EnemyVariant::Boat => EnemyVariantConfig {
                health: 50,
                damage: 5,
                kill_reward: 10,
                sprite_scale: Vec3::new(0.75, 0.75, 1.0),
                z_position: 0.02,
            },
            EnemyVariant::Submarine => EnemyVariantConfig {
                health: 200,
                damage: 5,
                kill_reward: 40,
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
    damage: u32,
    kill_reward: u32,
    update_required: bool,
}

#[allow(unused)]
impl Enemy {
    pub fn new(variant: EnemyVariant) -> Self {
        let config = variant.get_config();

        Self {
            variant,
            damage: config.damage,
            kill_reward: config.kill_reward,
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
        self.damage
    }
    pub fn set_damage(&mut self, damage: u32) {
        self.damage = damage;
    }
    pub fn get_kill_reward(&self) -> u32 {
        self.kill_reward
    }
    pub fn set_kill_reward(&mut self, kill_reward: u32) {
        self.kill_reward = kill_reward;
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

        app.add_systems(Update, init_enemy);
        app.add_systems(
            Update,
            (update_enemy_movement, update_enemy_health).run_if(in_state(GameState::InGame)),
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
            enemy.set_damage(config.get_damage());
            enemy.set_kill_reward(config.get_kill_reward());
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
                * time.delta_secs()
                * game_speed.as_f32()
                * 10.0;
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

        enemy_sprite.color = LinearRgba::from_vec3(enemy_sprite.color.to_linear().to_vec3().lerp(
            LinearRgba::WHITE.to_vec3(),
            time.delta_secs() * game_speed.as_f32() * 4.0,
        ))
        .into();

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
