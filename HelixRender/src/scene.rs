use crate::mesh::PyMeshHandle;
use crate::resource_manager::ResourceManager;

use crate::general_handler::{Handle, ObjectStorage};

use crate::mesh_object::{self, PyMeshObjectHandle};
use glam::Vec3;
use mesh_object::MeshObject;

use crate::transform::{self, PyTransformNodeHandle};
use transform::TransformNode;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
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

    // get name of object from obj handle
    pub fn obj_name(&self, object_handle: PyMeshObjectHandle) -> PyResult<String> {
        if let Some(obj) = self.mesh_objects_storage.resolve(&object_handle.handle) {
            Ok(obj.name.to_string())
        } else {
            Err(PyErr::new::<PyTypeError, _>(
                "Invalid Handle Error: Hanlde could not be Resolved!",
            ))
        }
    }

    // get transformation matrix obj from handle
    pub fn obj_transform(&self, object_handle: PyMeshObjectHandle) -> PyResult<[[f32; 4]; 4]> {
        Ok(self
            .resolve_object_and_node(object_handle)?
            .local
            .get_matrix()
            .to_cols_array_2d())
    }

    // translate obj by delta
    pub fn obj_translate(
        &mut self,
        object_handle: PyMeshObjectHandle,
        delta: [f32; 3],
    ) -> PyResult<()> {
        self.resolve_object_and_node_mut(object_handle)?
            .local
            .translate(delta);
        Ok(())
    }

    // set obj position to pos
    pub fn obj_set_pos(
        &mut self,
        object_handle: PyMeshObjectHandle,
        pos: [f32; 3],
    ) -> PyResult<()> {
        self.resolve_object_and_node_mut(object_handle)?
            .local
            .position = Vec3::from(pos);
        Ok(())
    }

    // rotate obj by delta
    pub fn obj_rotate(
        &mut self,
        object_handle: PyMeshObjectHandle,
        delta: [f32; 3],
    ) -> PyResult<()> {
        self.resolve_object_and_node_mut(object_handle)?
            .local
            .rotate(delta);
        Ok(())
    }

    // set obj rotation
    pub fn obj_set_rotatation(
        &mut self,
        object_handle: PyMeshObjectHandle,
        euler: [f32; 3],
    ) -> PyResult<()> {
        self.resolve_object_and_node_mut(object_handle)?
            .local
            .rotation = Vec3::from(euler);
        Ok(())
    }

    // scale obj by delta
    pub fn obj_scale(
        &mut self,
        object_handle: PyMeshObjectHandle,
        delta: [f32; 3],
    ) -> PyResult<()> {
        self.resolve_object_and_node_mut(object_handle)?
            .local
            .scale(delta);
        Ok(())
    }

    // set obj scale
    pub fn obj_set_scale(
        &mut self,
        object_handle: PyMeshObjectHandle,
        scaler: [f32; 3],
    ) -> PyResult<()> {
        self.resolve_object_and_node_mut(object_handle)?.local.scale = Vec3::from(scaler);
        Ok(())
    }
}

// Helper functions that cant be exposed to python
impl Scene {
    fn resolve_object_and_node_mut(
        &mut self,
        object_handle: PyMeshObjectHandle,
    ) -> PyResult<&mut TransformNode> {
        let obj = self
            .mesh_objects_storage
            .resolve(&object_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: mesh Handle could not be Resolved!",
                )
            })?;

        self.transform_node_storage
            .resolve_mut(&obj.transform_node_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })
    }

    fn resolve_object_and_node(
        &self,
        object_handle: PyMeshObjectHandle,
    ) -> PyResult<&TransformNode> {
        let obj = self
            .mesh_objects_storage
            .resolve(&object_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: mesh Handle could not be Resolved!",
                )
            })?;

        self.transform_node_storage
            .resolve(&obj.transform_node_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })
    }
}
