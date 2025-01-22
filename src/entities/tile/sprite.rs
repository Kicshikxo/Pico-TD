use bevy::prelude::*;

use crate::{
    assets::entities::tile::TilemapTileAssets,
    entities::{
        projectile::ProjectileVariant, structure::StructureVariant,
        tilemap::tile::TilemapTileVariant, unit::UnitVariant,
    },
    game::GameState,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileSpriteVariant {
    Projectile(ProjectileVariant),
    Structure(StructureVariant),
    Tilemap(TilemapTileVariant),
    Unit(UnitVariant),
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
                ProjectileVariant::Bullet => 120,
            },
            TileSpriteVariant::Structure(variant) => match variant {
                StructureVariant::Soldier => 8,
                StructureVariant::SoldierFast => 56,
                StructureVariant::SoldierStrong => 44,
            },
            TileSpriteVariant::Tilemap(variant) => match variant {
                TilemapTileVariant::Ground => 1,
                TilemapTileVariant::Flower => 3,
                TilemapTileVariant::Tree => 4,
                TilemapTileVariant::Road => 48,
                TilemapTileVariant::Water => 32,
                TilemapTileVariant::Unknown => 0,
            },
            TileSpriteVariant::Unit(variant) => match variant {
                UnitVariant::Truck => 60,
                UnitVariant::Plane => 65,
                UnitVariant::Tank => 63,
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
    tile_assets: Option<Res<TilemapTileAssets>>,
) {
    for (tile_sprite_entity, tile_sprite) in tile_sprites.iter() {
        let Some(tile_assets) = &tile_assets else {
            return;
        };
        let image = match tile_sprite.variant {
            TileSpriteVariant::Tilemap(_) => tile_assets.tilemap.clone(),
            _ => tile_assets.entities.clone(),
        };
        let layout = match tile_sprite.variant {
            TileSpriteVariant::Tilemap(_) => tile_assets.tilemap_layout.clone(),
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
