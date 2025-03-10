use bevy::prelude::*;

#[derive(Resource, Default)]
pub enum GameSpeed {
    #[default]
    Normal,
    Double,
    Triple,
    Quadruple,
    Quintuple,
}

impl GameSpeed {
    pub fn as_index(&self) -> usize {
        match self {
            GameSpeed::Normal => 0,
            GameSpeed::Double => 1,
            GameSpeed::Triple => 2,
            GameSpeed::Quadruple => 3,
            GameSpeed::Quintuple => 4,
        }
    }
    pub fn as_f32(&self) -> f32 {
        match self {
            GameSpeed::Normal => 1.0,
            GameSpeed::Double => 2.0,
            GameSpeed::Triple => 3.0,
            GameSpeed::Quadruple => 4.0,
            GameSpeed::Quintuple => 5.0,
        }
    }
    pub fn from_f32(value: f32) -> GameSpeed {
        match value {
            1.0 => GameSpeed::Normal,
            2.0 => GameSpeed::Double,
            3.0 => GameSpeed::Triple,
            4.0 => GameSpeed::Quadruple,
            5.0 => GameSpeed::Quintuple,
            _ => GameSpeed::default(),
        }
    }
    pub fn set_default(&mut self) {
        *self = GameSpeed::default();
    }
    pub fn set(&mut self, speed: GameSpeed) {
        *self = speed;
    }
}
