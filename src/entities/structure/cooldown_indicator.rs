use bevy::prelude::*;

#[derive(Component, Clone)]
#[require(Sprite, Transform)]
pub struct CooldownIndicator {
    structure_entity: Entity,
}

impl CooldownIndicator {
    pub fn new(unit_entity: Entity) -> Self {
        Self {
            structure_entity: unit_entity,
        }
    }
    pub fn get_structure_entity(&self) -> Entity {
        self.structure_entity
    }
}
