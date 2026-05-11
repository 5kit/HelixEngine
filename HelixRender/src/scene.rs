use crate::mesh::PyMeshHandle;
use crate::resource_manager::ResourceManager;

use crate::general_handler::{Handle, ObjectStorage};

use crate::mesh_object::{MeshObject, PyMeshObjectHandle};
use glam::{Mat4, Vec3};

use crate::camera::{self, Camera, PyCameraHandle};

use crate::transform::{
    PyTransformNodeHandle, PyTransformObjectHandle, TransformNode, TransformType,
};

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct Scene {
    // Owner of: MeshObjects, Cameras, transformNode
    mesh_objects_storage: ObjectStorage<MeshObject>,

    camera_object_storage: ObjectStorage<Camera>,
    active_camera: PyCameraHandle,

    transform_node_storage: ObjectStorage<TransformNode>,
    root: PyTransformNodeHandle,
}

// ------------------------------------------------------------------
//  TODO:
//  - add scene handling + engine handling
//  - add validation to check weather handles belong to the scene
//  - simplify python interface
// ------------------------------------------------------------------

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
        // Default to root if no parent supplied
        let parent_handle = parent_node_handle.unwrap_or_else(|| self.root.clone());

        // Create node with actual parent
        let new_node_handle = PyTransformNodeHandle {
            handle: self
                .transform_node_storage
                .insert(TransformNode::new(Some(parent_handle.clone()))),
        };

        // Register child with parent
        if let Some(parent_node) = self
            .transform_node_storage
            .resolve_mut(&parent_handle.handle)
        {
            parent_node.add_child(new_node_handle.clone());
        } else {
            eprintln!("Warning: invalid parent node handle!");
        }

        new_node_handle
    }

    fn apply_transform(
        &mut self,
        object_handle: PyTransformObjectHandle,
        t_delta: Option<[f32; 3]>,
        r_delta: Option<[f32; 3]>,
        s_delta: Option<[f32; 3]>,
    ) -> PyResult<()> {
        self.with_transform_mut(&object_handle, |node| {
            if let Some(t) = t_delta {
                node.local.position += Vec3::from_array(t);
            }
            if let Some(r) = r_delta {
                node.local.rotation += Vec3::from_array(r);
            }
            if let Some(s) = s_delta {
                node.local.scale += Vec3::from_array(s);
            }
        })
    }

    fn set_transform(
        &mut self,
        object_handle: PyTransformObjectHandle,
        t_val: Option<[f32; 3]>,
        r_val: Option<[f32; 3]>,
        s_val: Option<[f32; 3]>,
    ) -> PyResult<()> {
        self.with_transform_mut(&object_handle, |node| {
            if let Some(t) = t_val {
                node.local.position = Vec3::from_array(t);
            }
            if let Some(r) = r_val {
                node.local.rotation = Vec3::from_array(r);
            }
            if let Some(s) = s_val {
                node.local.scale = Vec3::from_array(s);
            }
        })
    }

    // A lazy update to all dirty nodes starting from root
    pub fn update_dirty_transforms(&mut self) {
        self.update_dirty_rec(self.root.clone(), Mat4::IDENTITY, false);
    }

    // --------------------------------------------
    // Mesh Object Methods
    // --------------------------------------------

    // Insert new mesh_object with transform Node
    pub fn create_mesh_object(
        &mut self,
        name: String,
        mesh: PyMeshHandle,
        parent: Option<PyMeshObjectHandle>,
    ) -> PyResult<PyMeshObjectHandle> {
        let parent_node_handle = match parent {
            Some(parent_handle) => {
                let parent_obj = self.resolve_mesh_object(&parent_handle)?;

                Some(parent_obj.transform_node_handle.clone())
            }
            None => None,
        };

        let new_transform_node_handle = self.create_transform_node(parent_node_handle);

        let new_obj = MeshObject {
            name,
            mesh_handle: mesh,
            transform_node_handle: new_transform_node_handle,
        };

        Ok(PyMeshObjectHandle {
            handle: self.mesh_objects_storage.insert(new_obj),
        })
    }

    // get name of object from obj handle
    pub fn object_get_name(&self, object_handle: PyMeshObjectHandle) -> PyResult<String> {
        Ok(self.resolve_mesh_object(&object_handle)?.name.clone())
    }

    // get transformation matrix obj from handle
    pub fn object_get_transform(
        &self,
        object_handle: PyMeshObjectHandle,
    ) -> PyResult<[[f32; 4]; 4]> {
        let obj = self.resolve_mesh_object(&object_handle)?;
        let node = self.resolve_transform_node(&obj.transform_node_handle)?;

        Ok(node.local.get_matrix().to_cols_array_2d())
    }

    // translate obj by delta
    pub fn object_translate(
        &mut self,
        object_handle: PyMeshObjectHandle,
        delta: [f32; 3],
    ) -> PyResult<()> {
        self.apply_transform(object_handle.to_transform(), Some(delta), None, None)
    }

    // set obj position to pos
    pub fn object_set_pos(
        &mut self,
        object_handle: PyMeshObjectHandle,
        pos: [f32; 3],
    ) -> PyResult<()> {
        self.set_transform(object_handle.to_transform(), Some(pos), None, None)
    }

    // rotate obj by delta
    pub fn object_rotate(
        &mut self,
        object_handle: PyMeshObjectHandle,
        delta: [f32; 3],
    ) -> PyResult<()> {
        self.apply_transform(object_handle.to_transform(), None, Some(delta), None)
    }

    // set obj rotation
    pub fn object_set_rotation(
        &mut self,
        object_handle: PyMeshObjectHandle,
        euler: [f32; 3],
    ) -> PyResult<()> {
        self.set_transform(object_handle.to_transform(), None, Some(euler), None)
    }

    // scale obj by delta
    pub fn object_scale(
        &mut self,
        object_handle: PyMeshObjectHandle,
        delta: [f32; 3],
    ) -> PyResult<()> {
        self.apply_transform(object_handle.to_transform(), None, None, Some(delta))
    }

    // set obj scale
    pub fn object_set_scale(
        &mut self,
        object_handle: PyMeshObjectHandle,
        scaler: [f32; 3],
    ) -> PyResult<()> {
        self.set_transform(object_handle.to_transform(), None, None, Some(scaler))
    }

    // get mesh handle from object handle
    pub fn object_get_mesh(&self, object_handle: PyMeshObjectHandle) -> PyResult<PyMeshHandle> {
        Ok(self
            .resolve_mesh_object(&object_handle)?
            .mesh_handle
            .clone())
    }

    // --------------------------------------------
    // Camera Object Methods TODO
    // --------------------------------------------

    // Insert new camera with transform Node
    pub fn new_camera(
        &mut self,
        parent: Option<PyTransformObjectHandle>,
    ) -> PyResult<PyCameraHandle> {
        let parent_node_handle = match parent {
            Some(h) => Some(self.resolve_transform_handle(&h)?),
            None => None,
        };

        let new_transform_node_handle = self.create_transform_node(parent_node_handle);

        let new_cam = Camera::new(new_transform_node_handle);

        Ok(PyCameraHandle {
            handle: self.camera_object_storage.insert(new_cam),
        })
    }

    // change active camera in scene
    pub fn set_active_camera(&mut self, cam: PyCameraHandle) -> PyResult<()> {
        self.resolve_camera(&cam)?;
        self.active_camera = cam;
        Ok(())
    }

    // get transformation matrix camera from handle
    pub fn camera_get_transform(&self, camera_handle: PyCameraHandle) -> PyResult<[[f32; 4]; 4]> {
        let cam = self.resolve_camera(&camera_handle)?;
        let node = self.resolve_transform_node(&cam.transform_node_handle)?;

        Ok(node.local.get_matrix().to_cols_array_2d())
    }

    // translate camera by delta
    pub fn camera_translate(
        &mut self,
        camera_handle: PyCameraHandle,
        delta: [f32; 3],
    ) -> PyResult<()> {
        self.apply_transform(camera_handle.to_transform(), Some(delta), None, None)
    }

    // set camera position to pos
    pub fn camera_set_pos(&mut self, camera_handle: PyCameraHandle, pos: [f32; 3]) -> PyResult<()> {
        self.set_transform(camera_handle.to_transform(), Some(pos), None, None)
    }

    // rotate camera by delta
    pub fn camera_rotate(
        &mut self,
        camera_handle: PyCameraHandle,
        delta: [f32; 3],
    ) -> PyResult<()> {
        self.apply_transform(camera_handle.to_transform(), None, Some(delta), None)
    }

    // set camera rotation
    pub fn camera_set_rotation(
        &mut self,
        camera_handle: PyCameraHandle,
        euler: [f32; 3],
    ) -> PyResult<()> {
        self.set_transform(camera_handle.to_transform(), None, Some(euler), None)
    }

    pub fn render(&self) {}

    pub fn render_from(&self) {}
}

