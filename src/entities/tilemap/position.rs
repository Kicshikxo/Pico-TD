use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[require(Transform)]
pub struct TilePosition {
    x: f32,
    y: f32,
    need_update: bool,
}

impl Default for TilePosition {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            need_update: true,
        }
    }
}

#[allow(unused)]
impl TilePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, ..default() }
    }
    pub fn from_vec2(vec: Vec2) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            ..default()
        }
    }
    pub fn from_ivec2(ivec: IVec2) -> Self {
        Self {
            x: ivec.x as f32,
            y: ivec.y as f32,
            ..default()
        }
    }
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    pub fn as_ivec2(&self) -> IVec2 {
        IVec2::new(self.x as i32, self.y as i32)
    }
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.need_update = true;
    }
    pub fn set_from_vec2(&mut self, vec: Vec2) {
        self.x = vec.x;
        self.y = vec.y;
        self.need_update = true;
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        self.need_update = true;
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
        self.need_update = true;
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_need_update(&self) -> bool {
        self.need_update
    }
    pub fn set_need_update(&mut self, value: bool) {
        self.need_update = value;
    }
}
