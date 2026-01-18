use glam::Mat4;
use glam::Vec4;

use crate::transform;
use transform::TransformNodeHandle;

use pyo3::prelude::*;

// Camera Object Handler for Scene
#[pyclass]
pub struct CameraHandle {
    index: usize,
    generation: u32,
}

#[pymethods]
impl CameraHandle {
    #[new]
    pub fn new(index: usize, generation: u32) -> Self {
        Self { index, generation }
    }
}

#[derive(Clone)]
pub struct Camera {
    pub transform_node_handle: Option<TransformNodeHandle>,

    pub fov: f32,    // field of view
    pub aspect: f32, // width / height
    pub near: f32,   // cam->screen
    pub far: f32,    // cam->maxdistance

    pub background_color: [f32; 3],
}

impl Camera {
    pub fn new() -> Self {
        Self {}
        //Camera {
        //    transform_node_handle: TransformNodeHandle::new(),
        //    fov: std::f32::consts::FRAC_PI_4, // 45 degrees
        //    aspect: 16.0 / 9.0,               // if problem, try flip
        //    near: 0.1,
        //    far: 1000.0,
        //    background_color: [1.0, 1.0, 1.0],
        //}
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        let f = 1.0 / (self.fov * 0.5).tan();

        let a = (self.far + self.near) / (self.near - self.far);
        let b = (2.0 * self.far * self.near) / (self.near - self.far);

        Mat4::from_cols(
            Vec4::new(f / self.aspect, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, a, -1.0),
            Vec4::new(0.0, 0.0, b, 0.0),
        )
    }

    //pub fn get_view_matrix(&self) -> Mat4 {
    //    self.transform.get_matrix().inverse()
    //}
}
