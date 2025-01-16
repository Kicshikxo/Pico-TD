use std::time::Duration;

use bevy::prelude::*;

use super::position::TilePosition;

#[derive(Component, Clone, Debug)]
#[require(TilePosition)]
pub struct TileMovement {
    path: Vec<Vec2>,
    path_segment_lengths: Vec<f32>,
    path_cumulative_lengths: Vec<f32>,
    position: Vec2,
    previous_position: Vec2,
    duration: Duration,
    progress: f32,
    elapsed_time: Duration,
}

impl Default for TileMovement {
    fn default() -> Self {
        Self {
            path: Vec::new(),
            path_segment_lengths: Vec::new(),
            path_cumulative_lengths: Vec::new(),
            position: Vec2::default(),
            previous_position: Vec2::default(),
            duration: Duration::from_secs(0),
            progress: 0.0,
            elapsed_time: Duration::from_secs(0),
        }
    }
}

impl TileMovement {
    pub fn new(path: Vec<Vec2>, duration: Duration) -> Self {
        let path_segment_lengths: Vec<f32> = std::iter::once(0.0)
            .chain(
                path.windows(2)
                    .map(|segment| (segment[1] - segment[0]).length()),
            )
            .collect();

        let path_cumulative_lengths = path_segment_lengths
            .iter()
            .scan(0.0, |sum, &length| {
                *sum += length;
                Some(*sum)
            })
            .collect();

        Self {
            path,
            path_segment_lengths,
            path_cumulative_lengths,
            duration,
            ..default()
        }
    }
    pub fn get_position(&self) -> Vec2 {
        self.position
    }
    pub fn get_previous_position(&self) -> Vec2 {
        self.previous_position
    }
    pub fn get_duration(&self) -> Duration {
        self.duration
    }
    pub fn get_progress(&self) -> f32 {
        self.progress
    }
    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
        self.elapsed_time = Duration::from_secs_f32(self.duration.as_secs_f32() * self.progress);
        self.update_current_position();
    }
    pub fn update_progress(&mut self, delta_time: f32) {
        self.elapsed_time += Duration::from_secs_f32(delta_time);
        self.progress =
            (self.elapsed_time.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        self.update_current_position();
    }
    pub fn update_current_position(&mut self) {
        if self.progress >= 1.0 {
            return;
        }

        self.previous_position = self.position.clone();
        self.position = self.position_at_progress(self.progress);
    }
    pub fn position_at_progress(&self, progress: f32) -> Vec2 {
        let target_distance =
            self.path_cumulative_lengths.last().unwrap_or(&0.0) * progress.clamp(0.0, 1.0);

        // let segment_index = self
        //     .path_cumulative_lengths
        //     .iter()
        //     .position(|&length| length >= target_distance)
        //     .unwrap_or(0);
        let segment_index = self
            .path_cumulative_lengths
            .binary_search_by(|&length| length.partial_cmp(&target_distance).unwrap())
            .unwrap_or_else(|index| index);

        if segment_index == 0 {
            return self.path[0];
        }

        let segment_start = self.path[segment_index - 1];
        let segment_end = self.path[segment_index];
        let start_distance = self.path_cumulative_lengths[segment_index - 1];
        let segment_length = self.path_segment_lengths[segment_index];

        segment_start
            + (segment_end - segment_start) * ((target_distance - start_distance) / segment_length)
    }
}
