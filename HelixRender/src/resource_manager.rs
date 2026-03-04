use glam::Vec3;

use crate::general_handler::{Handle, ObjectStorage};
use crate::mesh::{Mesh, PyMeshHandle};

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct ResourceManager {
    // mesh store uses a general_handler wrapper
    mesh_store: ObjectStorage<Mesh>,
}

// Python Exposed Methods
#[pymethods]
impl ResourceManager {
    #[new]
    pub fn new() -> Self {
        ResourceManager {
            mesh_store: ObjectStorage::new(),
        }
    }

    // Loads a new obj Mesh form
    pub fn load_mesh(&mut self, path: &str) -> PyResult<PyMeshHandle> {
        let mut mesh = Mesh::new();
        mesh.load_obj(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        Ok(PyMeshHandle {
            handle: self.mesh_store.insert(mesh),
        })
    }

    pub fn get_mesh(&self, mesh_handle: PyMeshHandle) -> PyResult<Vec<[f32; 3]>> {
        if let Some(mesh) = self.mesh_store.resolve(&mesh_handle.handle) {
            // Map Vec3 to [f32; 3] which PyO3 can convert to Python lists
            let vertices = mesh.vertices.iter().map(|v| v.to_array()).collect();
            Ok(vertices)
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                "Invalid Handle Error: Mesh Handle could not be Resolved!",
            ))
        }
    }

    // delete mesh from mesh store
    pub fn delete_mesh(&mut self, mesh_handle: PyMeshHandle) -> PyResult<bool> {
        Ok(self.mesh_store.remove(mesh_handle.handle).is_some())
    }
}
