use bevy::prelude::*;

use crate::{game::GameState, utils::meshes::ArcMesh};

use super::Soldier;

#[derive(Component, Clone)]
#[require(Transform)]
pub struct CooldownIndicator {
    soldier_entity: Entity,
    update_required: bool,
}

impl CooldownIndicator {
    pub fn new(enemy_entity: Entity) -> Self {
        Self {
            soldier_entity: enemy_entity,
            update_required: false,
        }
    }
    pub fn get_soldier_entity(&self) -> Entity {
        self.soldier_entity
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
}

pub struct CooldownIndicatorPlugin;

impl Plugin for CooldownIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_cooldown_indicator);
        app.add_systems(
            Update,
            update_cooldown_indicator.run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_cooldown_indicator(
    mut commands: Commands,
    mut cooldown_indicators: Query<Entity, Added<CooldownIndicator>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for cooldown_indicator_entity in cooldown_indicators.iter_mut() {
        commands.entity(cooldown_indicator_entity).insert((
            Mesh2d(meshes.add(ArcMesh::new(1.0, 2.0))),
            MeshMaterial2d(materials.add(Color::default())),
        ));
    }
}

fn update_cooldown_indicator(
    mut commands: Commands,
    soldiers: Query<(&Soldier, &Transform)>,
    mut cooldown_indicators: Query<
        (
            Entity,
            &mut CooldownIndicator,
            &Mesh2d,
            &MeshMaterial2d<ColorMaterial>,
            &mut Transform,
        ),
        Without<Soldier>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (
        cooldown_indicator_entity,
        mut cooldown_indicator,
        cooldown_indicator_mesh_2d,
        cooldown_indicator_mesh_material_2d,
        mut cooldown_indicator_transform,
    ) in cooldown_indicators.iter_mut()
    {
        if cooldown_indicator.get_update_required() == false {
            continue;
        }

        let Ok((soldier, soldier_transform)) =
            soldiers.get(cooldown_indicator.get_soldier_entity())
        else {
            commands
                .entity(cooldown_indicator_entity)
                .despawn_recursive();
            continue;
        };

        let cooldown_percentage = soldier.get_cooldown_percentage();

        if let Some(cooldown_indicator_mesh) = meshes.get_mut(&cooldown_indicator_mesh_2d.0) {
            ArcMesh::update_with_progress(cooldown_indicator_mesh, 1.0, 2.0, cooldown_percentage);
        }
        if let Some(cooldown_indicator_color_material) =
            materials.get_mut(&cooldown_indicator_mesh_material_2d.0)
        {
            cooldown_indicator_color_material.color =
                Color::srgb(0.5 + cooldown_percentage / 2.0, 1.0, 0.0);
        }

        cooldown_indicator_transform.translation =
            soldier_transform.translation + Vec3::new(8.0 - 2.0, 8.0 - 2.0, 1.0);

        cooldown_indicator.set_update_required(false);
    }
}
