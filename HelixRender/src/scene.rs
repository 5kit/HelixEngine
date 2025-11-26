use crate::object;

use pyo3::prelude::*;
use object::{MeshObject, Mesh};

#[pyclass]
pub struct Scene {
    #[pyo3(get, set)]
    pub background_color: [f32; 3],

    // Not exposed to python
    objects: Vec<Option<MeshObject>>, // None when not in use
    generations: Vec<u32>, // represents the newest object in slot
    meshes: Vec<Mesh>,
}

#[pymethods]
impl Scene {
    #[new]
    pub fn new() -> Self {
        Scene {
            background_color: [1.0, 1.0, 1.0],
            objects: Vec::new(),
            generations: Vec::new(),
            meshes: Vec::new(),
        }
    }

    pub fn get_object(&self, name: &str) -> Option<PyObjectHandle> {
        for i in 0..self.objects.len() {
            if let Some(obj) = self.objects[i].as_ref() {
                if obj.name == name {
                    return Some(PyObjectHandle::new(i, self.generations[i]));
                }
            }
        }
        None
    }

    pub fn add_object(&mut self, name: String, mesh_path: String) -> PyResult<()> {
        // Load mesh from path
        let mut mesh = Mesh::new();
        mesh.load_obj(&mesh_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        // Index of mesh in the mesh array owned by scene
        let mesh_index = self.meshes.len();

        // Push mesh and object to the scene
        self.meshes.push(mesh);
        let obj = Some(MeshObject::new(name, mesh_index));

        // Search for empty slot:
        for i in 0..self.objects.len() {
            if self.objects[i].is_none() {
                // set slot to obj and increment generation
                self.objects[i] = obj.clone();
                self.generations[i] += 1;
                return Ok(())
            }
        }

        // If no empty slots push new slot
        self.objects.push(obj);
        self.generations.push(0);

        Ok(())
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.generations.clear();
        self.meshes.clear();
    }


    // Object Handler functions

    pub fn delete_object(&mut self, h: &PyObjectHandle) {
        if self.resolve(h).is_some() {
            self.objects[h.index] = None;
            self.generations[h.index] += 1;
        }
    }

    pub fn translate_object(&mut self, h: &PyObjectHandle, vector: [f32; 3]) {
        if let Some(obj) = self.resolve_mut(h) {
            obj.transform.translate(vector);
        }
    }

    pub fn rotate_object(&mut self, h: &PyObjectHandle, rotation: [f32; 3]) {
        if let Some(obj) = self.resolve_mut(h) {
            obj.transform.rotate(rotation);
        }
    }

    pub fn scale_object(&mut self, h: &PyObjectHandle, scalar: [f32; 3]) {
        if let Some(obj) = self.resolve_mut(h) {
            obj.transform.scale(scalar);
        }
    }

    pub fn get_object_matrix(&self, h: &PyObjectHandle) -> Option<[[f32; 4]; 4]> {
        if let Some(obj) = self.resolve(h) {
            Some(obj.transform.get_matrix())
        } else {
            None
        }

    }
}

impl Scene {
    fn resolve(&self, h: &PyObjectHandle) -> Option<&MeshObject> {
        if self.generations[h.index] == h.generation {
            self.objects[h.index].as_ref()
        } else {
            None
        }
    }

    fn resolve_mut(&mut self, h: &PyObjectHandle) -> Option<&mut MeshObject> {
        if self.generations[h.index] == h.generation {
            self.objects[h.index].as_mut()
        } else {
            None
        }
    }
}


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
