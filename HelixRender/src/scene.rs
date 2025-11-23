use pyo3::prelude::*;

#[pyclass]
pub struct Scene {
    #[pyo3(get, set)]
    pub background_color: [f32; 3],
    // other
}


#[pymethods]
impl Scene {
    #[new]
    pub fn new() -> Self {
        Scene {
            background_color: [1.0, 1.0, 1.0],
        }
    }
}