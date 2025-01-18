use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

use crate::{
    assets::entities::tile::TilemapTileAssets,
    entities::{
        projectile::ProjectileVariant, structure::StructureVariant,
        tilemap::tile::TilemapTileVariant, unit::UnitVariant,
    },
    game::GameState,
};

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
    SoldierFast,
    SoldierStrong,
    Empty,
}
impl From<StructureVariant> for StructureTileSpriteVariant {
    fn from(variant: StructureVariant) -> Self {
        match variant {
            StructureVariant::Soldier => StructureTileSpriteVariant::Soldier,
            StructureVariant::SoldierFast => StructureTileSpriteVariant::SoldierFast,
            StructureVariant::SoldierStrong => StructureTileSpriteVariant::SoldierStrong,
            StructureVariant::Empty => StructureTileSpriteVariant::Empty,
        }
    }
}
impl StructureTileSpriteVariant {
    pub fn as_index(&self) -> usize {
        match self {
            StructureTileSpriteVariant::Soldier => 106,
            StructureTileSpriteVariant::SoldierFast => 178,
            StructureTileSpriteVariant::SoldierStrong => 160,
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
impl From<TilemapTileVariant> for TilemapTileSpriteVariant {
    fn from(variant: TilemapTileVariant) -> Self {
        match variant {
            TilemapTileVariant::Ground => TilemapTileSpriteVariant::Ground,
            TilemapTileVariant::Road => TilemapTileSpriteVariant::Road,
            TilemapTileVariant::Water => TilemapTileSpriteVariant::Water,
            TilemapTileVariant::Unknown => TilemapTileSpriteVariant::Unknown,
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
    update_required: bool,
}

#[allow(unused)]
impl TileSprite {
    pub fn new(variant: TileSpriteVariant) -> Self {
        Self {
            variant,
            update_required: false,
        }
    }
    pub fn on_add(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
        let tile_sprite = world.get::<Self>(entity).unwrap();
        let tile_assets = world.get_resource::<TilemapTileAssets>().unwrap();

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
    // !
    // pub fn get_image(&self) -> Handle<Image> {

    // }
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
        app.add_systems(
            Update,
            update_tile_sprite.run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_tile_sprite(mut tile_sprites: Query<(&mut TileSprite, &mut Sprite)>) {
    for (mut tile_sprite, mut sprite) in tile_sprites.iter_mut() {
        if tile_sprite.get_update_required() == false {
            continue;
        }

        if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
            texture_atlas.index = tile_sprite.get_index();
        }

        tile_sprite.set_update_required(false);
    }
}
