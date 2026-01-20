use glam::{Mat4, Vec3};

use crate::resource_manager;
use resource_manager::ResourceManager;

use crate::general_handler::{Handle, ObjectStorage};

use crate::mesh_object;
use mesh_object::MeshObject;

use crate::transform::{self, PyTransformNodeHandle};
use transform::{Transform, TransformNode};

use pyo3::prelude::*;

#[pyclass]
pub struct Scene {
    // Owner of: MeshObjects, Cameras, transformNode
    mesh_objects_storage: ObjectStorage<MeshObject>,

    transform_node_storage: ObjectStorage<TransformNode>,
}

#[pymethods]
impl Scene {
    #[new]
    pub fn new() -> Self {
        Scene {
            mesh_objects_storage: ObjectStorage::new(),
            transform_node_storage: ObjectStorage::new(),
        }
    }

    // new mesh_object
}
