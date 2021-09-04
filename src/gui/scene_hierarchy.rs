// ============================================================================
//
// scene_hierarchy.rs
//
// Purpose: Displays the scene hierarchy using imgui.
//
// ============================================================================

use glam::{Quat, Vec3};
use imgui::*;

use crate::scene::scene::LoadedScene;

pub fn gui_scene_hierarchy(ui: &Ui, scene: &mut LoadedScene) {
    let mut opened = true;

    Window::new(im_str!("Scene Hierarchy"))
        .opened(&mut opened)
        .build(ui, || {
            ui.text(im_str!("Scene Hierarchy"));
            ui.separator();

            for (i, point_light) in scene.point_lights.iter_mut().enumerate() {
                imgui::TreeNode::new(&imgui::ImString::new(format!("Point Light {:?}", i))).build(
                    &ui,
                    || {
                        let mut position = point_light.transform.position.to_array();
                        if ui
                            .input_float3(im_str!("Point light {} pos", i).as_ref(), &mut position)
                            .build()
                        {
                            point_light.transform.position =
                                Vec3::new(position[0], position[1], position[2]);
                        }

                        let mut color: [f32; 3] = point_light.color.into();

                        if imgui::ColorEdit::new(
                            im_str!("Point light {} color", i).as_ref(),
                            &mut color,
                        )
                        .build(&ui)
                        {
                            point_light.color = color.into();
                        }
                    },
                );
            }

            for (i, model) in scene.models.iter_mut().enumerate() {
                imgui::TreeNode::new(&imgui::ImString::new(format!("Model {:?}", i))).build(
                    &ui,
                    || {
                        let mut position = model.transform.position.to_array();
                        if ui
                            .input_float3(im_str!("Model {} pos", i).as_ref(), &mut position)
                            .build()
                        {
                            model.transform.position =
                                Vec3::new(position[0], position[1], position[2]);
                        }

                        let rotation = model.transform.rotation.to_euler(glam::EulerRot::XYZ);
                        let mut rot_array = [rotation.0, rotation.1, rotation.2];
                        if ui
                            .input_float3(im_str!("Model {} rot", i).as_ref(), &mut rot_array)
                            .build()
                        {
                            model.transform.rotation = Quat::from_euler(
                                glam::EulerRot::XYZ,
                                rot_array[0],
                                rot_array[1],
                                rot_array[2],
                            );
                        }

                        let mut scale = model.transform.scale.to_array();
                        if ui
                            .input_float3(im_str!("Model {} scale", i).as_ref(), &mut scale)
                            .build()
                        {
                            model.transform.scale = Vec3::new(scale[0], scale[1], scale[2]);
                        }
                    },
                );
            }
        });
}
