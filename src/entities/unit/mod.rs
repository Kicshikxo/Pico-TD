pub mod health;

use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Anchor};
use health::{UnitHealth, UnitHealthBar};

use crate::game::GameState;

use super::tile::{movement::TileMovement, position::TilePosition};

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(unused)]
pub enum UnitVariant {
    Soldier,
    Truck,
    Tank,
}

#[derive(Component, Clone)]
#[require(UnitHealth, TileMovement, TilePosition)]
pub struct Unit {
    variant: UnitVariant,
}

impl Unit {
    pub fn new(variant: UnitVariant) -> Self {
        Self { variant }
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_unit_movement, update_unit_health).run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_unit_movement(
    time: Res<Time>,
    mut units: Query<(&mut TileMovement, &mut TilePosition, &mut Transform), With<Unit>>,
) {
    for (mut unit_movement, mut unit_tile_position, mut unit_transform) in units.iter_mut() {
        unit_movement.update_progress(time.delta_secs());
        unit_tile_position.set_from_vec2(unit_movement.get_position());

        let (current_z, _current_y, _current_x) = unit_transform.rotation.to_euler(EulerRot::ZYX);
        let direction = (unit_movement.get_position() - unit_movement.get_previous_position())
            .normalize_or_zero();

        let rotation_z = current_z.lerp(
            direction.x.atan2(direction.y) - PI / 2.0,
            time.delta_secs() * 10.0,
        );
        unit_transform.rotation = Quat::from_rotation_z(rotation_z);
        unit_transform.scale = Vec3::new(0.75, 0.75, 1.0);
    }
}

fn update_unit_health(
    mut commands: Commands,
    mut units: Query<(Entity, &mut UnitHealth, &mut Sprite, Option<&Children>), With<Unit>>,
    mut health_bars: Query<(&mut Sprite, &mut Transform), (With<UnitHealthBar>, Without<Unit>)>,
    time: Res<Time>,
) {
    for (unit_entity, mut unit_health, mut unit_sprite, unit_children) in units.iter_mut() {
        if unit_health.get_current() == 0 {
            commands.entity(unit_entity).despawn_recursive();
            return;
        }

        unit_sprite.color = LinearRgba::from_vec3(
            unit_sprite
                .color
                .to_linear()
                .to_vec3()
                .lerp(LinearRgba::WHITE.to_vec3(), time.delta_secs() * 4.0),
        )
        .into();

        if unit_health.get_damage_indicator() {
            unit_sprite.color = Color::srgb(1.0, 0.0, 0.0);
            unit_health.clear_damage_indicator();
        }

        if let Some(children) = unit_children {
            for &child in children.iter() {
                let Ok((mut health_bar_sprite, mut health_bar_transform)) =
                    health_bars.get_mut(child)
                else {
                    continue;
                };

                let health_percentage =
                    unit_health.get_current() as f32 / unit_health.get_max() as f32;

                health_bar_sprite.color = match health_percentage {
                    health_percentage if health_percentage < 0.3 => Color::srgb(1.0, 0.0, 0.0),
                    health_percentage if health_percentage < 0.7 => Color::srgb(1.0, 1.0, 0.0),
                    _ => Color::srgb(0.0, 1.0, 0.0),
                };

                health_bar_sprite.anchor = Anchor::TopLeft;
                health_bar_sprite.custom_size = Some(Vec2::new(16.0, 2.0));
                health_bar_transform.scale = Vec3::new(health_percentage, 1.0, 1.0);
                health_bar_transform.translation = Vec3::new(-8.0, 8.0, 1.0);
            }
        }
    }
}
