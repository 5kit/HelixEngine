mod camera;
mod engine;
mod general_handler;
mod mesh;
mod mesh_object;
mod resource_manager;
mod scene;
mod scene_manager;
mod transform;

use engine::Engine;

use pyo3::prelude::*;

#[pymodule]
fn helix_render(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Engine>()?;
    Ok(())
}

/*
 * ToDO:
 * - implment handlers: mesh/, meshObj/, camera, transform/
 * - UPDATE Docs
 * - implement TransformNode /
 * - implement Scene /
 * - implment MeshObject, mesh /
 * - implment parent child global transform
 * - implment python interface -
 * - implment camera -
 * - implment OpenGL GLSL rasterisation
 */
