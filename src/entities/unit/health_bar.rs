use bevy::prelude::*;

#[derive(Component, Clone)]
#[require(Sprite, Transform)]
pub struct UnitHealthBar {
    unit_entity: Entity,
}

impl UnitHealthBar {
    pub fn new(unit_entity: Entity) -> Self {
        Self { unit_entity }
    }
    pub fn get_unit_entity(&self) -> Entity {
        self.unit_entity
    }
}
