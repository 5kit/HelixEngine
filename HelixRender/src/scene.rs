use crate::mesh::PyMeshHandle;
use crate::resource_manager::ResourceManager;

use crate::general_handler::{Handle, ObjectStorage};

use crate::mesh_object::{MeshObject, PyMeshObjectHandle};
use glam::{Mat4, Vec3};

use crate::camera::{Camera, PyCameraHandle};

use crate::transform::{PyTransformNodeHandle, TransformNode};

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Scene {
    // Owner of: MeshObjects, Cameras, transformNode
    mesh_objects_storage: ObjectStorage<MeshObject>,

    camera_object_storage: ObjectStorage<Camera>,
    active_camera: PyCameraHandle,

    transform_node_storage: ObjectStorage<TransformNode>,
    root: PyTransformNodeHandle,
}

#[pymethods]
impl Scene {
    #[new]
    pub fn new() -> Self {
        // inistalise node network and root
        let mut node_storage = ObjectStorage::new();
        let root = PyTransformNodeHandle {
            handle: node_storage.insert(TransformNode::new(None)),
        };

        // initialise camera storage and an inital camera
        let mut cam_storage = ObjectStorage::new();
        let active_camera = PyCameraHandle {
            handle: cam_storage.insert(Camera::new(root)),
        };

        Scene {
            mesh_objects_storage: ObjectStorage::new(),

            camera_object_storage: cam_storage,
            active_camera,

            transform_node_storage: node_storage,
            root,
        }
    }

    // -------------------------------------------
    // Transform Node Methods
    // -------------------------------------------

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
        } else {
            // Parent becomes root if no parent given
            match self.transform_node_storage.resolve_mut(&self.root.handle) {
                Some(node) => node.add_child(new_node_handle.clone()),
                None => eprintln!("Warning: invalid parent node handle!"),
            }
        }

        // Return hande
        new_node_handle
    }

    // A lazy update to all dirty nodes starting from root
    pub fn update_dirties(&mut self) {
        self.update_dirty_rec(self.root, Mat4::IDENTITY, false);
    }

    // --------------------------------------------
    // Mesh Object methods
    // --------------------------------------------

    // Insert new mesh_object with transform Node
    pub fn create_mesh_object(
        &mut self,
        name: String,
        mesh: PyMeshHandle,
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
            .resolve_transform_node_from_object(self.resolve_mesh_object(object_handle)?)?
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
        let transform_node_handle = {
            let obj = self.resolve_mesh_object_mut(object_handle)?;
            obj.transform_node_handle.handle
        };

        let node = self
            .transform_node_storage
            .resolve_mut(&transform_node_handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })?;

        node.local.translate(delta);
        node.dirty = true;

        Ok(())
    }

    // set obj position to pos
    pub fn obj_set_pos(
        &mut self,
        object_handle: PyMeshObjectHandle,
        pos: [f32; 3],
    ) -> PyResult<()> {
        let transform_node_handle = {
            let obj = self.resolve_mesh_object_mut(object_handle)?;
            obj.transform_node_handle.handle
        };

        let node = self
            .transform_node_storage
            .resolve_mut(&transform_node_handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })?;

        node.local.position = Vec3::from(pos);
        node.dirty = true;

        Ok(())
    }

    // rotate obj by delta
    pub fn obj_rotate(
        &mut self,
        object_handle: PyMeshObjectHandle,
        delta: [f32; 3],
    ) -> PyResult<()> {
        let transform_node_handle = {
            let obj = self.resolve_mesh_object_mut(object_handle)?;
            obj.transform_node_handle.handle
        };

        let node = self
            .transform_node_storage
            .resolve_mut(&transform_node_handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })?;

node.local.rotate(delta);
    node.dirty = true;

        Ok(())
    }

