use pyo3::prelude::*;
use glam::Vec3;
use glam::Mat4;
use glam::Quat;

#[pyclass]
pub struct MeshObject {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub mesh_data: usize,
    #[pyo3(get, set)]
    pub material: usize,
    #[pyo3(get, set)]
    pub transform: Transform,
}

#[pymethods]
impl MeshObject {
    #[new]
    pub fn new(name: String, mesh_data: usize, mesh_material: usize) -> Self {
        self.name = name;
        self.mesh_data = mesh_data;
        self.material = material
        self.transform = Transform();
    }
}

#[pyclass]
pub struct Mesh {
    #[pyo3(get, set)]

}

#[pymethods]
impl Mesh {
    #[new]
    pub fn new() -> Self {
        Mesh {

        }
    }
}

#[pyclass]
pub struct Material {
    #[pyo3(get, set)]

}

#[pyclass]
pub struct Transform {
    #[pyo3(get, set)]
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

#[pymethods]
impl Transform {
    #[new]
    pub fn new() -> Self {
        Transform {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO, // Roll, Pitch, Yaw in radians
            scale: Vec3::ONE,
        }
    }

    pub fn get_matrix(&self) -> Mat4 {
        // Roll 
        let roll   = Quat::from_rotation_x(self.rotation[0]);
        // Pitch
        let pitch  = Quat::from_rotation_y(self.rotation[1]);
        // Yaw
        let yaw    = Quat::from_rotation_z(self.rotation[2]);
        
        // apply roll -> pitch -> yaw
        let q = (yaw * pitch * roll).normalize();

        // convert to 4x4 matrix and also apply a local scale
        let transform_matrix = Mat4::from_rotation_translation(q, self.position) * Mat4::from_scale(self.scale);

        return transform_matrix;
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }

    pub fn rotate(&mut self, delta: Vec3) {
        self.rotation = delta * self.rotation;
    }

    pub fn scale(&mut self, s: Vec3) {
        self.scale *= s;
    }
}
