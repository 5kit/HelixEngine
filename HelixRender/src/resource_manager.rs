use crate::general_handler::{Handle, ObjectStorage};
use crate::mesh::{Mesh, PyMeshHandle};

use pyo3::prelude::*;

#[pyclass]
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

    pub fn delete_mesh(&mut self, mesh_handle: PyMeshHandle) -> bool {
        self.mesh_store.remove(mesh_handle.handle).is_some()
    }
}
