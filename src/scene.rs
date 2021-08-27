// ============================================================================
//
// scene.rs
//
// Purpose: Basic scene serialization
//
// ============================================================================

use std::fs;

use crate::{camera::Camera, model::Model, shader::Shader, transform::Transform};
use serde_json::*;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub objects: Vec<Object>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub path: String,
    pub transform: Transform,
}

impl std::ops::Deref for Object {
    type Target = Transform;

    fn deref(&self) -> &Self::Target {
        &self.transform
    }
}

// This is what we use after we load the scene
pub struct LoadedScene {
    pub models: Vec<Model>,
}

impl Scene {
    pub fn new(scene_path: &str) -> Self {
        let scene_raw_data = fs::read_to_string(scene_path).unwrap();

        let scene = from_str::<Scene>(&scene_raw_data).unwrap();
        return scene;
    }

    pub fn load(&self) -> LoadedScene {
        let mut loaded_scene = LoadedScene::new();

        for object in &self.objects {
            match object.type_field.as_str() {
                "model" => {
                    // Load model
                    let mut model = Model::new(object.path.as_str());
                    model.transform = object.transform;
                    loaded_scene.models.push(model);
                }
                _ => {
                    panic!("Unsupported object type {}", object.type_field);
                }
            }
        }

        return loaded_scene;
    }
}

impl LoadedScene {
    pub fn new() -> Self {
        LoadedScene { models: Vec::new() }
    }
    pub fn draw_this(&self, shader: &mut Shader, camera: &mut Camera) {
        for object in &self.models {
            object.draw_this(shader, camera);
        }
    }
}
