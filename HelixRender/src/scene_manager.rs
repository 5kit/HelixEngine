use crate::scene;
use scene::Scene;

use pyo3::prelude::*;

#[pyclass]
pub struct SceneManager {
    // Private Scene storage array (possible nulls)
    // Uses generations to prevent outdated handlers
    //Scenes: Vec<Option<Scene>>,
    //Scene_gen: Vec<u32>, // Same size as Scenes

    // Scene Handler for current active scene
    //pub active_scene: Option<SceneHandle>,
}

#[pymethods]
impl SceneManager {
    #[new]
    pub fn new() -> Self {
        SceneManager {
            // intialise TODO
        }
    }
}
