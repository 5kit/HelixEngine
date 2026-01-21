use crate::mesh::PyMeshHandle;
use crate::resource_manager::ResourceManager;

use crate::general_handler::ObjectStorage;

use crate::mesh_object::{self, PyMeshObjectHandle};
use mesh_object::MeshObject;

use crate::transform::{self, PyTransformNodeHandle};
use transform::TransformNode;

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

    // Insert a New Transform Node into Graph
    fn create_transform_node(
        &mut self,
        parent_node_handle: Option<PyTransformNodeHandle>,
    ) -> PyTransformNodeHandle {
        // Create Node, insert into Storage and get Handle
        let new_node = TransformNode::new(parent_node_handle.clone());
        let new_node_handle = PyTransformNodeHandle {
            handle: self.transform_node_storage.insert(new_node),
        };

        // Add the new child to parent ( if exists )
        if let Some(node_handle) = parent_node_handle.as_ref() {
            match self.transform_node_storage.resolve_mut(&node_handle.handle) {
                Some(node) => node.add_child(new_node_handle.clone()),
                None => eprintln!("Warning: invalid parent node handle!"),
            }
        }

        // Return hande
        new_node_handle
    }

    // Insert new mesh_object with transform Node
    pub fn create_mesh_object(
        &mut self,
        name: String,
        mesh: Option<PyMeshHandle>,
        parent: Option<PyMeshObjectHandle>,
    ) -> PyMeshObjectHandle {
        let parent_node_handle = parent.and_then(|parent_object_handle| {
            self.mesh_objects_storage
                .resolve(&parent_object_handle.handle)
                .map(|parent_obj| parent_obj.transform_node_handle.clone())
        });

        let new_transform_node_handle = self.create_transform_node(parent_node_handle);
        let new_obj = MeshObject {
            name,
            mesh_handle: mesh,
            transform_node_handle: new_transform_node_handle,
        };
        PyMeshObjectHandle {
            handle: self.mesh_objects_storage.insert(new_obj),
        }
    }
}
