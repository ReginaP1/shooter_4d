use nalgebra::{Matrix4, Vector4};

pub mod tesseract;
mod vector;

pub enum Rotation {
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
