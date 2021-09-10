// ============================================================================
//
// scene.rs
//
// Purpose: Basic scene serialization
//
// ============================================================================

use glam::{Quat, Vec3};
use imgui::{im_str, ColorEdit, Condition, Ui, Window};
use log::{info, warn};
use random_color::{Luminosity, RandomColor};
use serde_json::*;
use std::fs;

use super::{model::Model, transform::Transform};
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
    pub color: Option<Vec3>,
}

#[derive(Clone, Copy)]
pub struct SunLight {
    pub direction: Quat,
    pub color: Vec3,
}

#[derive(Clone, Copy)]
pub struct PointLight {
    pub transform: Transform,
    pub color: Vec3,

    pub orig_pos: Vec3,
}

// This is what we use after we load the scene
pub struct LoadedScene {
    pub models: Vec<Model>,
    pub point_lights: Vec<PointLight>,

    pub sun_light: SunLight,
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
                "light_sun" => {
                    info!("Scene: loading sun light");
                    loaded_scene.sun_light.direction = object.transform.rotation;
                    loaded_scene.sun_light.color =
                        object.color.unwrap_or(glam::vec3(1.0, 1.0, 1.0));
                }
                "light_point" => {
                    info!(
                        "Scene: loading point light at {}",
                        object.transform.position
                    );
                    loaded_scene.point_lights.push(PointLight {
                        transform: object.transform,
                        color: Vec3::new(1.0, 0.0, 1.0),
                        orig_pos: object.transform.position,
                    });
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
            sun_light: SunLight {
                color: Vec3::new(1.0, 1.0, 1.0),
                direction: Quat::IDENTITY,
            },
            point_lights: Vec::new(),
        }
    }

    pub fn render(&self, shader: &mut Shader, proj_view_mat: &glam::Mat4, cam_pos: &Vec3) {
        for object in &self.models {
            object.render(&self, shader, proj_view_mat, cam_pos);
        }
    }

    pub fn update(&mut self, ui: &Ui) {
        Window::new(im_str!("Lighting Debug"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(&ui, || {
                let mut color: [f32; 3] = self.sun_light.color.into();

                if ColorEdit::new(im_str!("Light Color"), &mut color).build(&ui) {
                    self.sun_light.color = color.into();
                }

                let mut direction: (f32, f32, f32) =
                    self.sun_light.direction.to_euler(glam::EulerRot::XYZ);
                direction.0 = direction.0.to_degrees();
                direction.1 = direction.1.to_degrees();
                direction.2 = direction.2.to_degrees();

                let mut direction_array = [direction.0, direction.1, direction.2];

                if ui
                    .input_float3(im_str!("Direction"), &mut direction_array)
                    .build()
                {
                    self.sun_light.direction = Quat::from_euler(
                        glam::EulerRot::XYZ,
                        direction_array[0].to_radians(),
                        direction_array[1].to_radians(),
                        direction_array[2].to_radians(),
                    );
                }
            });
    }
}
