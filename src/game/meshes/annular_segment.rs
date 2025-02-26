use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

pub struct AnnularSegment;

impl AnnularSegment {
    pub fn new(inner_radius: f32, outer_radius: f32) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        Self::update(&mut mesh, inner_radius, outer_radius, 0.0, TAU);
        mesh
    }

    pub fn update_with_progress(
        mesh: &mut Mesh,
        inner_radius: f32,
        outer_radius: f32,
        progress: f32,
    ) {
        Self::update(mesh, inner_radius, outer_radius, FRAC_PI_2, progress * -TAU);
    }

    pub fn update(
        mesh: &mut Mesh,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        sweep_angle: f32,
    ) {
        let resolution = 16;
        let step = sweep_angle / resolution as f32;

        let vertices: Vec<Vec3> = (0..=resolution)
            .flat_map(|index| {
                let angle = start_angle + step * index as f32;
                let (sin, cos) = angle.sin_cos();
                [
                    Vec3::new(cos * outer_radius, sin * outer_radius, 0.0),
                    Vec3::new(cos * inner_radius, sin * inner_radius, 0.0),
                ]
            })
            .collect();

        let indices: Vec<u32> = (0..resolution)
            .flat_map(|index| {
                let index = index * 2;
                [index, index + 1, index + 2, index + 1, index + 3, index + 2]
            })
            .collect();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));
    }
}
