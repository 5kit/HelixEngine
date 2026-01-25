use crate::general_handler::ObjectStorage;
use crate::scene::Scene;

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct SceneManager {
    // Private Python owned Scene array
    scenes: Vec<Py<Scene>>,

    // Scene Handler for current active scene
    #[pyo3(get)]
    pub active_scene: Py<Scene>,
}

#[pymethods]
impl SceneManager {
    #[new]
    pub fn new(py: Python) -> Self {
        let mut scene_vec = Vec::new();
        let py_scene: Py<Scene> = Py::new(py, Scene::new()).unwrap();
        scene_vec.push(py_scene.clone());
        SceneManager {
            scenes: scene_vec,
            active_scene: py_scene,
        }
    }
}
