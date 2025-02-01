use bevy::prelude::*;

#[derive(Component, Clone)]
#[require(Sprite, Transform)]
pub struct EnemyHealthBar {
    enemy_entity: Entity,
}

impl EnemyHealthBar {
    pub fn new(enemy_entity: Entity) -> Self {
        Self { enemy_entity }
    }
    pub fn get_enemy_entity(&self) -> Entity {
        self.enemy_entity
    }
}
