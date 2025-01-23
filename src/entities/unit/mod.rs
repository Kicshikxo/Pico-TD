pub mod health;

use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Anchor};
use health::{UnitHealth, UnitHealthBar};
use serde::Deserialize;

use crate::game::{GameState, GameTilemap};

use super::tile::{
    movement::TileMovement,
    position::TilePosition,
    sprite::{TileSprite, TileSpriteVariant},
};

pub struct UnitVariantConfig {
    damage: u32,
    health: u32,
}

impl UnitVariantConfig {
    pub fn get_health(&self) -> u32 {
        self.health
    }
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Deserialize)]
pub enum UnitVariant {
    Truck,
    Plane,
    Tank,
    Boat,
    Submarine,
}

impl UnitVariant {
    pub fn get_config(&self) -> UnitVariantConfig {
        match self {
            UnitVariant::Truck => UnitVariantConfig {
                health: 100,
                damage: 5,
            },
            UnitVariant::Plane => UnitVariantConfig {
                health: 150,
                damage: 5,
            },
            UnitVariant::Tank => UnitVariantConfig {
                health: 300,
                damage: 5,
            },
            UnitVariant::Boat => UnitVariantConfig {
                health: 50,
                damage: 5,
            },
            UnitVariant::Submarine => UnitVariantConfig {
                health: 200,
                damage: 5,
            },
        }
    }
}

#[derive(Component)]
#[require(UnitHealth, TileMovement, TilePosition)]
pub struct Unit {
    variant: UnitVariant,
    damage: u32,
    update_required: bool,
}

#[allow(unused)]
impl Unit {
    pub fn new(variant: UnitVariant) -> Self {
        let config = variant.get_config();

        Self {
            variant,
            damage: config.damage,
            update_required: true,
        }
    }
    pub fn get_variant(&self) -> UnitVariant {
        self.variant
    }
    pub fn set_variant(&mut self, variant: UnitVariant) {
        self.variant = variant;
        self.update_required = true;
    }
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
    pub fn set_damage(&mut self, damage: u32) {
        self.damage = damage;
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_unit);
        app.add_systems(
            Update,
            (update_unit_movement, update_unit_health).run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_unit(
    mut commands: Commands,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    units: Query<(Entity, &Unit), Added<Unit>>,
) {
    for (unit_entity, unit) in units.iter() {
        commands.entity(unit_entity).insert((
            UnitHealth::new(unit.get_variant().get_config().get_health()),
            TileSprite::new(unit.get_variant().into()),
        ));

        commands
            .entity(game_tilemap.single())
            .with_child(UnitHealthBar::new(unit_entity));
    }
}

fn update_unit_movement(
    mut commands: Commands,
    mut units: Query<
        (
            Entity,
            &mut Unit,
            &mut UnitHealth,
            &TileMovement,
            &mut TilePosition,
            &mut TileSprite,
            &mut Transform,
        ),
        With<Unit>,
    >,
    time: Res<Time>,
) {
    for (
        unit_entity,
        mut unit,
        mut unit_health,
        unit_movement,
        mut unit_tile_position,
        mut unit_tile_sprite,
        mut unit_transform,
    ) in units.iter_mut()
    {
        if unit_movement.get_progress() >= 1.0 {
            commands.entity(unit_entity).despawn_recursive();
            continue;
        }
        if unit.get_update_required() == true {
            unit_tile_sprite.set_variant(TileSpriteVariant::Unit(unit.get_variant().into()));
            let config = unit.get_variant().get_config();
            unit_health.set_max(config.get_health());
            unit_health.heal(config.get_health());
            unit.set_damage(config.get_damage());
            unit.set_update_required(false);
        }
        unit_tile_position.set_from_vec2(unit_movement.get_position());

        let (current_z, _current_y, _current_x) = unit_transform.rotation.to_euler(EulerRot::ZYX);
        let direction = (unit_movement.get_position() - unit_movement.get_previous_position())
            .normalize_or_zero();

        let rotation_z = current_z.lerp(
            direction.x.atan2(direction.y) - PI / 2.0,
            time.delta_secs() * 10.0,
        );
        unit_transform.rotation = Quat::from_rotation_z(rotation_z);
        unit_transform.scale = Vec3::new(0.75, 0.75, 1.0);
    }
}

fn update_unit_health(
    mut commands: Commands,
    mut units: Query<(Entity, &mut UnitHealth, &mut Sprite, &Transform), With<Unit>>,
    mut health_bars: Query<(Entity, &UnitHealthBar, &mut Sprite, &mut Transform), Without<Unit>>,
    time: Res<Time>,
) {
    for (unit_entity, mut unit_health, mut unit_sprite, _unit_transform) in units.iter_mut() {
        if unit_health.get_current() == 0 {
            commands.entity(unit_entity).despawn_recursive();
            continue;
        }

        unit_sprite.color = LinearRgba::from_vec3(
            unit_sprite
                .color
                .to_linear()
                .to_vec3()
                .lerp(LinearRgba::WHITE.to_vec3(), time.delta_secs() * 4.0),
        )
        .into();

        if unit_health.get_damage_indicator() {
            unit_sprite.color = Color::srgb(1.0, 0.0, 0.0);
            unit_health.clear_damage_indicator();
        }
    }

    for (health_bar_entity, health_bar, mut health_bar_sprite, mut health_bar_transform) in
        health_bars.iter_mut()
    {
        if let Ok((_unit_entity, unit_health, _unit_sprite, unit_transform)) =
            units.get(health_bar.get_unit_entity())
        {
            let health_percentage = unit_health.get_current() as f32 / unit_health.get_max() as f32;

            health_bar_sprite.color = match health_percentage {
                health_percentage if health_percentage < 0.3 => Color::srgba(1.0, 0.0, 0.0, 0.75),
                health_percentage if health_percentage < 0.7 => Color::srgba(1.0, 1.0, 0.0, 0.75),
                health_percentage if health_percentage == 1.0 => Color::srgba(0.0, 0.0, 0.0, 0.0),
                _ => Color::srgba(0.0, 1.0, 0.0, 0.75),
            };

            health_bar_sprite.anchor = Anchor::TopLeft;
            health_bar_sprite.custom_size = Some(Vec2::new(16.0, 2.0));
            health_bar_transform.scale = Vec3::new(health_percentage, 1.0, 1.0);
            health_bar_transform.translation =
                unit_transform.translation + Vec3::new(-8.0, 8.0, 1.0);
        } else {
            commands.entity(health_bar_entity).despawn_recursive();
        }
    }
}
