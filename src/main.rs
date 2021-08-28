// ============================================================================
//
// main.rs
//
// Purpose: The main entry point for the application.
//
// ============================================================================

extern crate gl;
extern crate sdl2;

use glam::*;
use imgui::{sys::*, FontConfig, FontSource};
use render::{gfx::*, shader::Shader};
use scene::orbitcamera::OrbitCamera;
use scene::{camera::Camera, scene::Scene};
use sdl2::sys::SDL_GL_SetAttribute;
use util::{input::INPUT, screen::update_screen, time::update_time};

use crate::render::mesh::Mesh;
use crate::scene::model::Model;

pub mod render;
pub mod scene;
pub mod util;

fn main() {
    crate::util::logger::init();

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

    let mut window = video_subsystem
        .window("", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();

    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
    let _viewport = gl::Viewport::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let mut imgui = imgui::Context::create();

    //
    // Imgui setup
    //
    let mut io = imgui.io_mut();
    io.display_size = [1280.0, 720.0];
    update_screen(IVec2::new(1280, 720));

    let font_data = include_bytes!("../content/fonts/Roboto-Regular.ttf");
    imgui.fonts().add_font(&[FontSource::TtfData {
        data: font_data,
        config: Some(FontConfig::default()),
        size_pixels: 14.0,
    }]);

    // let style = imgui.style_mut();
    imgui.style_mut().colors[ImGuiCol_Text as usize] = [1.00, 1.00, 1.00, 1.00];
    imgui.style_mut().colors[ImGuiCol_TextDisabled as usize] = [0.50, 0.50, 0.50, 1.00];
    imgui.style_mut().colors[ImGuiCol_WindowBg as usize] = [0.13, 0.14, 0.15, 1.00];
    imgui.style_mut().colors[ImGuiCol_ChildBg as usize] = [0.13, 0.14, 0.15, 1.00];
    imgui.style_mut().colors[ImGuiCol_PopupBg as usize] = [0.13, 0.14, 0.15, 1.00];
    imgui.style_mut().colors[ImGuiCol_Border as usize] = [0.43, 0.43, 0.50, 0.50];
    imgui.style_mut().colors[ImGuiCol_BorderShadow as usize] = [0.00, 0.00, 0.00, 0.00];
    imgui.style_mut().colors[ImGuiCol_FrameBg as usize] = [0.25, 0.25, 0.25, 1.00];
    imgui.style_mut().colors[ImGuiCol_FrameBgHovered as usize] = [0.38, 0.38, 0.38, 1.00];
    imgui.style_mut().colors[ImGuiCol_FrameBgActive as usize] = [0.67, 0.67, 0.67, 0.39];
    imgui.style_mut().colors[ImGuiCol_TitleBg as usize] = [0.08, 0.08, 0.09, 1.00];
    imgui.style_mut().colors[ImGuiCol_TitleBgActive as usize] = [0.08, 0.08, 0.09, 1.00];
    imgui.style_mut().colors[ImGuiCol_TitleBgCollapsed as usize] = [0.00, 0.00, 0.00, 0.51];
    imgui.style_mut().colors[ImGuiCol_MenuBarBg as usize] = [0.14, 0.14, 0.14, 1.00];
    imgui.style_mut().colors[ImGuiCol_ScrollbarBg as usize] = [0.02, 0.02, 0.02, 0.53];
    imgui.style_mut().colors[ImGuiCol_ScrollbarGrab as usize] = [0.31, 0.31, 0.31, 1.00];
    imgui.style_mut().colors[ImGuiCol_ScrollbarGrabHovered as usize] = [0.41, 0.41, 0.41, 1.00];
    imgui.style_mut().colors[ImGuiCol_ScrollbarGrabActive as usize] = [0.51, 0.51, 0.51, 1.00];
    imgui.style_mut().colors[ImGuiCol_CheckMark as usize] = [0.11, 0.64, 0.92, 1.00];
    imgui.style_mut().colors[ImGuiCol_SliderGrab as usize] = [0.11, 0.64, 0.92, 1.00];
    imgui.style_mut().colors[ImGuiCol_SliderGrabActive as usize] = [0.08, 0.50, 0.72, 1.00];
    imgui.style_mut().colors[ImGuiCol_Button as usize] = [0.25, 0.25, 0.25, 1.00];
    imgui.style_mut().colors[ImGuiCol_ButtonHovered as usize] = [0.38, 0.38, 0.38, 1.00];
    imgui.style_mut().colors[ImGuiCol_ButtonActive as usize] = [0.67, 0.67, 0.67, 0.39];
    imgui.style_mut().colors[ImGuiCol_Header as usize] = [0.22, 0.22, 0.22, 1.00];
    imgui.style_mut().colors[ImGuiCol_HeaderHovered as usize] = [0.25, 0.25, 0.25, 1.00];
    imgui.style_mut().colors[ImGuiCol_HeaderActive as usize] = [0.67, 0.67, 0.67, 0.39];
    imgui.style_mut().colors[ImGuiCol_Separator as usize] =
        imgui.style_mut().colors[ImGuiCol_Border as usize];
    imgui.style_mut().colors[ImGuiCol_SeparatorHovered as usize] = [0.41, 0.42, 0.44, 1.00];
    imgui.style_mut().colors[ImGuiCol_SeparatorActive as usize] = [0.26, 0.59, 0.98, 0.95];
    imgui.style_mut().colors[ImGuiCol_ResizeGrip as usize] = [0.00, 0.00, 0.00, 0.00];
    imgui.style_mut().colors[ImGuiCol_ResizeGripHovered as usize] = [0.29, 0.30, 0.31, 0.67];
    imgui.style_mut().colors[ImGuiCol_ResizeGripActive as usize] = [0.26, 0.59, 0.98, 0.95];
    imgui.style_mut().colors[ImGuiCol_Tab as usize] = [0.08, 0.08, 0.09, 0.83];
    imgui.style_mut().colors[ImGuiCol_TabHovered as usize] = [0.33, 0.34, 0.36, 0.83];
    imgui.style_mut().colors[ImGuiCol_TabActive as usize] = [0.23, 0.23, 0.24, 1.00];
    imgui.style_mut().colors[ImGuiCol_TabUnfocused as usize] = [0.08, 0.08, 0.09, 1.00];
    imgui.style_mut().colors[ImGuiCol_TabUnfocusedActive as usize] = [0.13, 0.14, 0.15, 1.00];
    imgui.style_mut().colors[ImGuiCol_PlotLines as usize] = [0.61, 0.61, 0.61, 1.00];
    imgui.style_mut().colors[ImGuiCol_PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
    imgui.style_mut().colors[ImGuiCol_PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.00];
    imgui.style_mut().colors[ImGuiCol_PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
    imgui.style_mut().colors[ImGuiCol_TextSelectedBg as usize] = [0.26, 0.59, 0.98, 0.35];
    imgui.style_mut().colors[ImGuiCol_DragDropTarget as usize] = [0.11, 0.64, 0.92, 1.00];
    imgui.style_mut().colors[ImGuiCol_NavHighlight as usize] = [0.26, 0.59, 0.98, 1.00];
    imgui.style_mut().colors[ImGuiCol_NavWindowingHighlight as usize] = [1.00, 1.00, 1.00, 0.70];
    imgui.style_mut().colors[ImGuiCol_NavWindowingDimBg as usize] = [0.80, 0.80, 0.80, 0.20];
    imgui.style_mut().colors[ImGuiCol_ModalWindowDimBg as usize] = [0.80, 0.80, 0.80, 0.35];
    imgui.style_mut().frame_rounding = 32f32;
    imgui.style_mut().grab_rounding = imgui.style_mut().frame_rounding;

    let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

    gfx_setup(&mut window);

    let mut camera: Camera = OrbitCamera::new();
    let mut shader = Shader::new("content/shaders/standard.glsl");
    shader.scan_uniforms();

    //
    // Scene setup
    //
    let scene = Scene::new("content/scene.json");
    let mut loaded_scene = scene.load();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut last_time = std::time::Instant::now();

    let mut cube = Model::new("content/models/cube.obj");
    cube.transform.scale = vec3(0.1, 0.1, 0.1);

    'main: loop {
        //
        // Input
        //
        {
            // Reset input
            unsafe {
                INPUT.mouse.delta = vec2(0.0, 0.0);
                INPUT.mouse.wheel = 0.0;
            }

            for event in event_pump.poll_iter() {
                imgui_sdl2.handle_event(&mut imgui, &event);
                if imgui_sdl2.ignore_event(&event) {
                    continue;
                }

                match event {
                    //
                    // Mouse
                    //
                    sdl2::event::Event::MouseMotion { x, y, .. } => unsafe {
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
                    sdl2::event::Event::Quit { .. } => break 'main,
                    sdl2::event::Event::Window { win_event, .. } => match win_event {
                        sdl2::event::WindowEvent::Resized(w, h) => {
                            gfx_resize(w, h);
                            update_screen(IVec2::new(w, h));
                        }
                        _ => {}
                    },

                    //
                    //
                    //
                    _ => {}
                }
            }
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());
        let ui = imgui.frame();

        //
        // Update
        //
        {
            loaded_scene.update();
            cube.transform.position = loaded_scene.light.position;
            camera.update(&ui);
        }

        //
        // Render
        //
        {
            gfx_clear();
            loaded_scene.draw_this(&mut shader, &mut camera);

            imgui_sdl2.prepare_render(&ui, &window);
            imgui_renderer.render(ui);

            cube.draw_this(&mut loaded_scene, &mut shader, &mut camera);

            window.gl_swap_window();
        }

        //
        // Timings
        //
        let mut delta = std::time::Instant::now()
            .duration_since(last_time)
            .as_millis() as f32;
        delta /= 1000.0;
        update_time(delta);

        last_time = std::time::Instant::now();
    }
}
