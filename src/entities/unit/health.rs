use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
#[require(Transform)]
pub struct UnitHealth {
    max: u32,
    current: u32,
    damage_indicator: bool,
}

impl Default for UnitHealth {
    fn default() -> Self {
        Self {
            max: 0,
            current: 0,
            damage_indicator: false,
        }
    }
}

#[allow(unused)]
impl UnitHealth {
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
        self.current = self.current.saturating_add(heal).max(self.max);
    }
    pub fn get_current(&self) -> u32 {
        self.current
    }
    pub fn get_max(&self) -> u32 {
        self.max
    }
}

#[derive(Component, Clone, Debug)]
#[require(Sprite, Transform)]
pub struct UnitHealthBar;
