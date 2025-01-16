use bevy::prelude::*;

use crate::game::GameState;

#[derive(Component, Clone)]
pub struct EntityShadow;

impl EntityShadow {
    pub fn new() -> Self {
        Self {}
    }
    pub fn new_with_movement(movement: UnitMovement) -> (Self, UnitMovement, Transform) {
        (Self {}, movement, Transform::default())
    }
}

pub struct EntityShadowPlugin;

impl Plugin for EntityShadowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (shadow_movement.run_if(in_state(GameState::InGame)),),
        );
    }
}

fn shadow_movement(time: Res<Time>, mut shadows: Query<(&mut EntityShadow, &mut Transform)>) {
    for (mut shadow, mut shadow_transform) in units.iter_mut() {}
}
