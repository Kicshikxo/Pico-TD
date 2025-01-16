use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::entities::tilemap::tile::TilePosition;

#[derive(Component, Clone, Debug)]
#[require(TilePosition)]
pub struct ProjectileMovement {
    start_position: Vec2,
    end_position: Vec2,
    transform: Transform,
    duration: Duration,
    progress: f32,
    elapsed_time: Duration,
}

impl Default for ProjectileMovement {
    fn default() -> Self {
        Self {
            start_position: Vec2::default(),
            end_position: Vec2::default(),
            transform: Transform::default(),
            duration: Duration::from_secs(0),
            progress: 0.0,
            elapsed_time: Duration::from_secs(0),
        }
    }
}

impl ProjectileMovement {
    pub fn new(start_position: Vec2, end_position: Vec2, duration: Duration) -> Self {
        Self {
            start_position,
            end_position,
            duration,
            ..default()
        }
    }
    pub fn get_transform(&self) -> Transform {
        self.transform
    }
    pub fn get_progress(&self) -> f32 {
        self.progress
    }
    pub fn update_progress(&mut self, delta_time: f32) {
        self.elapsed_time += Duration::from_secs_f32(delta_time);
        self.progress =
            (self.elapsed_time.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        self.update_current_position(delta_time);
    }
    pub fn update_current_position(&mut self, delta_time: f32) {
        if self.progress >= 1.0 {
            return;
        }

        let previous_transform = self.transform.clone();
        self.transform.translation = self
            .position_at_progress(self.progress)
            .extend(self.transform.translation.z);

        let direction =
            (self.transform.translation - previous_transform.translation).normalize_or_zero();

        self.transform.rotation = Quat::from_rotation_z(direction.x.atan2(direction.y) - PI / 2.0);
    }
    pub fn position_at_progress(&self, progress: f32) -> Vec2 {
        self.start_position + (self.end_position - self.start_position) * progress
    }
}
