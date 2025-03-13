use std::time::Duration;

use bevy::prelude::*;

use crate::game::{
    assets::images::entity::{EntityAssets, UtilSpriteVariant},
    entities::tilemap::Tilemap,
    speed::GameSpeed,
    GameState, GameTilemap,
};

#[derive(Component, Clone)]
#[require(Transform)]
pub struct ProjectileBlast {
    radius: f32,
    alpha: f32,
}

impl ProjectileBlast {
    pub fn new(radius: f32) -> Self {
        Self { radius, alpha: 1.0 }
    }
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
    pub fn get_alpha(&self) -> f32 {
        self.alpha
    }
    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }
}

pub struct ProjectileBlastPlugin;

impl Plugin for ProjectileBlastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, init_projectile_blast);

        app.add_systems(
            Update,
            update_projectile_blast.run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_projectile_blast(
    mut commands: Commands,
    game_tilemap: Query<&Tilemap, With<GameTilemap>>,
    mut projectile_blasts: Query<(Entity, &ProjectileBlast), Added<ProjectileBlast>>,
    entity_assets: Option<Res<EntityAssets>>,
) {
    for (projectile_blast_entity, projectile_blast) in projectile_blasts.iter_mut() {
        if let Some(entity_assets) = &entity_assets {
            commands.entity(projectile_blast_entity).insert(Sprite {
                image: entity_assets.tilemap.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: entity_assets.tilemap_layout.clone(),
                    index: UtilSpriteVariant::Glow as usize,
                }),
                color: Color::srgb(1.0, 1.0, 0.0),
                custom_size: Some(
                    Vec2::splat(projectile_blast.get_radius() * 2.0)
                        * game_tilemap.single().get_tile_size() as f32,
                ),
                ..default()
            });
        }
    }
}

fn update_projectile_blast(
    mut commands: Commands,
    mut projectile_blasts: Query<(Entity, &mut ProjectileBlast, &mut Sprite)>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    for (projectile_blast_entity, mut projectile_blast, mut projectile_blast_sprite) in
        projectile_blasts.iter_mut()
    {
        if projectile_blast.get_alpha() < 1e-3 {
            commands.entity(projectile_blast_entity).despawn_recursive();
            continue;
        }

        let current_alpha = projectile_blast.get_alpha();

        projectile_blast_sprite.color.set_alpha(current_alpha);

        projectile_blast.set_alpha(current_alpha.lerp(
            0.0,
            time.delta_secs() * game_speed.as_f32() / Duration::from_millis(100).as_secs_f32(),
        ));
    }
}
