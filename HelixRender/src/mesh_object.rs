use crate::general_handler::Handle;
use crate::mesh::PyMeshHandle;
use crate::scene::Scene;
use crate::transform::{PyTransformNodeHandle, PyTransformObjectHandle, TransformType};

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

// Mesh Object Handler for Scene
#[pyclass]
#[derive(Clone)]
pub struct PyMeshObjectHandle {
    pub handle: Handle,
}

impl PyMeshObjectHandle {
    pub fn from_transform(obj: PyTransformObjectHandle) -> PyResult<Self> {
        match obj.identity {
            TransformType::MeshObject => Ok(Self { handle: obj.handle }),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected MeshObject",
            )),
        }
    }

    pub fn to_transform(&self) -> PyTransformObjectHandle {
        PyTransformObjectHandle {
            identity: TransformType::MeshObject,
            handle: self.handle,
        }
    }
}

// Basic Data Strucutre
// Related Objects Stored elsewhere, refrenced here
#[derive(Clone)]
pub struct MeshObject {
    pub name: String,
    pub mesh_handle: PyMeshHandle,
    pub transform_node_handle: PyTransformNodeHandle,
}
