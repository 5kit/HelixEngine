use glam::Mat4;
use glam::Quat;
use glam::Vec3;

use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
pub struct MeshObject {
    pub name: String,
    pub mesh_index: Option<usize>,
    pub transform: Transform,
}

impl MeshObject {
    pub fn new(name: String) -> Self {
        MeshObject {
            name,
            mesh_index: None,
            transform: Transform::new(),
        }
    }

    pub fn set_mesh(&mut self, mesh_index: usize) {
        self.mesh_index = Some(mesh_index);
    }

    // Transform setters

    pub fn set_scale(&mut self, s: [f32; 3]) {
        self.transform.scale = Vec3::from(s);
    }

    pub fn set_position(&mut self, p: [f32; 3]) {
        self.transform.position = Vec3::from(p);
    }

    pub fn set_rotation(&mut self, r: [f32; 3]) {
        self.transform.rotation = Vec3::from(r);
    }
}

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

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.polygons.len()
    }
}

#[derive(Clone)]
pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }

    pub fn get_matrix(&self) -> Mat4 {
        // get quaternions for roll, pitch and yaw
        let roll = Quat::from_rotation_x(self.rotation.x);
        let pitch = Quat::from_rotation_y(self.rotation.y);
        let yaw = Quat::from_rotation_z(self.rotation.z);

        // combine and normalize quaternion rotation
        let q = (yaw * pitch * roll).normalize();

        // convert quaternion into a 4x4 transformation matrix
        // add translation and scale
        let transform_matrix =
            Mat4::from_rotation_translation(q, self.position) * Mat4::from_scale(self.scale);

        // return as a 2D list
        transform_matrix
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.position += Vec3::from(delta);
    }

    pub fn rotate(&mut self, delta: Vec3) {
        self.rotation += Vec3::from(delta);
    }

    pub fn scale(&mut self, s: Vec3) {
        self.scale *= Vec3::from(s);
    }
}
