use bevy::prelude::Vec3;
use nalgebra::{Matrix4, Vector3, Vector4};

pub(crate) enum Rotation {
    XW,
    YW,
    ZW,
}

struct Vertex {
    position: Vector4<f32>,
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            position: Vector4::new(x, y, z, w),
        }
    }

    fn apply_matrix(&mut self, matrix: Matrix4<f32>) {
        self.position = matrix * self.position;
    }

    fn translate(&mut self, dw: f32) {
        self.position.w += dw;
    }
}

pub struct HyperCube {
    vertices: Vec<Vertex>,
}

impl HyperCube {
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

    pub(crate) fn translate(&mut self, dw: f32) {
        for v in &mut self.vertices {
            v.translate(dw);
        }
    }
}