// -----------------------------------------------------
// Helper functions that cant be exposed to python
// -----------------------------------------------------

impl Scene {
    // Generic Handle -> obj resolvers
    // generic storage with ownership lifetime

    // non-mut resolve
    fn resolve<'a, T>(storage: &'a ObjectStorage<T>, handle: Handle) -> PyResult<&'a T> {
        storage
            .resolve(&handle)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid handle"))
    }

    // mut resolve
    fn resolve_mut<'a, T>(
        storage: &'a mut ObjectStorage<T>,
        handle: Handle,
    ) -> PyResult<&'a mut T> {
        storage
            .resolve_mut(&handle)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyTypeError, _>("Invalid handle"))
    }

    // resolver wrappers
    pub fn resolve_mesh_object(&self, h: &PyMeshObjectHandle) -> PyResult<&MeshObject> {
        Self::resolve(&self.mesh_objects_storage, h.handle)
    }
    pub fn resolve_mesh_object_mut(&mut self, h: &PyMeshObjectHandle) -> PyResult<&mut MeshObject> {
        Self::resolve_mut(&mut self.mesh_objects_storage, h.handle)
    }
    pub fn resolve_camera(&self, h: &PyCameraHandle) -> PyResult<&Camera> {
        Self::resolve(&self.camera_object_storage, h.handle)
    }
    pub fn resolve_camera_mut(&mut self, h: &PyCameraHandle) -> PyResult<&mut Camera> {
        Self::resolve_mut(&mut self.camera_object_storage, h.handle)
    }
    pub fn resolve_transform_node(&self, h: &PyTransformNodeHandle) -> PyResult<&TransformNode> {
        Self::resolve(&self.transform_node_storage, h.handle)
    }
    pub fn resolve_transform_node_mut(
        &mut self,
        h: &PyTransformNodeHandle,
    ) -> PyResult<&mut TransformNode> {
        Self::resolve_mut(&mut self.transform_node_storage, h.handle)
    }

    // TransformObjectHandle -> TransformHandle
    pub fn resolve_transform_handle(
        &self,
        h: &PyTransformObjectHandle,
    ) -> PyResult<PyTransformNodeHandle> {
        match h.identity {
            TransformType::MeshObject => {
                let obj = self.resolve_mesh_object(&PyMeshObjectHandle { handle: h.handle })?;

                Ok(obj.transform_node_handle.clone())
            }

            TransformType::Camera => {
                let cam = self.resolve_camera(&PyCameraHandle { handle: h.handle })?;

                Ok(cam.transform_node_handle.clone())
            }
        }
    }

    // Generic transform mutator wrapper
    fn with_transform_mut<F>(&mut self, h: &PyTransformObjectHandle, f: F) -> PyResult<()>
    where
        F: FnOnce(&mut TransformNode),
    {
        let node_handle = self.resolve_transform_handle(h)?;

        let node = self.resolve_transform_node_mut(&node_handle)?;

        f(node);
        node.dirty = true;

        Ok(())
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
