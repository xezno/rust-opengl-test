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
use imgui::sys::{
    igGetContentRegionAvail, igSetNextItemWidth, igSetWindowSizeStr,
    ImGuiDockNodeFlags_PassthruCentralNode, ImVec2,
};
use imgui::{im_str, Image, TextureId};
use render::{gfx::*, shader::Shader};
use renderdoc::{RenderDoc, V110};
use scene::orbitcamera::OrbitCamera;
use scene::{camera::Camera, scene::Scene};
use sdl2::sys::SDL_GL_SetAttribute;
use util::{input::INPUT, screen::update_screen, time::update_time};

use crate::gui::scene_hierarchy::gui_scene_hierarchy;
use crate::util::screen::get_screen;

pub mod gui;
pub mod render;
pub mod scene;
pub mod util;

fn main() {
    {
        #[cfg(debug_timed)]
        pretty_env_logger::init_timed();
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

    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
    let _viewport = gl::Viewport::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let mut imgui = crate::util::imgui::imgui_init();
    let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

    gfx_setup(&mut window);

    //
    // Gbuf setup
    //
    let mut g_position: GLuint = 0;
    let mut g_normal: GLuint = 0;
    let mut g_color_spec: GLuint = 0;
    let mut g_buffer = gfx_setup_gbuffer(&mut g_position, &mut g_normal, &mut g_color_spec);

    let mut camera: Camera = OrbitCamera::new();
    let mut gbuffer_shader = Shader::new("content/shaders/gbuffer.glsl");
    gbuffer_shader.scan_uniforms();
    let mut lighting_shader = Shader::new("content/shaders/lighting.glsl");
    lighting_shader.scan_uniforms();

    //
    // Scene setup
    //
    let scene = Scene::new("content/scene.json");
    let mut loaded_scene = scene.load();
    let quad_vao = gfx_quad_setup();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut last_time = std::time::Instant::now();

    let mut frames_last_second = 0;
    let mut fps_calc_time = std::time::Instant::now();
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
            // Geo pass
            {
                unsafe {
                    gl::Enable(gl::DEPTH_TEST);
                    let attachments = [
                        gl::COLOR_ATTACHMENT0,
                        gl::COLOR_ATTACHMENT1,
                        gl::COLOR_ATTACHMENT2,
                    ];
                    gl::ClearColor(0.0, 0.0, 0.0, 0.0);
                    gl::DrawBuffers(3, &attachments[0]);
                }
                gfx_bind_framebuffer(g_buffer);
                gfx_clear();
                loaded_scene.render(&mut gbuffer_shader, &mut camera);

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
                let mut scale = loaded_scene.light.color.x
                    + loaded_scene.light.color.y
                    + loaded_scene.light.color.z;
                scale /= 3.0;

                sky_color.0 *= scale;
                sky_color.1 *= scale;
                sky_color.2 *= scale;

                unsafe {
                    gl::Disable(gl::DEPTH_TEST);
                    gl::ClearColor(sky_color.0, sky_color.1, sky_color.2, 1.0);
                }
                gfx_bind_framebuffer(0);
                gfx_clear();

                unsafe {
                    gl::Enable(gl::FRAMEBUFFER_SRGB);

                    // Bind lighting pass shader
                    lighting_shader.bind();

                    // Bind gbuffer textures
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, g_position);
                    gl::ActiveTexture(gl::TEXTURE1);
                    gl::BindTexture(gl::TEXTURE_2D, g_normal);
                    gl::ActiveTexture(gl::TEXTURE2);
                    gl::BindTexture(gl::TEXTURE_2D, g_color_spec);

                    lighting_shader.set_i32("gPosition", 0);
                    lighting_shader.set_i32("gNormal", 1);
                    lighting_shader.set_i32("gColorSpec", 2);

                    // Submit scene uniforms
                    lighting_shader.set_mat4("uProjViewMat", &camera.proj_view_mat);
                    lighting_shader.set_vec3("uCamPos", &camera.position);

                    // Set lighting uniforms
                    lighting_shader.set_vec3(
                        "lightingInfo.vLightDir",
                        &loaded_scene.light.direction.to_euler(EulerRot::XYZ).into(),
                    );
                    lighting_shader.set_vec3("lightingInfo.vLightColor", &loaded_scene.light.color);
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

                    gl::Disable(gl::FRAMEBUFFER_SRGB);
                }
            }

            // Draw imgui
            {
                imgui_sdl2.prepare_render(&ui, &window);

                // Dock space
                unsafe {
                    imgui::sys::igDockSpaceOverViewport(
                        imgui::sys::igGetMainViewport(),
                        ImGuiDockNodeFlags_PassthruCentralNode as i32,
                        ::std::ptr::null::<imgui::sys::ImGuiWindowClass>(),
                    );
                }

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

                    Image::new(TextureId::new(g_position as usize), size_arr)
                        .uv0([0.0, 1.0])
                        .uv1([1.0, 0.0])
                        .build(&ui);

                    Image::new(TextureId::new(g_normal as usize), size_arr)
                        .uv0([0.0, 1.0])
                        .uv1([1.0, 0.0])
                        .build(&ui);

                    Image::new(TextureId::new(g_color_spec as usize), size_arr)
                        .uv0([0.0, 1.0])
                        .uv1([1.0, 0.0])
                        .build(&ui);
                });

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

                gui_scene_hierarchy(&ui, &mut loaded_scene);

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
                .duration_since(last_time)
                .as_millis() as f64;
            delta /= 1000f64;
            update_time(delta);

            last_time = std::time::Instant::now();

            if std::time::Instant::now()
                .duration_since(fps_calc_time)
                .as_secs()
                >= 1
            {
                fps_calc_time = std::time::Instant::now();
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
                    *g_buffer = gfx_setup_gbuffer(g_position, g_normal, g_color_spec);
                }
                _ => {}
            },

            //
            //
            //
            _ => {}
        }
    }
    return true;
}
