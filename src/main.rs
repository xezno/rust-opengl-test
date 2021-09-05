// ============================================================================
//
// main.rs
//
// Purpose: The main entry point for the application.
//
// ============================================================================

extern crate gl;
extern crate sdl2;

use gl::types::GLuint;
use glam::*;
use gui::gui_helpers::gui_g_buffers;
use imgui::sys::ImGuiDockNodeFlags_PassthruCentralNode;

use render::{gfx::*, shader::Shader};
use renderdoc::{RenderDoc, V110};

use scene::orbitcamera::OrbitCamera;
use scene::{camera::Camera, scene::Scene};

use sdl2::sys::{SDL_GL_SetAttribute, SDL_GL_SetSwapInterval};
use util::{input::INPUT, screen::update_screen, time::update_time};

use crate::gui::gui_helpers::{gui_perf_overlay, gui_scene_hierarchy};

pub mod gui;
pub mod render;
pub mod scene;
pub mod util;

fn main() {
    {
        #[cfg(not(debug_timed))]
        pretty_env_logger::init();
    }
    let _rd: RenderDoc<V110> = RenderDoc::new().expect("Unable to connect");

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    unsafe {
        assert_eq!(
            0,
            SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_MULTISAMPLEBUFFERS, 1)
        );
        assert_eq!(
            0,
            SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_MULTISAMPLESAMPLES, 4)
        );
    }

    let win_size = uvec2(1600, 900);

    let mut window = video_subsystem
        .window("", win_size.x, win_size.y)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    unsafe {
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 4);
        SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 6);

        SDL_GL_SetAttribute(
            sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_PROFILE_MASK,
            sdl2::sys::SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_CORE as i32,
        );
    }

    update_screen(win_size.as_i32());

    let _gl_context = window.gl_create_context().unwrap();

    unsafe {
        assert_eq!(0, SDL_GL_SetSwapInterval(0));
    }

    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
    let _viewport = gl::Viewport::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let mut imgui = crate::util::imgui::imgui_init();
    let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

    gfx_setup(&mut window);
    //
    // Shadow buffer setup
    //
    let (mut shadow_buffer, mut shadow_texture) = gfx_setup_shadow_buffer();
    // let mut shadow_texture = 0;

    //
    // Gbuffer setup
    //
    let mut g_position: GLuint = 0;
    let mut g_normal: GLuint = 0;
    let mut g_color_spec: GLuint = 0;
    let mut g_orm: GLuint = 0;
    let mut g_buffer = gfx_setup_gbuffer(
        &mut g_position,
        &mut g_normal,
        &mut g_color_spec,
        &mut g_orm,
    );
    let mut gbuffer_shader = Shader::new("content/shaders/gbuffer.glsl");
    gbuffer_shader.scan_uniforms();
    let mut lighting_shader = Shader::new("content/shaders/lighting.glsl");
    lighting_shader.scan_uniforms();

    //
    // Scene setup
    //
    let scene = Scene::new("content/scene.json");
    let mut loaded_scene = scene.load();
    let mut camera: Camera = OrbitCamera::new();
    let quad_vao = gfx_quad_setup();

    //
    // Events
    //
    let mut event_pump = sdl.event_pump().unwrap();
    let mut last_render = std::time::Instant::now();

    //
    // Fps counter
    //
    let mut frames_last_second = 0;
    let mut last_fps_calc = std::time::Instant::now();
    let mut fps_counter = 0;

    //
    // Debug shape
    //
    let debug_model = crate::scene::model::Model::new("content/models/sphere.gltf");
    let mut debug_shader = Shader::new("content/shaders/gbuffer_light_debug.glsl");
    debug_shader.scan_uniforms();

    'main: loop {
        // Reset input
        unsafe {
            INPUT.mouse.delta = vec2(0.0, 0.0);
            INPUT.mouse.wheel = 0.0;
        }

        if !handle_input(
            &mut event_pump,
            &mut imgui,
            &mut imgui_sdl2,
            &mut g_buffer,
            &mut g_position,
            &mut g_normal,
            &mut g_color_spec,
            &mut g_orm,
            &mut shadow_texture,
        ) {
            break 'main;
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());
        let ui = imgui.frame();

        //
        // Update
        //
        {
            loaded_scene.update(&ui);
            camera.update(&ui);

            // DEBUG: Move lights around a bit
            for (_, point_light) in loaded_scene.point_lights.iter_mut().enumerate() {
                let speed = 8.0;
                let time = (util::time::get_time().total
                    + point_light.orig_pos.x
                    + point_light.orig_pos.y
                    + point_light.orig_pos.z)
                    * speed;

                let strength = 4.0;
                let offset = vec3(
                    time.sin() * strength,
                    time.cos() * strength,
                    time.sin() * 1.0,
                );
                point_light.transform.position = point_light.orig_pos + offset;
            }
        }

        //
        // Render
        //
        {
            let light_space_mat;
            // Shadow pass
            {
                // gfx_bind_framebuffer(shadow_buffer);
                gfx_prepare_shadow_pass(shadow_buffer);
                gfx_clear();

                let pos = vec3(0.0, 0.0, 100.0);
                let size = 50.0;
                let view_matrix = Mat4::IDENTITY
                    * Mat4::from_quat(loaded_scene.sun_light.direction)
                    * Mat4::from_translation(pos);
                let proj_matrix = Mat4::orthographic_lh(-size, size, -size, size, 0.1, 1000.0);
                light_space_mat = proj_matrix * view_matrix;

                loaded_scene.render(&mut gbuffer_shader, &light_space_mat, &pos);
            }

            // Geo pass
            {
                //gfx_bind_framebuffer(g_buffer);
                gfx_prepare_geometry_pass(g_buffer);
                gfx_clear();
                loaded_scene.render(&mut gbuffer_shader, &camera.proj_view_mat, &camera.position);

                // Draw debug
                {
                    debug_shader.bind();
                    debug_shader.set_mat4("uProjViewMat", &camera.proj_view_mat);
                    for (_, point_light) in loaded_scene.point_lights.iter().enumerate() {
                        debug_shader.set_vec3("uCamPos", &camera.position);

                        // Calc model matrix
                        let mut model_mat = Mat4::from_translation(point_light.transform.position);
                        model_mat *= Mat4::from_scale(vec3(0.1, 0.1, 0.1));
                        debug_shader.set_mat4("uModelMat", &model_mat);
                        debug_shader.set_vec3("vDebugLightCol", &point_light.color);
                        for mesh in &debug_model.meshes {
                            mesh.render();
                        }
                    }
                }
            }

            // Main lighting pass
            {
                // TODO: Proper skyboxes
                let mut sky_color = crate::render::color::col_from_hex("#6495ED");
                let mut scale = loaded_scene.sun_light.color.x
                    + loaded_scene.sun_light.color.y
                    + loaded_scene.sun_light.color.z;
                scale /= 3.0;

                sky_color.0 *= scale;
                sky_color.1 *= scale;
                sky_color.2 *= scale;

                gfx_prepare_lighting_pass(&sky_color);
                gfx_bind_framebuffer(0);
                gfx_clear();

                // Bind lighting pass shader
                lighting_shader.bind();

                unsafe {
                    // Bind gbuffer textures
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, g_position);
                    gl::ActiveTexture(gl::TEXTURE1);
                    gl::BindTexture(gl::TEXTURE_2D, g_normal);
                    gl::ActiveTexture(gl::TEXTURE2);
                    gl::BindTexture(gl::TEXTURE_2D, g_color_spec);
                    gl::ActiveTexture(gl::TEXTURE3);
                    gl::BindTexture(gl::TEXTURE_2D, g_orm);
                    gl::ActiveTexture(gl::TEXTURE4);
                    gl::BindTexture(gl::TEXTURE_2D, shadow_texture);
                }

                lighting_shader.set_i32("gPosition", 0);
                lighting_shader.set_i32("gNormal", 1);
                lighting_shader.set_i32("gColorSpec", 2);
                lighting_shader.set_i32("gOrm", 3);
                lighting_shader.set_i32("sShadowMap", 4);

                // Submit scene uniforms
                lighting_shader.set_mat4("uProjViewMat", &camera.proj_view_mat);
                lighting_shader.set_vec3("uCamPos", &camera.position);
                lighting_shader.set_mat4("uLightSpaceMat", &light_space_mat);

                // Set lighting uniforms
                lighting_shader.set_vec3(
                    "lightingInfo.vLightDir",
                    &loaded_scene
                        .sun_light
                        .direction
                        .to_euler(EulerRot::XYZ)
                        .into(),
                );
                lighting_shader.set_vec3("lightingInfo.vLightColor", &loaded_scene.sun_light.color);
                lighting_shader.set_vec3(
                    "lightingInfo.vFogColor",
                    &Vec3::new(sky_color.0, sky_color.1, sky_color.2),
                );

                // Submit scene point lighting
                lighting_shader.set_i32(
                    "lightingInfo.iPointLightCount",
                    loaded_scene.point_lights.len() as i32,
                );

                for (i, point_light) in loaded_scene.point_lights.iter().enumerate() {
                    lighting_shader.set_vec3(
                        format!("pointLights[{}].vPos", i).as_str(),
                        &point_light.transform.position,
                    );
                    lighting_shader.set_vec3(
                        format!("pointLights[{}].vColor", i).as_str(),
                        &point_light.color,
                    );
                }

                // Render quad
                gfx_quad_render(quad_vao);
            }

            // Draw imgui
            {
                gfx_prepare_imgui_pass();
                imgui_sdl2.prepare_render(&ui, &window);

                // Dock space
                unsafe {
                    imgui::sys::igDockSpaceOverViewport(
                        imgui::sys::igGetMainViewport(),
                        ImGuiDockNodeFlags_PassthruCentralNode as i32,
                        ::std::ptr::null::<imgui::sys::ImGuiWindowClass>(),
                    );
                }

                gui_scene_hierarchy(&ui, &mut loaded_scene);
                gui_perf_overlay(&ui, frames_last_second);
                gui_g_buffers(
                    &ui,
                    &g_position,
                    &g_normal,
                    &g_color_spec,
                    &g_orm,
                    &shadow_texture,
                );

                imgui_renderer.render(ui);
            }

            fps_counter += 1;
            window.gl_swap_window();
        }

        //
        // Timings
        //
        {
            let mut delta = std::time::Instant::now()
                .duration_since(last_render)
                .as_millis() as f64;
            delta /= 1000f64;
            update_time(delta);

            last_render = std::time::Instant::now();

            if std::time::Instant::now()
                .duration_since(last_fps_calc)
                .as_secs()
                >= 1
            {
                last_fps_calc = std::time::Instant::now();
                frames_last_second = fps_counter;
                fps_counter = 0;
            }
        }
    }
}

