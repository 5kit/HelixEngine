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
    pub fn new() -> Self {
        MeshObject {
        }
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
    pub rotation: Quat,
    pub scale: Vec3,
}

#[pymethods]
impl Transform {
    #[new]
    pub fn new() -> Self {
        Transform {
            position: ,
            rotation: [0.0, 0.0, 0.0, 0.0], # a + bi + cj + dk
            scale: [1.0, 1.0, 1.0],
        }
    }

    pub fn get_matrix(&self) -> Mat4 {
        [
            [1.0, 0.0, 0.0, self.position[0]],
            [0.0, 1.0, 0.0, self.position[1]],
            [0.0, 0.0, 1.0, self.position[2]],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}
