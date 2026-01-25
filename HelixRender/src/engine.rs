use crate::resource_manager;
use resource_manager::ResourceManager;

use crate::scene_manager;
use scene_manager::SceneManager;

use pyo3::prelude::*;

#[pyclass]
pub struct Engine {
    #[pyo3(get)]
    pub resource_manager: ResourceManager,
    #[pyo3(get)]
    pub scene_manager: SceneManager,
}

#[pymethods]
impl Engine {
    #[new]
    pub fn new(py: Python) -> Self {
        Engine {
            resource_manager: ResourceManager::new(),
            scene_manager: SceneManager::new(py),
        }
    }
}
