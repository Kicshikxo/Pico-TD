use bevy::{prelude::*, sprite::AlphaMode2d};

use crate::{
    entities::{tile::position::TilePosition, tilemap::Tilemap},
    game::{GameState, GameTilemap},
    input::SelectedTile,
};

use super::Soldier;

#[derive(Component, Clone)]
#[require(Transform)]
pub struct FireRadius {
    visible: bool,
    soldier_entity: Entity,
}

impl FireRadius {
    pub fn new(enemy_entity: Entity) -> Self {
        Self {
            visible: false,
            soldier_entity: enemy_entity,
        }
    }
    pub fn get_soldier_entity(&self) -> Entity {
        self.soldier_entity
    }
    pub fn get_visible(&self) -> bool {
        self.visible
    }
    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }
}

pub struct FireRadiusPlugin;

impl Plugin for FireRadiusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_fire_radius, despawn_file_radius));
        app.add_systems(
            Update,
            update_fire_radius
                .run_if(in_state(GameState::InGame).and(resource_changed::<SelectedTile>)),
        );
        app.add_systems(
            Update,
            update_fire_radius_opacity.run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_fire_radius(
    mut commands: Commands,
    mut fire_radii: Query<Entity, Added<FireRadius>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for fire_radius_entity in fire_radii.iter_mut() {
        commands
            .entity(fire_radius_entity)
            .insert((
                Mesh2d(meshes.add(Annulus::new(0.0, 0.0))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: Color::srgb(1.0, 1.0, 0.0).with_alpha(0.0),
                    alpha_mode: AlphaMode2d::Blend,
                    ..default()
                })),
            ))
            .with_child((
                Mesh2d(meshes.add(Circle::new(0.0))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: Color::srgb(1.0, 1.0, 0.0).with_alpha(0.0),
                    alpha_mode: AlphaMode2d::Blend,
                    ..default()
                })),
            ));
    }
}

fn despawn_file_radius(
    mut commands: Commands,
    fire_radii: Query<(Entity, &FireRadius)>,
    mut removed_soldiers: RemovedComponents<Soldier>,
) {
    for removed_soldier_entity in removed_soldiers.read() {
        for (fire_radius_entity, fire_radius) in fire_radii.iter() {
            if fire_radius.get_soldier_entity() == removed_soldier_entity {
                commands.entity(fire_radius_entity).despawn_recursive();
            }
        }
    }
}

fn update_fire_radius(
    game_tilemap: Query<&Tilemap, With<GameTilemap>>,
    soldiers: Query<(&Soldier, &TilePosition, &Transform)>,
    mut fire_radii: Query<(&mut FireRadius, &Mesh2d, &mut Transform, &Children), Without<Soldier>>,
    inner_fire_radii: Query<&Mesh2d, Without<FireRadius>>,
    selected_tile: Res<SelectedTile>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut fire_radius, fire_radius_mesh_2d, mut fire_radius_transform, fire_radius_children) in
        fire_radii.iter_mut()
    {
        if let Ok((soldier, soldier_tile_position, soldier_transform)) =
            soldiers.get(fire_radius.get_soldier_entity())
        {
            if soldier_tile_position.as_vec2() == selected_tile.tile_position.as_vec2() {
                let inner_radius = soldier.get_fire_radius()
                    * game_tilemap.single().get_tile_size().max_element() as f32;

                if let Some(fire_radius_mesh) = meshes.get_mut(&fire_radius_mesh_2d.0) {
                    *fire_radius_mesh = Annulus::new(inner_radius, inner_radius + 2.0)
                        .mesh()
                        .build();
                }

                for inner_fire_radius_entity in fire_radius_children {
                    if let Ok(inner_fire_radius_mesh_2d) =
                        inner_fire_radii.get(*inner_fire_radius_entity)
                    {
                        if let Some(inner_fire_radius_mesh) =
                            meshes.get_mut(&inner_fire_radius_mesh_2d.0)
                        {
                            *inner_fire_radius_mesh = Circle::new(inner_radius).mesh().build();
                        }
                    }
                }

                fire_radius_transform.translation = soldier_transform.translation.with_z(-1.0);

                fire_radius.set_visible(true);
            } else {
                fire_radius.set_visible(false);
            }
        };
    }
}

fn update_fire_radius_opacity(
    fire_radii: Query<(&FireRadius, &MeshMaterial2d<ColorMaterial>, &Children)>,
    inner_fire_radii: Query<&MeshMaterial2d<ColorMaterial>, Without<FireRadius>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (fire_radius, fire_radius_mesh_material_2d, fire_radius_children) in fire_radii.iter() {
        if let Some(fire_radius_color_material) = materials.get_mut(&fire_radius_mesh_material_2d.0)
        {
            let target_alpha = if fire_radius.get_visible() { 0.5 } else { 0.0 };
            let current_alpha = fire_radius_color_material.color.alpha();

            if (current_alpha - target_alpha).abs() > f32::EPSILON {
                let new_alpha = current_alpha.lerp(target_alpha, time.delta_secs() * 20.0);
                fire_radius_color_material.color.set_alpha(new_alpha);
            }
        }
        for inner_fire_radius_entity in fire_radius_children {
            if let Ok(inner_fire_radius_mesh_material_2d) =
                inner_fire_radii.get(*inner_fire_radius_entity)
            {
                if let Some(inner_fire_radius_color_material) =
                    materials.get_mut(&inner_fire_radius_mesh_material_2d.0)
                {
                    let target_alpha = if fire_radius.get_visible() { 0.25 } else { 0.0 };
                    let current_alpha = inner_fire_radius_color_material.color.alpha();

                    if (current_alpha - target_alpha).abs() > f32::EPSILON {
                        let new_alpha = current_alpha.lerp(target_alpha, time.delta_secs() * 20.0);
                        inner_fire_radius_color_material.color.set_alpha(new_alpha);
                    }
                }
            }
        }
    }
}
