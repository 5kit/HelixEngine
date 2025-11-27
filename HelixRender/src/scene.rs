use crate::object;
use crate::camera;

use pyo3::prelude::*;
use object::{MeshObject, Mesh};
use camera::Camera;

#[pyclass]
pub struct Scene {
    objects: Vec<Option<MeshObject>>,   // None when not in use
    object_gen: Vec<u32>,               // represents the newest object in slot

    cameras: Vec<Option<Camera>>,       // None when not in use
    camera_gen: Vec<u32>,               // represents the newest object in slot
    active_camera: usize,               // index of current in use camera

    meshes: Vec<Option<Mesh>>,          // None when not in use
    mesh_gen: Vec<u32>,                 // represents the newest mesh in slot
}

#[pymethods]
impl Scene {
    #[new]
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            object_gen: Vec::new(),
            cameras: Vec::new(Camera::new()),
            camera_gen: Vec::new([0]),
            active_camera: 0,
            meshes: Vec::new(),
            mesh_gen: Vec::new(),
        }
    }

    // Clear the scene
    pub fn clear(&mut self) {
        self.background_color = [1.0, 1.0, 1.0];
        self.objects.clear();
        self.object_gen.clear();
        self.meshes.clear();
        self.mesh_gen.clear();
    }

    //  ---------------------------------------------
    // Object Management
    //  ---------------------------------------------

    pub fn add_object(&mut self, name: String) -> PyResult<PyObjectHandle> {
        let obj = MeshObject::new(name);

        // Search for empty slot:
        for i in 0..self.objects.len() {
            if self.objects[i].is_none() {
                // set slot to obj and increment generation
                self.objects[i] = Some(obj.clone());
                return Ok(PyObjectHandle::new(i, self.object_gen[i]));
            }
        }

        // If no empty slots push new slot
        self.objects.push(Some(obj));
        self.object_gen.push(0);

        return Ok(PyObjectHandle::new(self.objects.len()-1, 0));
    }

    // get object handler by name
    pub fn get_object(&self, name: &str) -> Option<PyObjectHandle> {
        for i in 0..self.objects.len() {
            if let Some(obj) = self.objects[i].as_ref() {
                if obj.name == name {
                    return Some(PyObjectHandle::new(i, self.object_gen[i]));
                }
            }
        }
        // Not found
        None
    }
    
    pub fn delete_object(&mut self, h: &PyObjectHandle) {
        if self.resolve_obj(h).is_some() {
            self.objects[h.index] = None;
            self.object_gen[h.index] += 1;
        }
    }

    //  ---------------------------------------------
    // Object Transforms
    //  ---------------------------------------------

    pub fn translate_object(&mut self, h: &PyObjectHandle, vector: [f32; 3]) {
        if let Some(obj) = self.resolve_obj_mut(h) {
            obj.transform.translate(vector);
        }
    }

    pub fn rotate_object(&mut self, h: &PyObjectHandle, rotation: [f32; 3]) {
        if let Some(obj) = self.resolve_obj_mut(h) {
            obj.transform.rotate(rotation);
        }
    }

    pub fn scale_object(&mut self, h: &PyObjectHandle, scalar: [f32; 3]) {
        if let Some(obj) = self.resolve_obj_mut(h) {
            obj.transform.scale(scalar);
        }
    }

    pub fn get_object_matrix(&self, h: &PyObjectHandle) -> Option<[[f32; 4]; 4]> {
        if let Some(obj) = self.resolve_obj(h) {
            Some(obj.transform.get_matrix())
        } else {
            None
        }
    }

    // ---------------------------------------------
    // Camera Management
    // ---------------------------------------------

    pub fn create_camera(&mut self, name: &str) -> PyResult<PyCameraHandle> {
        let cam = Camera::new(name);

        // Search for empty slot:
        for i in 0..self.cameras.len() {
            if self.cameras[i].is_none() {
                // set slot to obj and increment generation
                self.cameras[i] = Some(cam.clone());
                return Ok(PyCameraHandle::new(i, self.camera_gen[i]));
            }
        }

        // If no empty slots push new slot
        self.cameras.push(Some(cam));
        self.camera_gen.push(0);

        return Ok(PyCameraHandle::new(self.cameras.len()-1, 0));
    }

    pub fn delete_camera(&mut self, h: &PyCameraHandle) {
        if self.resolve_cam(h).is_some() {
            if h.index != self.active_camera {
                self.cameras[h.index] = None;
                self.camera_gen[h.index] += 1;
            }
        }
    }

    //pub fn get_camera_view(&self, h: &PyCameraHandle) -> [[f32;4];4]
    //pub fn get_camera_projection(&self, h: &PyCameraHandle) -> [[f32;4];4]
    //pub fn translate_camera(&mut self, ...)
    //pub fn rotate_camera(&mut self, ...)


    // ---------------------------------------------
    // Mesh Management
    // ---------------------------------------------

    pub fn create_mesh(&mut self, mesh_path: String) -> PyResult<PyMeshHandle> {
        // Load mesh from path
        let mut mesh = Mesh::new();
        mesh.load_obj(&mesh_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        // Search for empty slot:
        for i in 0..self.meshes.len() {
            if self.meshes[i].is_none() {
                // set slot to meshes and increment generation
                self.meshes[i] = Some(mesh.clone());
                self.mesh_gen[i] += 1;
                return Ok(PyMeshHandle::new(i, self.mesh_gen[i]));
            }
        }

        // Push mesh and object to the scene
        self.meshes.push(Some(mesh));
        self.mesh_gen.push(0);
        return Ok(PyMeshHandle::new(self.meshes.len()-1, 0));
    }

    pub fn delete_mesh(&mut self, h: &PyMeshHandle) {
        if self.resolve_mesh(h).is_some() {
            self.meshes[h.index] = None;
            self.mesh_gen[h.index] += 1;
        }
    }

    // Apply mesh to object using their handlers
    pub fn apply_mesh_to_object(&mut self, obj_handle: &PyObjectHandle, mesh_handle: &PyMeshHandle) -> PyResult<()> {
        let mesh = self.resolve_mesh(mesh_handle) 
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid object handle"))?;
        let obj = self.resolve_obj_mut(obj_handle)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid object handle"))?;
        
        obj.set_mesh(mesh_handle.index);
        Ok(())
    }

    
}

