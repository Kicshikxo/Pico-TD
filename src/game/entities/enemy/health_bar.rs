use bevy::prelude::*;

use crate::game::{
    GameState,
    entities::enemy::{Enemy, health::EnemyHealth},
    meshes::rounded_rectangle::RoundedRectangle,
};

#[derive(Component, Clone)]
#[require(Transform)]
pub struct EnemyHealthBar {
    enemy_entity: Entity,
    update_required: bool,
}

impl EnemyHealthBar {
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

pub struct EnemyHealthBarPlugin;

impl Plugin for EnemyHealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, init_enemy_health_bar);
        app.add_systems(PostUpdate, despawn_enemy_health_bar);

        app.add_systems(
            Update,
            update_enemy_health_bar.run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_enemy_health_bar(
    mut commands: Commands,
    mut enemy_health_bars: Query<Entity, Added<EnemyHealthBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for enemy_health_bar_entity in enemy_health_bars.iter_mut() {
        commands.entity(enemy_health_bar_entity).insert((
            Mesh2d(meshes.add(RoundedRectangle::new(0.0, 0.0, 0.0))),
            MeshMaterial2d(materials.add(Color::default())),
        ));
    }
}

fn despawn_enemy_health_bar(
    mut commands: Commands,
    enemy_health_bars: Query<(Entity, &EnemyHealthBar)>,
    mut removed_enemies: RemovedComponents<Enemy>,
) {
    for removed_enemy_entity in removed_enemies.read() {
        for (enemy_health_bar_entity, enemy_health_bar) in enemy_health_bars.iter() {
            if enemy_health_bar.get_enemy_entity() == removed_enemy_entity {
                commands.entity(enemy_health_bar_entity).despawn();
            }
        }
    }
}

fn update_enemy_health_bar(
    enemies: Query<(&EnemyHealth, &Transform), Without<EnemyHealthBar>>,
    mut enemy_health_bars: Query<
        (
            &mut EnemyHealthBar,
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
        mut enemy_health_bar,
        enemy_health_bar_mesh_2d,
        enemy_health_bar_mesh_material_2d,
        mut enemy_health_bar_transform,
    ) in enemy_health_bars.iter_mut()
    {
        if let Ok((enemy_health, enemy_transform)) =
            enemies.get(enemy_health_bar.get_enemy_entity())
        {
            let health_percentage = enemy_health.get_percentage();

            if enemy_health_bar.get_update_required() == true {
                if let Some(enemy_health_bar_mesh) = meshes.get_mut(&enemy_health_bar_mesh_2d.0) {
                    RoundedRectangle::update(
                        enemy_health_bar_mesh,
                        12.0 * health_percentage,
                        2.0,
                        1.0,
                    )
                }
                if let Some(enemy_health_bar_color_material) =
                    materials.get_mut(&enemy_health_bar_mesh_material_2d.0)
                {
                    let health_color_intensity = health_percentage * 2.0;
                    enemy_health_bar_color_material.color = Color::srgb(
                        2.0 - health_color_intensity.max(1.0),
                        health_color_intensity.min(1.0),
                        0.0,
                    );
                }
                enemy_health_bar.set_update_required(false);
            }

            enemy_health_bar_transform.translation =
                enemy_transform.translation + Vec3::new(6.0 * -(1.0 - health_percentage), 8.0, 1.0);
        }
    }
}