// set obj rotation
pub fn obj_set_rotatation(
    &mut self,
    object_handle: PyMeshObjectHandle,
    euler: [f32; 3],
) -> PyResult<()> {
    let transform_node_handle = {
        let obj = self.resolve_mesh_object_mut(object_handle)?;
        obj.transform_node_handle.handle
    };

    let node = self
        .transform_node_storage
        .resolve_mut(&transform_node_handle)
        .ok_or_else(|| {
            PyErr::new::<PyTypeError, _>(
                "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })?;

        node.local.rotation = Vec3::from(euler);
        node.dirty = true;

        Ok(())
    }

    // scale obj by delta
    pub fn obj_scale(
        &mut self,
        object_handle: PyMeshObjectHandle,
        delta: [f32; 3],
    ) -> PyResult<()> {
        let transform_node_handle = {
            let obj = self.resolve_mesh_object_mut(object_handle)?;
            obj.transform_node_handle.handle
        };

        let node = self
            .transform_node_storage
            .resolve_mut(&transform_node_handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })?;

        node.local.scale(delta);
        node.dirty = true;

        Ok(())
    }

    // set obj scale
    pub fn obj_set_scale(
        &mut self,
        object_handle: PyMeshObjectHandle,
        scaler: [f32; 3],
    ) -> PyResult<()> {
        let transform_node_handle = {
            let obj = self.resolve_mesh_object_mut(object_handle)?;
            obj.transform_node_handle.handle
        };

        let node = self
            .transform_node_storage
            .resolve_mut(&transform_node_handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })?;

        node.local.scale = Vec3::from(scaler);
        node.dirty = true;

        Ok(())
    }

    // get mesh handle from object handle
    pub fn obj_mesh(&self, object_handle: PyMeshObjectHandle) -> PyResult<PyMeshHandle> {
        if let Some(obj) = self.mesh_objects_storage.resolve(&object_handle.handle) {
            Ok(obj.mesh_handle)
        } else {
            Err(PyErr::new::<PyTypeError, _>(
                "Invalid Handle Error: Handle could not be Resolved!",
            ))
        }
    }

    // --------------------------------------------
    // Camera Object Methods TODO
    // --------------------------------------------

    pub fn new_camera(&mut self) {}

    pub fn set_active_camera(&mut self) {}

    pub fn translate_camera(&mut self) {}

    pub fn rotate_camera(&mut self) {}

    pub fn render(&self) {}

    pub fn render_from(&self) {}
}

// Helper functions that cant be exposed to python
impl Scene {
    // Resolve Mesh Object functions
    pub fn resolve_mesh_object(&self, object_handle: PyMeshObjectHandle) -> PyResult<&MeshObject> {
        self.mesh_objects_storage
            .resolve(&object_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: mesh Handle could not be Resolved!",
                )
            })
    }

    pub fn resolve_mesh_object_mut(
        &mut self,
        object_handle: PyMeshObjectHandle,
    ) -> PyResult<&mut MeshObject> {
        self.mesh_objects_storage
            .resolve_mut(&object_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: mesh Handle could not be Resolved!",
                )
            })
    }

    // Resolve Camera functions
    pub fn resolve_camera(&self, cam_handle: PyCameraHandle) -> PyResult<&Camera> {
        self.camera_object_storage
            .resolve(&cam_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: mesh Handle could not be Resolved!",
                )
            })
    }

    pub fn resolve_camera_mut(&mut self, cam_handle: PyCameraHandle) -> PyResult<&mut Camera> {
        self.camera_object_storage
            .resolve_mut(&cam_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: mesh Handle could not be Resolved!",
                )
            })
    }

    // Resolve Node From Mesh Object functions
    pub fn resolve_transform_node_from_object(&self, obj: &MeshObject) -> PyResult<&TransformNode> {
        self.transform_node_storage
            .resolve(&obj.transform_node_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })
    }

    pub fn resolve_transform_node_from_object_mut(
        &mut self,
        obj: &MeshObject,
    ) -> PyResult<&mut TransformNode> {
        self.transform_node_storage
            .resolve_mut(&obj.transform_node_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })
    }

    // Resolve Node From Camera functions
    pub fn resolve_transform_node_from_camera(&self, obj: &Camera) -> PyResult<&TransformNode> {
        self.transform_node_storage
            .resolve(&obj.transform_node_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })
    }

    pub fn resolve_transform_node_from_camera_mut(
        &mut self,
        obj: &Camera,
    ) -> PyResult<&mut TransformNode> {
        self.transform_node_storage
            .resolve_mut(&obj.transform_node_handle.handle)
            .ok_or_else(|| {
                PyErr::new::<PyTypeError, _>(
                    "Invalid Handle Error: node Handle could not be Resolved!",
                )
            })
    }

    // Update dirty nodes recursively

    fn update_dirty_rec(
        &mut self,
        root: PyTransformNodeHandle,
        parent_matrix: Mat4,
        inherit: bool,
    ) {
        let mut node_children = Vec::new();
        // default pass values
        let mut world_matrix = parent_matrix;
        let mut update = false;

        if let Some(node) = self.transform_node_storage.resolve_mut(&root.handle) {
            // update values only if dirty or child of a dirty
            if node.dirty || inherit {
                // updated node values
                node.world = parent_matrix * node.local.get_matrix();
                node.dirty = false;
                // all children must recalculate
                update = true;
            }

            // values to recurse with outside loop
            world_matrix = node.world;
            node_children = node.children.clone();
        }

        // call recursively for every child of node
        for child in node_children {
            self.update_dirty_rec(child, world_matrix, update);
        }
    }
}
