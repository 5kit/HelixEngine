use std::path;

use crate::mesh;
use mesh::{Mesh, MeshHandle};

use pyo3::prelude::*;

#[pyclass]
pub struct ResourceManager {
    meshes: Vec<Option<Mesh>>, // Meshes belong to the Manager
    mesh_gen: Vec<u32>,        // Mesh gen
}

// Python Exposed Methods
#[pymethods]
impl ResourceManager {
    #[new]
    pub fn new() -> Self {
        ResourceManager {
            meshes: Vec::new(),
            mesh_gen: Vec::new(),
        }
    }

    // Loads a new obj Mesh form
    pub fn load_mesh(&mut self, path: String) -> PyResult<MeshHandle> {
        let mut mesh = Mesh::new();
        mesh.load_obj(&path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        // Search for empty slot:
        for i in 0..self.meshes.len() {
            if self.meshes[i].is_none() {
                // set slot to mesh and increment generation
                self.meshes[i] = Some(mesh.clone());
                self.mesh_gen[i] += 1;
                return Ok(MeshHandle::new(i, self.mesh_gen[i]));
            }
        }

        // If no empty slots push new slot
        self.meshes.push(Some(mesh));
        self.mesh_gen.push(0);

        return Ok(MeshHandle::new(self.meshes.len() - 1, 0));
    }
}

// Handler Management
impl ResourceManager {
    // Mesh handle resolvers
    fn resolve_mesh(&self, h: &MeshHandle) -> Option<&Mesh> {
        if self.mesh_gen[h.index] == h.generation {
            self.meshes[h.index].as_ref()
        } else {
            None
        }
    }

    fn resolve_mesh_mut(&mut self, h: &MeshHandle) -> Option<&mut Mesh> {
        if self.mesh_gen[h.index] == h.generation {
            self.meshes[h.index].as_mut()
        } else {
            None
        }
    }
}
