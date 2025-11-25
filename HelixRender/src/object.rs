use pyo3::prelude::*;
use glam::Vec3;
use glam::Mat4;
use glam::Quat;

#[pyclass]
pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

#[pymethods]
impl Transform {
    #[new]
    pub fn new() -> Self {
        Transform {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        let roll = Quat::from_rotation_x(self.rotation.x);
        let pitch = Quat::from_rotation_y(self.rotation.y);
        let yaw = Quat::from_rotation_z(self.rotation.z);

        let q = (yaw * pitch * roll).normalize();

        let transform_matrix = Mat4::from_rotation_translation(q, self.position) * Mat4::from_scale(self.scale);

        transform_matrix.to_cols_array_2d()
    }

    pub fn translate(&mut self, delta: [f32; 3]) {
        self.position += Vec3::from(delta);
    }

    pub fn rotate(&mut self, delta: [f32; 3]) {
        self.rotation += Vec3::from(delta);
    }

    pub fn scale(&mut self, s: [f32; 3]) {
        self.scale = Vec3::from(s);
    }
}
