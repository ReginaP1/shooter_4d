use crate::vector::{calc_4d_matrix, dot4};
use crate::{Rotation, Vertex};
use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use nalgebra::{Matrix4, Vector4};

pub struct Tesseract {
    vertices: Vec<Vertex>,
}

impl Tesseract {
    pub fn new() -> Self {
        let vertices = vec![
            Vertex::new(-1.0, -1.0, -1.0, -1.0),
            Vertex::new(1.0, -1.0, -1.0, -1.0),
            Vertex::new(-1.0, 1.0, -1.0, -1.0),
            Vertex::new(1.0, 1.0, -1.0, -1.0),
            Vertex::new(-1.0, -1.0, 1.0, -1.0),
            Vertex::new(1.0, -1.0, 1.0, -1.0),
            Vertex::new(-1.0, 1.0, 1.0, -1.0),
            Vertex::new(1.0, 1.0, 1.0, -1.0),
            Vertex::new(-1.0, -1.0, -1.0, 1.0),
            Vertex::new(1.0, -1.0, -1.0, 1.0),
            Vertex::new(-1.0, 1.0, -1.0, 1.0),
            Vertex::new(1.0, 1.0, -1.0, 1.0),
            Vertex::new(-1.0, -1.0, 1.0, 1.0),
            Vertex::new(1.0, -1.0, 1.0, 1.0),
            Vertex::new(-1.0, 1.0, 1.0, 1.0),
            Vertex::new(1.0, 1.0, 1.0, 1.0),
        ];
        Self { vertices }
    }

    pub fn projected_vertices(
        &self,
        from: Vector4<f32>,
        to: Vector4<f32>,
        up: Vector4<f32>,
        over: Vector4<f32>,
        theta: f32,
    ) -> Vec<Vec3> {
        let mut projected = vec![];
        let (a, b, c, d) = calc_4d_matrix(from, to, up, over);
        let t = 1.0 / (theta / 2.0).tanh();
        for vertex in &self.vertices {
            let v = vertex.position - from;
            let s = t / dot4(&v, &d);
            let projection = Vec3::new(s * dot4(&v, &a), s * dot4(&v, &b), s * dot4(&v, &c));
            projected.push(projection);
        }
        projected
    }

    pub fn mesh(&self) -> Mesh {
        let indices = vec![
            3, 5, 7, 1, 5, 3, 11, 9, 3, 1, 3, 9, 7, 5, 15, 13, 15, 5, 3, 7, 11, 15, 11, 7, 1, 9, 5,
            13, 5, 9, 11, 15, 9, 13, 9, 15, 2, 6, 4, 8, 4, 6, 10, 12, 14, 16, 14, 12, 12, 10, 4, 2,
            4, 10, 8, 6, 16, 14, 16, 6, 12, 4, 16, 8, 16, 4, 2, 10, 6, 14, 6, 10, 1, 3, 2, 4, 2, 3,
            9, 10, 11, 12, 11, 10, 3, 1, 11, 9, 11, 1, 2, 4, 10, 12, 10, 4, 1, 2, 9, 10, 9, 2, 4,
            3, 12, 11, 12, 3, 8, 7, 4, 3, 4, 7, 16, 12, 15, 11, 15, 12, 12, 4, 11, 3, 11, 4, 16,
            15, 8, 7, 8, 15, 12, 16, 4, 8, 4, 16, 11, 3, 15, 7, 15, 3, 6, 5, 8, 7, 8, 5, 14, 16,
            13, 15, 13, 16, 5, 13, 7, 15, 7, 13, 7, 15, 8, 16, 8, 15, 8, 16, 6, 14, 6, 16, 6, 5,
            14, 13, 14, 5, 5, 1, 6, 2, 6, 1, 13, 14, 9, 10, 9, 14, 5, 13, 1, 9, 1, 13, 6, 2, 14,
            10, 14, 2, 1, 9, 2, 10, 2, 9, 13, 5, 14, 6, 14, 5,
        ];
        let mut normal_indices = vec![];
        for i in indices {
            normal_indices.push(i - 1);
        }
        let projected_vertices: Vec<Vec3> = self.projected_vertices(
            Vector4::new(5.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            45.0,
        );
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
