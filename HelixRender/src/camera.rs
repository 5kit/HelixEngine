use crate::object;

use pyo3::prelude::*;
use object::Transform;

#[pyclass]
pub struct Camera {
    pub name: String,
    pub transform: Transform,

    pub fov: f32,       // field of view
    pub aspect: f32,    // width / height
    pub near: f32,      // 
    pub far: f32,
    
    pub background_color: [f32; 3],
}

#[pymethods]
impl Camera {
    #[new]
    pub fn new(name: String) -> Self {
        Camera {
            name,
            transform: Transform::new(),
            fov: std::f32::consts::FRAC_PI_4,  // 45 degrees
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
            background_color: [1.0, 1.0, 1.0]
        }
    }
}