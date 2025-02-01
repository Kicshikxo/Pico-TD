use bevy::prelude::*;

use crate::{
    assets::sprites::tile::{
        EntityEnemyVariant, EntityProjectileVariant, EntitySoldierVariant, EntityUtilVariant,
        TileAssets, TilemapTileSpriteVariant,
    },
    entities::{
        enemy::EnemyVariant,
        soldier::{projectile::ProjectileVariant, SoldierVariant},
        tilemap::tile::TilemapTileVariant,
    },
    game::GameState,
};

#[derive(Clone, Copy, PartialEq)]
pub enum UtilVariant {
    TileIndicator,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileSpriteVariant {
    Projectile(ProjectileVariant),
    Soldier(SoldierVariant),
    Tilemap(TilemapTileVariant),
    Enemy(EnemyVariant),
    Util(UtilVariant),
}
impl From<ProjectileVariant> for TileSpriteVariant {
    fn from(variant: ProjectileVariant) -> Self {
        Self::Projectile(variant)
    }
}
impl From<SoldierVariant> for TileSpriteVariant {
    fn from(variant: SoldierVariant) -> Self {
        Self::Soldier(variant)
    }
}
impl From<TilemapTileVariant> for TileSpriteVariant {
    fn from(variant: TilemapTileVariant) -> Self {
        Self::Tilemap(variant)
    }
}
impl From<EnemyVariant> for TileSpriteVariant {
    fn from(variant: EnemyVariant) -> Self {
        Self::Enemy(variant)
    }
}
impl TileSpriteVariant {
    pub fn as_index(&self) -> usize {
        match self {
            TileSpriteVariant::Projectile(variant) => match variant {
                ProjectileVariant::Bullet => EntityProjectileVariant::Bullet as usize,
                ProjectileVariant::Rocket => EntityProjectileVariant::Rocket as usize,
            },
            TileSpriteVariant::Soldier(variant) => match variant {
                SoldierVariant::Soldier => EntitySoldierVariant::Soldier as usize,
                SoldierVariant::SoldierFast => EntitySoldierVariant::SoldierFast as usize,
                SoldierVariant::SoldierStrong => EntitySoldierVariant::SoldierStrong as usize,
                SoldierVariant::SoldierSniper => EntitySoldierVariant::SoldierSniper as usize,
                SoldierVariant::RocketLauncher => EntitySoldierVariant::RocketLauncher as usize,
            },
            TileSpriteVariant::Tilemap(variant) => match variant {
                TilemapTileVariant::Ground => TilemapTileSpriteVariant::Ground as usize,
                TilemapTileVariant::Flower => TilemapTileSpriteVariant::GroundWithFlower as usize,
                TilemapTileVariant::Tree => TilemapTileSpriteVariant::GroundWithTree as usize,
                TilemapTileVariant::Road => TilemapTileSpriteVariant::Road as usize,
                TilemapTileVariant::Water => TilemapTileSpriteVariant::Water as usize,
                TilemapTileVariant::Unknown => TilemapTileSpriteVariant::Unknown as usize,
            },
            TileSpriteVariant::Enemy(variant) => match variant {
                EnemyVariant::Dron => EntityEnemyVariant::Dron as usize,
                EnemyVariant::Truck => EntityEnemyVariant::Truck as usize,
                EnemyVariant::Tank => EntityEnemyVariant::Tank as usize,
                EnemyVariant::Plane => EntityEnemyVariant::Plane as usize,
                EnemyVariant::Helicopter => EntityEnemyVariant::Helicopter as usize,
                EnemyVariant::Boat => EntityEnemyVariant::Boat as usize,
                EnemyVariant::Submarine => EntityEnemyVariant::Submarine as usize,
            },
            TileSpriteVariant::Util(variant) => match variant {
                UtilVariant::TileIndicator => EntityUtilVariant::TileIndicator as usize,
            },
        }
    }
}

#[derive(Component)]
#[require(Sprite)]
pub struct TileSprite {
    variant: TileSpriteVariant,
    update_required: bool,
}

impl TileSprite {
    pub fn new(variant: TileSpriteVariant) -> Self {
        Self {
            variant,
            update_required: false,
        }
    }
    pub fn get_variant(&self) -> TileSpriteVariant {
        self.variant
    }
    pub fn set_variant(&mut self, variant: TileSpriteVariant) {
        self.variant = variant;
        self.update_required = true;
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
}

pub struct TileSpritePlugin;

impl Plugin for TileSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, init_tile_sprite);
        app.add_systems(
            Update,
            update_tile_sprite.run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_tile_sprite(
    mut commands: Commands,
    tile_sprites: Query<(Entity, &TileSprite), Added<TileSprite>>,
    tile_assets: Option<Res<TileAssets>>,
) {
    for (tile_sprite_entity, tile_sprite) in tile_sprites.iter() {
        let Some(tile_assets) = &tile_assets else {
            return;
        };
        let image = match tile_sprite.variant {
            TileSpriteVariant::Tilemap(_) => tile_assets.tilemap_tiles.clone(),
            _ => tile_assets.entities.clone(),
        };
        let layout = match tile_sprite.variant {
            TileSpriteVariant::Tilemap(_) => tile_assets.tilemap_tiles_layout.clone(),
            _ => tile_assets.entities_layout.clone(),
        };
        let index = tile_sprite.get_variant().as_index();

        commands.entity(tile_sprite_entity).insert(Sprite {
            image,
            texture_atlas: Some(TextureAtlas { layout, index }),
            ..default()
        });
    }
}

fn update_tile_sprite(mut tile_sprites: Query<(&mut TileSprite, &mut Sprite)>) {
    for (mut tile_sprite, mut sprite) in tile_sprites.iter_mut() {
        if tile_sprite.get_update_required() == false {
            continue;
        }

        if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
            texture_atlas.index = tile_sprite.get_variant().as_index();
        }

        tile_sprite.set_update_required(false);
    }
}
