// ============================================================================
//
// main.rs
//
// Purpose: The main entry point for the application.
//
// ============================================================================

extern crate gl;
extern crate sdl2;

use std::ffi::c_void;
use std::ptr;

use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint};
use glam::*;
use imgui::sys::{igGetContentRegionAvail, igImage, igSetNextItemWidth, ImVec2, ImVec4};
use render::{gfx::*, shader::Shader};
use scene::orbitcamera::OrbitCamera;
use scene::{camera::Camera, scene::Scene};
use sdl2::sys::SDL_GL_SetAttribute;
use util::{input::INPUT, screen::update_screen, time::update_time};

pub mod render;
pub mod scene;
pub mod util;

// ( VAO, VBO )
fn quad_setup() -> (GLuint, GLuint) {
    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;

    #[rustfmt::skip]
    let quad_verts: [f32; 20] = [
        -1.0,  1.0, 0.0, 0.0, 1.0,
        -1.0, -1.0, 0.0, 0.0, 0.0,
        1.0,  1.0, 0.0, 1.0, 1.0,
        1.0, -1.0, 0.0, 1.0, 0.0,
    ];

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (quad_verts.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            &quad_verts[0] as *const GLfloat as *const c_void,
            gl::STATIC_DRAW,
        );
        let stride = (5 * std::mem::size_of::<GLfloat>()) as GLsizei;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<GLfloat>()) as *const c_void,
        );
    }

    return (vao, vbo);
}

unsafe fn quad_render(vao: GLuint) {
    gl::BindVertexArray(vao);
    gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
    gl::BindVertexArray(0);
}

fn main() {
    crate::util::logger::init().expect("Wasn't able to start logger");

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
    let g_buffer = gfx_setup_gbuffer(&mut g_position, &mut g_normal, &mut g_color_spec);

    let mut camera: Camera = OrbitCamera::new();
    let mut gbuffer_shader = Shader::new("content/shaders/gbuffer.glsl");
    gbuffer_shader.scan_uniforms();
    let mut lighting_shader = Shader::new("content/shaders/lighting.glsl");
    lighting_shader.scan_uniforms();

    //
    // Quad
    //
    let (quad_vao, quad_vbo) = quad_setup();

    //
    // Scene setup
    //
    let scene = Scene::new("content/scene.json");
    let mut loaded_scene = scene.load();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut last_time = std::time::Instant::now();

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
                    sdl2::event::Event::Quit { .. } => break 'main,
                    sdl2::event::Event::Window { win_event, .. } => match win_event {
                        // sdl2::event::WindowEvent::Resized(w, h) => {
                        // }
                        sdl2::event::WindowEvent::SizeChanged(w, h) => {
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
            loaded_scene.update(&ui);
            camera.update(&ui);
        }

        //
        // Render
        //
        {
            // Geo pass
            {
                unsafe {
                    // gl::ClearDepth(0.0);
                    gl::Enable(gl::DEPTH_TEST);
                    // gl::DepthFunc(gl::GREATER);
                    let attachments = [
                        gl::COLOR_ATTACHMENT0,
                        gl::COLOR_ATTACHMENT1,
                        gl::COLOR_ATTACHMENT2,
                    ];
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::DrawBuffers(3, &attachments[0]);
                }
                gfx_bind_framebuffer(g_buffer);
                gfx_clear();
                // Bind gbuffer shader
                loaded_scene.draw_this(&mut gbuffer_shader, &mut camera);
            }

            // Main render pass
            {
                unsafe {
                    gl::Disable(gl::DEPTH_TEST);

                    // Cornflower blue as hex
                    let col = crate::render::color::from_hex("#6495ED");
                    gl::ClearColor(col.0, col.1, col.2, 1.0);
                }
                gfx_bind_framebuffer(0);
                gfx_clear();
                // gfx_resize(get_screen().size.x, get_screen().size.y);

                unsafe {
                    gl::Enable(gl::FRAMEBUFFER_SRGB);

                    // Bind lighting pass shader
                    lighting_shader.use_this();

                    // Bind gbuffer textures
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, g_position);
                    gl::ActiveTexture(gl::TEXTURE1);
                    gl::BindTexture(gl::TEXTURE_2D, g_normal);
                    gl::ActiveTexture(gl::TEXTURE2);
                    gl::BindTexture(gl::TEXTURE_2D, g_color_spec);

                    lighting_shader.set_int("gPosition", 0);
                    lighting_shader.set_int("gNormal", 1);
                    lighting_shader.set_int("gColorSpec", 2);

                    // Submit scene uniforms
                    lighting_shader.set_mat4("uProjViewMat", &camera.proj_view_mat);
                    lighting_shader.set_vec3("uCamPos", &camera.position);

                    // Set lighting uniforms
                    lighting_shader.set_vec3(
                        "lightingInfo.vLightDir",
                        &loaded_scene.light.direction.to_euler(EulerRot::XYZ).into(),
                    );
                    lighting_shader.set_vec3("lightingInfo.vLightColor", &loaded_scene.light.color);

                    // Render quad
                    quad_render(quad_vao);

                    gl::Disable(gl::FRAMEBUFFER_SRGB);
                }
            }

            // Draw imgui
            {
                imgui_sdl2.prepare_render(&ui, &window);

                unsafe {
                    igSetNextItemWidth(-1.0);
                    let mut size: ImVec2 = ImVec2::new(0.0, 0.0);
                    igGetContentRegionAvail(&mut size);

                    let aspect = size.y / 900.0;
                    size.y = size.x * aspect;

                    igImage(
                        g_position as *mut c_void,
                        size,
                        ImVec2::new(0.0, 1.0),
                        ImVec2::new(1.0, 0.0),
                        ImVec4::new(1.0, 1.0, 1.0, 1.0),
                        ImVec4::new(1.0, 1.0, 1.0, 1.0),
                    );

                    igImage(
                        g_normal as *mut c_void,
                        size,
                        ImVec2::new(0.0, 1.0),
                        ImVec2::new(1.0, 0.0),
                        ImVec4::new(1.0, 1.0, 1.0, 1.0),
                        ImVec4::new(1.0, 1.0, 1.0, 1.0),
                    );

                    igImage(
                        g_color_spec as *mut c_void,
                        size,
                        ImVec2::new(0.0, 1.0),
                        ImVec2::new(1.0, 0.0),
                        ImVec4::new(1.0, 1.0, 1.0, 1.0),
                        ImVec4::new(1.0, 1.0, 1.0, 1.0),
                    );
                }
                imgui_renderer.render(ui);
            }
            window.gl_swap_window();
        }

        //
        // Timings
        //
        {
            let mut delta = std::time::Instant::now()
                .duration_since(last_time)
                .as_millis() as f32;
            delta /= 1000.0;
            update_time(delta);

            last_time = std::time::Instant::now();
        }
    }
}
