use bevy::prelude::*;

use crate::{game::GameState, utils::meshes::RoundedRectangle};

use super::{health::EnemyHealth, Enemy};

#[derive(Component, Clone)]
#[require(Transform)]
pub struct HealthBar {
    enemy_entity: Entity,
    update_required: bool,
}

impl HealthBar {
    pub fn new(enemy_entity: Entity) -> Self {
        Self {
            enemy_entity,
            update_required: false,
        }
    }
    pub fn get_enemy_entity(&self) -> Entity {
        self.enemy_entity
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_enemy_health_bar, despawn_health_bar));
        app.add_systems(
            Update,
            update_enemy_health_bar.run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_enemy_health_bar(
    mut commands: Commands,
    mut health_bars: Query<Entity, Added<HealthBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for health_bars_entity in health_bars.iter_mut() {
        commands.entity(health_bars_entity).insert((
            Mesh2d(meshes.add(RoundedRectangle::new(0.0, 0.0, 0.0))),
            MeshMaterial2d(materials.add(Color::default())),
        ));
    }
}

fn despawn_health_bar(
    mut commands: Commands,
    health_bars: Query<(Entity, &HealthBar)>,
    mut removed_enemies: RemovedComponents<Enemy>,
) {
    for removed_soldier_entity in removed_enemies.read() {
        for (health_bar_entity, health_bar) in health_bars.iter() {
            if health_bar.get_enemy_entity() == removed_soldier_entity {
                commands.entity(health_bar_entity).despawn_recursive();
            }
        }
    }
}

fn update_enemy_health_bar(
    enemies: Query<(&EnemyHealth, &Transform), Without<HealthBar>>,
    mut health_bars: Query<
        (
            &mut HealthBar,
            &Mesh2d,
            &MeshMaterial2d<ColorMaterial>,
            &mut Transform,
        ),
        Without<Enemy>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (
        mut health_bar,
        health_bar_mesh_2d,
        health_bar_mesh_material_2d,
        mut health_bar_transform,
    ) in health_bars.iter_mut()
    {
        if let Ok((enemy_health, enemy_transform)) = enemies.get(health_bar.get_enemy_entity()) {
            let health_percentage = enemy_health.get_percentage();

            if health_bar.get_update_required() == true {
                if let Some(health_bar_mesh) = meshes.get_mut(&health_bar_mesh_2d.0) {
                    RoundedRectangle::update(health_bar_mesh, 12.0 * health_percentage, 2.0, 1.0)
                }
                if let Some(health_bar_color_material) =
                    materials.get_mut(&health_bar_mesh_material_2d.0)
                {
                    let health_color_intensity = health_percentage * 2.0;
                    health_bar_color_material.color = Color::srgb(
                        2.0 - health_color_intensity.max(1.0),
                        health_color_intensity.min(1.0),
                        0.0,
                    );
                }
                health_bar.set_update_required(false);
            }

            health_bar_transform.translation =
                enemy_transform.translation + Vec3::new(6.0 * -(1.0 - health_percentage), 8.0, 1.0);
        }
    }
}
