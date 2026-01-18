use crate::transform;
use transform::TransformNodeHandle;

use crate::mesh;
use mesh::MeshHandle;

use pyo3::prelude::*;

// Mesh Object Handler for Scene
#[pyclass]
pub struct MeshObjectHandle {
    pub index: usize,
    pub generation: u32,
}

#[pymethods]
impl MeshObjectHandle {
    #[new]
    pub fn new(index: usize, generation: u32) -> Self {
        MeshObjectHandle { index, generation }
    }
}

// Basic Data Strucutre
// Most processing happens in Scene
#[derive(Clone)]
pub struct MeshObject {
    pub name: String,
    pub mesh_handle: Option<MeshHandle>,
    pub transform_node_handle: Option<TransformNodeHandle>,
}