fn handle_input(
    event_pump: &mut sdl2::EventPump,
    imgui: &mut imgui::Context,
    imgui_sdl2: &mut imgui_sdl2::ImguiSdl2,

    g_buffer: &mut GLuint,
    g_position: &mut GLuint,
    g_normal: &mut GLuint,
    g_color_spec: &mut GLuint,
    g_orm: &mut GLuint,
    shadow_texture: &mut GLuint,
) -> bool {
    for event in event_pump.poll_iter() {
        imgui_sdl2.handle_event(imgui, &event);
        if imgui_sdl2.ignore_event(&event) {
            continue;
        }

        match event {
            //
            // Mouse
            //
            sdl2::event::Event::MouseMotion { x, y, .. } => unsafe {
                // TODO: Work out why this shits the bed when you move the mouse quickly

                let delta = vec2(
                    (x - INPUT.mouse.position.x) as f32,
                    (y - INPUT.mouse.position.y) as f32,
                );

                INPUT.mouse.delta = delta;
                INPUT.mouse.position = IVec2::new(x, y);
            },
            sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => unsafe {
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => INPUT.mouse.left = true,
                    sdl2::mouse::MouseButton::Right => INPUT.mouse.right = true,
                    _ => {}
                }
            },
            sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => unsafe {
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => INPUT.mouse.left = false,
                    sdl2::mouse::MouseButton::Right => INPUT.mouse.right = false,
                    _ => {}
                }
            },

            sdl2::event::Event::MouseWheel { y, .. } => unsafe {
                INPUT.mouse.wheel = y as f32;
            },

            //
            // Window
            //
            sdl2::event::Event::Quit { .. } => return false,
            sdl2::event::Event::Window { win_event, .. } => match win_event {
                sdl2::event::WindowEvent::SizeChanged(w, h) => {
                    gfx_resize(w, h);
                    update_screen(IVec2::new(w, h));

                    // HACK? Resize gbuffers
                    *g_buffer = gfx_setup_gbuffer(g_position, g_normal, g_color_spec, g_orm);
                }
                _ => {}
            },
            _ => {}
        }
    }
    return true;
}
