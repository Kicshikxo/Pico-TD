use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

pub struct RoundedRectangle;

impl RoundedRectangle {
    pub fn new(width: f32, height: f32, corner_radius: f32) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        Self::update(&mut mesh, width, height, corner_radius);
        mesh
    }

    pub fn update(mesh: &mut Mesh, width: f32, height: f32, corner_radius: f32) {
        let resolution = 16;
        let segments_per_corner = resolution / 4;

        let corner_radius = corner_radius.min(width / 2.0).min(height / 2.0);

        let corners = [
            (
                width / 2.0 - corner_radius,
                height / 2.0 - corner_radius,
                0.0,
                FRAC_PI_2,
            ),
            (
                -width / 2.0 + corner_radius,
                height / 2.0 - corner_radius,
                FRAC_PI_2,
                PI,
            ),
            (
                -width / 2.0 + corner_radius,
                -height / 2.0 + corner_radius,
                PI,
                3.0 * FRAC_PI_2,
            ),
            (
                width / 2.0 - corner_radius,
                -height / 2.0 + corner_radius,
                3.0 * FRAC_PI_2,
                2.0 * PI,
            ),
        ];

        let vertices: Vec<Vec3> = corners
            .iter()
            .flat_map(|&(corner_x, corner_y, corner_start, corner_end)| {
                let step = (corner_end - corner_start) / segments_per_corner as f32;
                (0..=segments_per_corner).map(move |index| {
                    let angle = corner_start + step * index as f32;
                    Vec3::new(
                        corner_x + angle.cos() * corner_radius,
                        corner_y + angle.sin() * corner_radius,
                        0.0,
                    )
                })
            })
            .chain(std::iter::once(Vec3::ZERO))
            .collect();

        let center_index = vertices.len() as u32 - 1;
        let indices: Vec<u32> = (0..center_index)
            .flat_map(|index| {
                let next_index = (index + 1) % center_index;
                [index, next_index, center_index]
            })
            .collect();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));
    }
}
