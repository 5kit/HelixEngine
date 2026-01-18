mod camera;
mod object;
mod scene;

use object::{Mesh, MeshObject, Transform};
use pyo3::prelude::*;
use scene::Scene;

#[pymodule]
fn helix_render(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Scene>()?;
    Ok(())
}
