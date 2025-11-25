mod scene;
mod object;

use pyo3::prelude::*;
use scene::Scene;
use object::Transform;

#[pymodule]
fn helix_render(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Scene>()?;
    m.add_class::<Transform>()?;
    Ok(())
}