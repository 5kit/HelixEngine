mod scene;

use pyo3::prelude::*;
use scene::Scene;

#[pymodule]
fn helix_render(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Scene>()?;
    Ok(())
}