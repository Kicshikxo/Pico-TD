use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

use crate::{
    assets::entities::tile::TileAssets,
    entities::{projectile::ProjectileVariant, structure::StructureVariant, unit::UnitVariant},
};

use super::TileVariant;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProjectileTileSpriteVariant {
    Bullet,
}
impl From<ProjectileVariant> for ProjectileTileSpriteVariant {
    fn from(variant: ProjectileVariant) -> Self {
        match variant {
            ProjectileVariant::Bullet => ProjectileTileSpriteVariant::Bullet,
        }
    }
}
impl ProjectileTileSpriteVariant {
    pub fn as_index(&self) -> usize {
        match self {
            ProjectileTileSpriteVariant::Bullet => 191,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StructureTileSpriteVariant {
    Soldier,
    Empty,
}
impl From<StructureVariant> for StructureTileSpriteVariant {
    fn from(variant: StructureVariant) -> Self {
        match variant {
            StructureVariant::Soldier => StructureTileSpriteVariant::Soldier,
            StructureVariant::Empty => StructureTileSpriteVariant::Empty,
        }
    }
}
impl StructureTileSpriteVariant {
    pub fn as_index(&self) -> usize {
        match self {
            StructureTileSpriteVariant::Soldier => 106,
            StructureTileSpriteVariant::Empty => 197,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TilemapTileSpriteVariant {
    Ground,
    Road,
    Water,
    Unknown,
}
impl From<TileVariant> for TilemapTileSpriteVariant {
    fn from(variant: TileVariant) -> Self {
        match variant {
            TileVariant::Ground => TilemapTileSpriteVariant::Ground,
            TileVariant::Road => TilemapTileSpriteVariant::Road,
            TileVariant::Water => TilemapTileSpriteVariant::Water,
            TileVariant::Unknown => TilemapTileSpriteVariant::Unknown,
        }
    }
}
impl TilemapTileSpriteVariant {
    pub fn as_index(&self) -> usize {
        match self {
            TilemapTileSpriteVariant::Ground => 0,
            TilemapTileSpriteVariant::Road => 108,
            TilemapTileSpriteVariant::Water => 37,
            TilemapTileSpriteVariant::Unknown => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UnitTileSpriteVariant {
    Soldier,
    Truck,
    Tank,
}
impl From<UnitVariant> for UnitTileSpriteVariant {
    fn from(variant: UnitVariant) -> Self {
        match variant {
            UnitVariant::Soldier => UnitTileSpriteVariant::Soldier,
            UnitVariant::Truck => UnitTileSpriteVariant::Truck,
            UnitVariant::Tank => UnitTileSpriteVariant::Tank,
        }
    }
}
impl UnitTileSpriteVariant {
    pub fn as_index(&self) -> usize {
        match self {
            UnitTileSpriteVariant::Soldier => 106,
            UnitTileSpriteVariant::Truck => 95,
            UnitTileSpriteVariant::Tank => 98,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileSpriteVariant {
    Projectile(ProjectileTileSpriteVariant),
    Structure(StructureTileSpriteVariant),
    Tilemap(TilemapTileSpriteVariant),
    Unit(UnitTileSpriteVariant),
}

#[derive(Component)]
#[require(Sprite)]
#[component(on_add = TileSprite::on_add)]
pub struct TileSprite {
    variant: TileSpriteVariant,
    need_update: bool,
}

#[allow(unused)]
impl TileSprite {
    pub fn new(variant: TileSpriteVariant) -> Self {
        Self {
            variant,
            need_update: false,
        }
    }
    pub fn on_add(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
        let tile_sprite = world.get::<Self>(entity).unwrap();
        let tile_assets = world.get_resource::<TileAssets>().unwrap();

        let image = match tile_sprite.variant {
            _ => tile_assets.forest_tilemap.clone(),
        };
        let layout = match tile_sprite.variant {
            _ => tile_assets.forest_tilemap_layout.clone(),
        };
        let index = tile_sprite.get_index();

        world.commands().entity(entity).insert(Sprite {
            image,
            texture_atlas: Some(TextureAtlas { layout, index }),
            ..default()
        });
    }
    pub fn get_index(&self) -> usize {
        match self.variant {
            TileSpriteVariant::Projectile(variant) => variant.as_index(),
            TileSpriteVariant::Structure(variant) => variant.as_index(),
            TileSpriteVariant::Tilemap(variant) => variant.as_index(),
            TileSpriteVariant::Unit(variant) => variant.as_index(),
        }
    }
    pub fn set_variant(&mut self, variant: TileSpriteVariant) {
        self.variant = variant;
        self.need_update = true;
    }
    pub fn get_need_update(&self) -> bool {
        self.need_update
    }
    pub fn set_need_update(&mut self, value: bool) {
        self.need_update = value;
    }
}
