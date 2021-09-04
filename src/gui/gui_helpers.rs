// ============================================================================
//
// scene_hierarchy.rs
//
// Purpose: Displays the scene hierarchy using imgui.
//
// ============================================================================

use gl::types::GLuint;
use glam::{Quat, Vec3};
use imgui::sys::*;
use imgui::*;

use crate::{scene::scene::LoadedScene, util::screen::get_screen};

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

pub fn gui_perf_overlay(ui: &Ui, frames_last_second: i32) {
    imgui::Window::new(imgui::im_str!("perfOverlay##hidelabel"))
        .flags(
            imgui::WindowFlags::NO_DECORATION
                | imgui::WindowFlags::NO_BACKGROUND
                | imgui::WindowFlags::NO_INPUTS,
        )
        .build(&ui, || {
            let draw_list = ui.get_background_draw_list();

            draw_list.add_text(
                [17.0, 17.0],
                0x44000000,
                im_str!("FPS: {:#?}", frames_last_second),
            ); // Shadow
            draw_list.add_text(
                [16.0, 16.0],
                0xFFFFFFFF,
                im_str!("FPS: {:#?}", frames_last_second),
            );

            unsafe {
                igSetWindowSizeStr(
                    im_str!("perfOverlay##hidelabel").as_ptr(),
                    ImVec2::new(0.0, 0.0),
                    0,
                );
            }
        });
}

pub fn gui_g_buffers(
    ui: &imgui::Ui,
    g_position: &GLuint,
    g_normal: &GLuint,
    g_color_spec: &GLuint,
) -> () {
    imgui::Window::new(imgui::im_str!("G-Buffers")).build(&ui, || {
        let mut size: ImVec2 = ImVec2::new(0.0, 0.0);
        unsafe {
            igSetNextItemWidth(-1.0);
            igGetContentRegionAvail(&mut size);
        }

        let screen_size = get_screen().size;
        let aspect = screen_size.y as f32 / screen_size.x as f32;
        size.y = size.x * aspect;

        let size_arr = [size.x, size.y];

        Image::new(TextureId::new(g_position.clone() as usize), size_arr)
            .uv0([0.0, 1.0])
            .uv1([1.0, 0.0])
            .build(&ui);

        Image::new(TextureId::new(g_normal.clone() as usize), size_arr)
            .uv0([0.0, 1.0])
            .uv1([1.0, 0.0])
            .build(&ui);

        Image::new(TextureId::new(g_color_spec.clone() as usize), size_arr)
            .uv0([0.0, 1.0])
            .uv1([1.0, 0.0])
            .build(&ui);
    });
}
