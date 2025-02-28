use std::{f32::consts::FRAC_PI_2, ops::Deref, time::Duration};

use bevy::prelude::*;

use crate::game::{
    assets::sprites::entity::{EntityAssets, UtilSpriteVariant},
    entities::{
        enemy::{health::EnemyHealth, Enemy},
        tile::{movement::TileMovement, position::TilePosition, sprite::TileSprite},
    },
    GameState,
};

pub struct ProjectileVariantConfig {
    duration: Duration,
    sprite_scale: Vec3,
}

impl ProjectileVariantConfig {
    pub fn get_duration(&self) -> Duration {
        self.duration
    }
    pub fn get_sprite_scale(&self) -> Vec3 {
        self.sprite_scale
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ProjectileVariant {
    Bullet,
    Rocket,
}

#[derive(Component, Clone, Copy)]
#[require(TilePosition)]
pub struct Projectile {
    variant: ProjectileVariant,
    target: Entity,
    damage: u32,
}

impl ProjectileVariant {
    pub fn get_config(&self) -> ProjectileVariantConfig {
        match self {
            ProjectileVariant::Bullet => ProjectileVariantConfig {
                duration: Duration::from_secs_f32(0.1),
                sprite_scale: Vec3::new(0.5, 0.5, 1.0),
            },
            ProjectileVariant::Rocket => ProjectileVariantConfig {
                duration: Duration::from_secs_f32(0.2),
                sprite_scale: Vec3::new(0.75, 0.75, 1.0),
            },
        }
    }
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
    pub fn get_target(&self) -> Entity {
        self.target
    }
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
}

impl Deref for Projectile {
    type Target = ProjectileVariant;

    fn deref(&self) -> &Self::Target {
        &self.variant
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, init_projectile);

        app.add_systems(
            Update,
            update_projectile.run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_projectile(
    mut commands: Commands,
    projectiles: Query<(Entity, &Projectile), Added<Projectile>>,
    entity_assets: Option<Res<EntityAssets>>,
) {
    for (projectile_entity, projectile) in projectiles.iter() {
        commands.entity(projectile_entity).insert((
            TileSprite::new(projectile.get_variant().into()),
            Transform::from_scale(Vec3::ZERO),
        ));

        if let Some(entity_assets) = &entity_assets {
            commands.entity(projectile_entity).with_child(Sprite {
                image: entity_assets.tilemap.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: entity_assets.tilemap_layout.clone(),
                    index: UtilSpriteVariant::Glow as usize,
                }),
                color: Color::srgba(1.0, 1.0, 0.5, 1.0),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            });
        }
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
    mut enemies: Query<&mut EnemyHealth, With<Enemy>>,
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
            if let Ok(mut enemy_health) = enemies.get_mut(projectile.target) {
                enemy_health.damage(projectile.damage);
            }
            continue;
        }
        projectile_tile_position.set_from_vec2(projectile_movement.get_position());

        let direction = (projectile_movement.get_position()
            - projectile_movement.get_previous_position())
        .normalize_or_zero();

        projectile_transform.rotation =
            Quat::from_rotation_z(direction.x.atan2(direction.y) - FRAC_PI_2);
        projectile_transform.scale = projectile.get_config().get_sprite_scale();
    }
}
