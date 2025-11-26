mod scene;
mod object;

use pyo3::prelude::*;
use scene::Scene;
use object::{MeshObject, Transform, Mesh};

#[pymodule]
fn helix_render(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Scene>()?;
    m.add_class::<MeshObject>()?;
    m.add_class::<Transform>()?;
    m.add_class::<Mesh>()?;
    Ok(())
}