use std::f32::consts::PI;

use bevy::prelude::*;

use crate::game::GameState;

use super::{
    tile::{movement::TileMovement, position::TilePosition, sprite::TileSprite},
    unit::{health::UnitHealth, Unit},
};

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(unused)]
pub enum ProjectileVariant {
    Bullet,
}

#[derive(Component)]
#[require(TilePosition)]
pub struct Projectile {
    variant: ProjectileVariant,
    target: Entity,
    damage: u32,
}

impl Projectile {
    pub fn new(variant: ProjectileVariant, target: Entity, damage: u32) -> Self {
        Self {
            variant,
            target,
            damage,
        }
    }
    pub fn get_variant(&self) -> ProjectileVariant {
        self.variant
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_projectile);
        app.add_systems(
            Update,
            update_projectile.run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_projectile(
    mut commands: Commands,
    projectiles: Query<(Entity, &Projectile), Added<Projectile>>,
) {
    for (projectile_entity, projectile) in projectiles.iter() {
        commands.entity(projectile_entity).insert((
            TileSprite::new(projectile.get_variant().into()),
            Transform::from_scale(Vec3::ZERO),
        ));
    }
}

fn update_projectile(
    mut commands: Commands,
    mut projectiles: Query<
        (
            &Projectile,
            Entity,
            &TileMovement,
            &mut TilePosition,
            &mut Transform,
        ),
        With<Projectile>,
    >,
    mut units: Query<&mut UnitHealth, With<Unit>>,
) {
    for (
        projectile,
        projectile_entity,
        projectile_movement,
        mut projectile_tile_position,
        mut projectile_transform,
    ) in projectiles.iter_mut()
    {
        if projectile_movement.get_progress() >= 1.0 {
            commands.entity(projectile_entity).despawn_recursive();
            if let Ok(mut unit_health) = units.get_mut(projectile.target) {
                unit_health.damage(projectile.damage);
            }
            continue;
        }
        projectile_tile_position.set_from_vec2(projectile_movement.get_position());

        let direction = (projectile_movement.get_position()
            - projectile_movement.get_previous_position())
        .normalize_or_zero();

        projectile_transform.rotation =
            Quat::from_rotation_z(direction.x.atan2(direction.y) - PI / 2.0);
        projectile_transform.scale = Vec3::new(0.5, 0.5, 1.0);
    }
}