// ---------------------------------------
// Internal resolver functions
// ---------------------------------------

impl Scene {

    // Object handle resolvers

    fn resolve_obj(&self, h: &PyObjectHandle) -> Option<&MeshObject> {
        if self.object_gen[h.index] == h.generation {
            self.objects[h.index].as_ref()
        } else {
            None
        }
    }

    fn resolve_obj_mut(&mut self, h: &PyObjectHandle) -> Option<&mut MeshObject> {
        if self.object_gen[h.index] == h.generation {
            self.objects[h.index].as_mut()
        } else {
            None
        }
    }


    // Camera handle resolvers

    fn resolve_cam(&self, h: &PyCameraHandle) -> Option<&MeshObject> {
        if self.camera_gen[h.index] == h.generation {
            self.cameras[h.index].as_ref()
        } else {
            None
        }
    }

    fn resolve_cam_mut(&mut self, h: &PyCameraHandle) -> Option<&mut MeshObject> {
        if self.camera_gen[h.index] == h.generation {
            self.cameras[h.index].as_mut()
        } else {
            None
        }
    }


    // Mesh handle resolvers

    fn resolve_mesh(&self, h: &PyMeshHandle) -> Option<&Mesh> {
        if self.mesh_gen[h.index] == h.generation {
            self.meshes[h.index].as_ref()
        } else {
            None
        }
    }

    fn resolve_mesh_mut(&mut self, h: &PyMeshHandle) -> Option<&mut Mesh> {
        if self.mesh_gen[h.index] == h.generation {
            self.meshes[h.index].as_mut()
        } else {
            None
        }
    }
}

// ------------------------------
// Python Handle Structs
// ------------------------------

// Object Handler for python
#[pyclass]
pub struct PyObjectHandle {
    index: usize,
    generation: u32,
}

#[pymethods]
impl PyObjectHandle {
    #[new]
    pub fn new(index: usize, generation: u32) -> Self {
        PyObjectHandle{
            index,
            generation,
        }
    }
}

// Camera Handler for python
#[pyclass]
pub struct PyCameraHandle {
    index: usize,
    generation: u32,
}

#[pymethods]
impl PyCameraHandle {
    #[new]
    pub fn new(index: usize, generation: u32) -> Self {
        Self { 
            index, 
            generation,
        }
    }
}


// Mesh Handler for python
#[pyclass]
pub struct PyMeshHandle {
    index: usize,
    generation: u32,
}

#[pymethods]
impl PyMeshHandle {
    #[new]
    pub fn new(index: usize, generation: u32) -> Self {
        PyMeshHandle{
            index,
            generation,
        }
    }
}
