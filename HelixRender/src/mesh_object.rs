use crate::general_handler::Handle;

use crate::scene::Scene;

use crate::transform::PyTransformNodeHandle;

use crate::mesh::PyMeshHandle;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

// Mesh Object Handler for Scene
#[pyclass]
#[derive(Clone)]
pub struct PyMeshObjectHandle {
    pub handle: Handle,
}

// Basic Data Strucutre
// Related Objects Stored elsewhere, refrenced here
#[derive(Clone)]
pub struct MeshObject {
    pub name: String,
    pub mesh_handle: PyMeshHandle,
    pub transform_node_handle: PyTransformNodeHandle,
}
