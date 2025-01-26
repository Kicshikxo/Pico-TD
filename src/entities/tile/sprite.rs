use bevy::prelude::*;

use crate::{
    assets::sprites::tile::{
        EntityProjectileVariant, EntityStructureVariant, EntityUnitVariant, EntityUtilVariant,
        TileAssets, TilemapTileSpriteVariant,
    },
    entities::{
        projectile::ProjectileVariant, structure::StructureVariant,
        tilemap::tile::TilemapTileVariant, unit::UnitVariant,
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
    Structure(StructureVariant),
    Tilemap(TilemapTileVariant),
    Unit(UnitVariant),
    Util(UtilVariant),
}
impl From<ProjectileVariant> for TileSpriteVariant {
    fn from(variant: ProjectileVariant) -> Self {
        Self::Projectile(variant)
    }
}
impl From<StructureVariant> for TileSpriteVariant {
    fn from(variant: StructureVariant) -> Self {
        Self::Structure(variant)
    }
}
impl From<TilemapTileVariant> for TileSpriteVariant {
    fn from(variant: TilemapTileVariant) -> Self {
        Self::Tilemap(variant)
    }
}
impl From<UnitVariant> for TileSpriteVariant {
    fn from(variant: UnitVariant) -> Self {
        Self::Unit(variant)
    }
}
impl TileSpriteVariant {
    pub fn as_index(&self) -> usize {
        match self {
            TileSpriteVariant::Projectile(variant) => match variant {
                ProjectileVariant::Bullet => EntityProjectileVariant::Bullet as usize,
                ProjectileVariant::Rocket => EntityProjectileVariant::Rocket as usize,
            },
            TileSpriteVariant::Structure(variant) => match variant {
                StructureVariant::Soldier => EntityStructureVariant::Soldier as usize,
                StructureVariant::SoldierFast => EntityStructureVariant::SoldierFast as usize,
                StructureVariant::SoldierStrong => EntityStructureVariant::SoldierStrong as usize,
                StructureVariant::RocketLauncher => EntityStructureVariant::RocketLauncher as usize,
            },
            TileSpriteVariant::Tilemap(variant) => match variant {
                TilemapTileVariant::Ground => TilemapTileSpriteVariant::Ground as usize,
                TilemapTileVariant::Flower => TilemapTileSpriteVariant::GroundWithFlower as usize,
                TilemapTileVariant::Tree => TilemapTileSpriteVariant::GroundWithTree as usize,
                TilemapTileVariant::Road => TilemapTileSpriteVariant::Road as usize,
                TilemapTileVariant::Water => TilemapTileSpriteVariant::Water as usize,
                TilemapTileVariant::Unknown => TilemapTileSpriteVariant::Unknown as usize,
            },
            TileSpriteVariant::Unit(variant) => match variant {
                UnitVariant::Truck => EntityUnitVariant::Truck as usize,
                UnitVariant::Plane => EntityUnitVariant::Plane as usize,
                UnitVariant::Tank => EntityUnitVariant::Tank as usize,
                UnitVariant::Boat => EntityUnitVariant::Boat as usize,
                UnitVariant::Submarine => EntityUnitVariant::Submarine as usize,
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
