// ============================================================================
//
// scene.rs
//
// Purpose: Basic scene serialization
//
// ============================================================================

use glam::Vec3;
use imgui::{im_str, ColorPicker, Condition, Ui, Window};
use log::{info, warn};
use serde_json::*;
use std::fs;

use super::{camera::Camera, model::Model, transform::Transform};
use crate::render::{material::Material, shader::Shader};

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
    pub path: Option<String>,
    pub transform: Transform,
    pub material: Option<Material>,
    pub phys: Option<String>,
}

pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
}

// This is what we use after we load the scene
pub struct LoadedScene {
    pub models: Vec<Model>,

    pub light: Light,
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
                    info!("Scene: loading model");
                    let mut model = Model::new(object.path.as_ref().unwrap().as_str());
                    model.transform = object.transform;
                    model.material = object.material.unwrap();
                    loaded_scene.models.push(model);

                    if object.phys.is_some() {
                        let phys_val = object.phys.as_ref().unwrap();

                        info!("Creating phys {}", phys_val);
                        match phys_val.as_str() {
                            "cuboid" => {}
                            "ball" => {}
                            _ => {
                                warn!("Unsupported phystype {}", phys_val);
                            }
                        }
                    }
                }
                "light" => {
                    info!("Scene: loading light");
                    loaded_scene.light.position = object.transform.position;
                }
                _ => {
                    warn!("Unsupported objtype {}", object.type_field);
                }
            }
        }

        return loaded_scene;
    }
}

impl LoadedScene {
    pub fn new() -> Self {
        LoadedScene {
            models: Vec::new(),
            light: Light {
                color: Vec3::new(1.0, 1.0, 1.0),
                position: Vec3::new(0.0, 0.0, 0.0),
            },
        }
    }

    pub fn draw_this(&self, shader: &mut Shader, camera: &mut Camera) {
        for object in &self.models {
            object.draw_this(&self, shader, camera);
        }
    }

    pub fn update(&mut self, ui: &Ui) {
        let time = crate::util::time::get_time().total;

        Window::new(im_str!("Lighting Debug"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(&ui, || {
                let mut color: [f32; 3] = self.light.color.into();
                let color_picker = ColorPicker::new(im_str!("Light Color"), &mut color);

                if color_picker.build(&ui) {
                    self.light.color = color.into();
                }
            });

        let position = Vec3::new(time.sin() * 2.0, time.cos() * 2.0, time.sin() * 2.0);
        self.light.position = position;
    }
}
