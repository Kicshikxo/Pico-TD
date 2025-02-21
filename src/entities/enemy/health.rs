use bevy::prelude::*;

#[derive(Component, Clone)]
#[require(Transform)]
pub struct EnemyHealth {
    max: u32,
    current: u32,
    damage_indicator: bool,
}

impl Default for EnemyHealth {
    fn default() -> Self {
        Self {
            max: 0,
            current: 0,
            damage_indicator: false,
        }
    }
}

impl EnemyHealth {
    pub fn new(max: u32) -> Self {
        Self {
            max,
            current: max,
            ..default()
        }
    }
    pub fn damage(&mut self, damage: u32) {
        self.current = self.current.saturating_sub(damage);
        self.damage_indicator = true;
    }
    pub fn get_damage_indicator(&self) -> bool {
        self.damage_indicator
    }
    pub fn clear_damage_indicator(&mut self) {
        self.damage_indicator = false;
    }
    pub fn heal(&mut self, heal: u32) {
        self.current = self.current.saturating_add(heal).min(self.max);
    }
    pub fn get_current(&self) -> u32 {
        self.current
    }
    pub fn get_max(&self) -> u32 {
        self.max
    }
    pub fn get_percentage(&self) -> f32 {
        (self.get_current() as f32 / self.get_max() as f32).clamp(0.0, 1.0)
    }
    pub fn set_max(&mut self, max: u32) {
        self.max = max;
    }
}
