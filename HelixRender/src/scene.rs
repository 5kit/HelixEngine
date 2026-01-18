use glam::{Mat4, Vec3};

use crate::mesh_object;
use mesh_object::{MeshObject, MeshObjectHandle};

use crate::camera;
use camera::{Camera, CameraHandle};

use crate::transform;
use transform::{Transform, TransformNode, TransformNodeHandle};

use pyo3::prelude::*;

#[pyclass]
pub struct Scene {
    // Owner of: MeshObjects, Cameras, transformNode
    mesh_objects: Vec<Option<MeshObject>>,
    mesh_object_gen: Vec<u32>,

    camera: Vec<Option<Camera>>,
    camera_gen: Vec<u32>,

    transform_nodes: Vec<Option<TransformNode>>,
    transform_node_gen: Vec<u32>,
}

#[pymethods]
impl Scene {
    #[new]
    pub fn new() -> Self {
        Scene {
            mesh_objects: Vec::new(),
            mesh_object_gen: Vec::new(),

            camera: Vec::new(),
            camera_gen: Vec::new(),

            transform_nodes: Vec::new(),
            transform_node_gen: Vec::new(),
        }
    }
}
