use glam::{Mat4, Quat, Vec3};
use std::fs::File;
use std::io::{self, BufRead};

use crate::general_handler::Handle;

use pyo3::prelude::*;

// Pythoon exposed handle wrapper for handle type safety
#[pyclass]
#[derive(Clone)]
pub struct PyMeshHandle {
    pub handle: Handle,
}

// Mesh Object
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    indices: Vec<usize>,
    polygons: Vec<Vec<usize>>,
    face_normals: Vec<Vec3>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            polygons: Vec::new(),
            face_normals: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.polygons.clear();
        self.face_normals.clear();
    }

    // load obj data from provided obj file
    pub fn load_obj(&mut self, path: &str) -> io::Result<()> {
        self.clear();

        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            match parts[0] {
                "v" => {
                    // Vertex
                    let x: f32 = parts[1].parse().unwrap();
                    let y: f32 = parts[2].parse().unwrap();
                    let z: f32 = parts[3].parse().unwrap();
                    self.vertices.push(Vec3::new(x, y, z));
                }
                "f" => {
                    // Face
                    let mut face = Vec::new();
                    for vertex in &parts[1..] {
                        let v_index = vertex.split('/').next().unwrap();
                        let v: usize = v_index.parse::<usize>().unwrap();
                        face.push(v - 1); // obj uses starts index at 1
                    }
                    self.polygons.push(face.clone());

                    // Compute normals assuming at least 3 sides
                    let edge1 = self.vertices[face[1]] - self.vertices[face[0]];
                    let edge2 = self.vertices[face[2]] - self.vertices[face[0]];
                    let normal = edge1.cross(edge2).normalize();
                    self.face_normals.push(normal);
                }
                _ => {}
            }
        }

        Ok(())
    }

    // compile -> convert to vertices

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.polygons.len()
    }
}
