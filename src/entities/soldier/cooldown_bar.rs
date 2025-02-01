use bevy::prelude::*;

#[derive(Component, Clone)]
#[require(Sprite, Transform)]
pub struct CooldownBar {
    soldier_entity: Entity,
}

impl CooldownBar {
    pub fn new(enemy_entity: Entity) -> Self {
        Self {
            soldier_entity: enemy_entity,
        }
    }
    pub fn get_soldier_entity(&self) -> Entity {
        self.soldier_entity
    }
}
