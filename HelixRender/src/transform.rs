use glam::{Mat4, Quat, Vec3};

use crate::camera::PyCameraHandle;
use crate::general_handler::Handle;
use crate::mesh::PyMeshHandle;
use crate::mesh_object::PyMeshObjectHandle;
use crate::scene::Scene;

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy)]
pub struct PyTransformNodeHandle {
    pub handle: Handle,
}

// handle for any Transfor Object
#[pyclass]
#[derive(Clone, Copy)]
pub struct PyTransformObjectHandle {
    pub identity: TransformType,
    pub handle: Handle,
}

// identify any objects with a transform
#[derive(Clone, Copy)]
pub enum TransformType {
    MeshObject,
    Camera,
    // add more later (light, collider, ect...)
}

// Node belongs to a Object (1 - 1)
// Which can have parent (1 - 1) or children (1 - 0..*) nodes
// An objects Transform is stored SOLELY in its repersetitive node
#[derive(Clone)]
pub struct TransformNode {
    // Local Object Transform
    pub local: Transform,

    // Parent and child Handlers
    pub parent: Option<PyTransformNodeHandle>,
    pub children: Vec<PyTransformNodeHandle>,

    // Pre-computed world Transform from parent
    pub world: Mat4,
    pub dirty: bool, // Update flag
}

impl TransformNode {
    // Initialise With parent (minimum root)
    pub fn new(parent: Option<PyTransformNodeHandle>) -> Self {
        TransformNode {
            local: Transform::new(),

            parent,
            children: Vec::new(),

            world: Mat4::ZERO,
            dirty: true,
        }
    }

    pub fn add_child(&mut self, new_child: PyTransformNodeHandle) {
        self.children.push(new_child);
    }
}

// Transform DataType for transformation matrix operations
#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
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

        transform_matrix.transpose()
    }

    pub fn translate(&mut self, delta: [f32; 3]) {
        self.position += Vec3::from(delta);
    }

    pub fn rotate(&mut self, delta: [f32; 3]) {
        self.rotation += Vec3::from(delta);
    }

    pub fn scale(&mut self, s: [f32; 3]) {
        self.scale *= Vec3::from(s);
    }
}
