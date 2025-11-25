use pyo3::prelude::*;
use glam::Vec3;
use glam::Mat4;
use glam::Quat;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[pyclass]
pub struct Mesh {
    vertices: Vec<Vec3>,
    indices: Vec<usize>,
    polygons: Vec<Vec<usize>>,
    face_normals: Vec<Vec3>
}

#[pymethods]
impl Mesh {
    #[new]
    pub fn new() -> Self {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            polygons: Vec::new(),
            face_normals: Vec::new()
        }
    }

    pub fn load_obj(&mut self, path: &str)-> io::Result<()>  {
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
                    self.vertices.push(Vec3::new(x,y,z));
                },
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

                },
                _ => {}
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.polygons.clear();
        self.face_normals.clear();
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.polygons.len()
    }
}


#[pyclass]
pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

#[pymethods]
impl Transform {
    #[new]
    pub fn new() -> Self {
        Transform {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }

    pub fn get_matrix(&self) -> [[f32; 4]; 4] {
        let roll = Quat::from_rotation_x(self.rotation.x);
        let pitch = Quat::from_rotation_y(self.rotation.y);
        let yaw = Quat::from_rotation_z(self.rotation.z);

        let q = (yaw * pitch * roll).normalize();

        let transform_matrix = Mat4::from_rotation_translation(q, self.position) * Mat4::from_scale(self.scale);

        transform_matrix.to_cols_array_2d()
    }

    pub fn translate(&mut self, delta: [f32; 3]) {
        self.position += Vec3::from(delta);
    }

    pub fn rotate(&mut self, delta: [f32; 3]) {
        self.rotation += Vec3::from(delta);
    }

    pub fn scale(&mut self, s: [f32; 3]) {
        self.scale = Vec3::from(s);
    }
}
