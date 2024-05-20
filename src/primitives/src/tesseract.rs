use bevy::math::{Vec3};
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use nalgebra::{Matrix4};
use crate::{Rotation, Vertex};


pub struct Tesseract {
    vertices: Vec<Vertex>,
}

impl Tesseract {
    pub fn new() -> Self {
        let vertices = vec![
            Vertex::new(0.0, 0.0, 0.0, 0.0),
            Vertex::new(1.0, 0.0, 0.0, 0.0),
            Vertex::new(1.0, 0.0, 1.0, 0.0),
            Vertex::new(0.0, 0.0, 1.0, 0.0),
            Vertex::new(0.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, 1.0, 0.0),
            Vertex::new(0.0, 1.0, 1.0, 0.0),
            Vertex::new(0.0, 0.0, 0.0, 1.0),
            Vertex::new(1.0, 0.0, 0.0, 1.0),
            Vertex::new(1.0, 0.0, 1.0, 1.0),
            Vertex::new(0.0, 0.0, 1.0, 1.0),
            Vertex::new(0.0, 1.0, 0.0, 1.0),
            Vertex::new(1.0, 1.0, 0.0, 1.0),
            Vertex::new(1.0, 1.0, 1.0, 1.0),
            Vertex::new(0.0, 1.0, 1.0, 1.0),
        ];
        Self { vertices }
    }

    pub fn projected_vertices(&self) -> Vec<Vec3> {
        let mut projected = vec![];
        for vertex in &self.vertices {
            projected.push(Vec3::new(
                vertex.position.x,
                vertex.position.y,
                vertex.position.z,
            ));
        }
        projected
    }

    pub fn mesh(&self) -> Mesh {
        let indices = vec![
            6, 2, 1, 5, 6, 1, 8, 4, 7, 7, 4, 3, 7, 6, 5, 8, 7, 5, 3, 1, 2, 1, 3, 4, 3, 2, 6, 7, 3,
            6, 8, 5, 4, 1, 4, 5, 5, 1, 13, 9, 13, 1, 8, 16, 4, 12, 4, 16, 8, 4, 5, 1, 5, 4, 16, 13,
            12, 9, 12, 13, 8, 5, 13, 16, 8, 13, 4, 9, 1, 12, 9, 4, 13, 10, 9, 14, 10, 13, 16, 12,
            11, 15, 16, 11, 16, 13, 12, 12, 13, 9, 11, 10, 14, 15, 11, 14, 12, 9, 10, 11, 12, 10,
            16, 14, 13, 15, 14, 16, 14, 10, 2, 6, 14, 2, 11, 15, 3, 7, 3, 15, 11, 3, 10, 3, 2, 10,
            15, 14, 7, 7, 14, 6, 15, 11, 14, 10, 14, 11, 7, 6, 3, 2, 3, 6, 14, 6, 13, 13, 6, 5, 15,
            16, 7, 8, 7, 16, 15, 7, 14, 7, 6, 14, 8, 13, 5, 16, 13, 8, 7, 5, 6, 8, 5, 7, 15, 14,
            13, 16, 15, 13, 10, 2, 9, 1, 9, 2, 11, 12, 3, 4, 3, 12, 11, 3, 10, 2, 10, 3, 12, 9, 4,
            4, 9, 1, 12, 10, 9, 11, 10, 12, 3, 1, 2, 4, 1, 3,
        ];
        let mut normal_indices = vec![];
        for i in indices {
            normal_indices.push(i - 1);
        }
        let projected_vertices: Vec<Vec3> = self.projected_vertices();
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, projected_vertices)
        .with_inserted_indices(Indices::U32(normal_indices))
            .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
        ],
    )
    }

    pub fn translate(&mut self, dw: f32) {
        for v in &mut self.vertices {
            v.translate(dw);
        }
    }

    pub fn rotate(&mut self, plane: Rotation, theta: f32) {
        let rotation_matrix = match plane {
            Rotation::XW => Matrix4::new(
                theta.cos(),
                0.0,
                0.0,
                theta.sin(),
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                -theta.sin(),
                0.0,
                0.0,
                theta.cos(),
            ),
            Rotation::YW => Matrix4::new(
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                theta.cos(),
                0.0,
                -theta.sin(),
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                theta.sin(),
                0.0,
                theta.cos(),
            ),
            Rotation::ZW => Matrix4::new(
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                theta.cos(),
                -theta.sin(),
                0.0,
                0.0,
                theta.sin(),
                theta.cos(),
            ),
        };
        for vertex in &mut self.vertices {
            vertex.apply_matrix(rotation_matrix);
        }
    }
}
