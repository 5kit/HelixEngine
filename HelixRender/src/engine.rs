use crate::resource_manager;
use resource_manager::ResourceManager;

use crate::scene_manager;
use scene_manager::SceneManager;

use pyo3::prelude::*;

#[pyclass]
pub struct Engine {
    pub resourceManager: ResourceManager,
    pub sceneManager: SceneManager,
}

#[pymethods]
impl Engine {
    #[new]
    pub fn new() -> Self {
        Engine {
            resourceManager: ResourceManager::new(),
            sceneManager: SceneManager::new(),
        }
    }
}
