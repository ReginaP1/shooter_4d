use bevy::math::{Mat4, Vec4};

pub mod tesseract;

pub enum Rotation {
    XW,
    YW,
    ZW,
}

struct Vertex {
    position: Vec4,
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            position: Vec4::new(x, y, z, w),
        }
    }

    fn apply_matrix(&mut self, matrix: Mat4) {
        self.position = matrix * self.position;
    }

    fn translate(&mut self, dw: f32) {
        self.position.w += dw;
    }
}