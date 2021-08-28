// ============================================================================
//
// orbitcamera.rs
//
// Purpose: Orbit camera.
//
// ============================================================================

use super::camera::Camera;

use glam::*;
use imgui::*;

pub trait OrbitCamera {
    fn new() -> Self;
    fn set_position_calc_view_proj_mat(&mut self, pos: Vec3);
    fn set_rotation_calc_view_proj_mat(&mut self, rot: Quat);
    fn update(&mut self, ui: &Ui);
    fn rotate(&mut self, ui: &Ui);
    fn move_lookat(&mut self, ui: &Ui);
    fn calc_view_proj_mat(&mut self);
    fn create_perspective_reversed_z(fov_radians: f32, aspect_ratio: f32, z_near: f32) -> Mat4;
}

use crate::util::{input::INPUT, lerp::Lerp, screen::get_screen, time::TIME};

impl OrbitCamera for Camera {
    fn new() -> Self {
        // Initial camera values
        let mut cam = Camera {
            position: Vec3::new(0.0, -1.0, 0.0),
            rotation: Quat::IDENTITY,

            euler_rot: Vec3::new(0.0, 0.0, 0.0),

            fov: 60.0,
            wish_fov: 60.0,

            look_at: Vec3::ZERO,

            z_near: 0.01,
            z_far: 100.0,

            view_mat: Mat4::IDENTITY,
            proj_mat: Mat4::IDENTITY,

            proj_view_mat: Mat4::IDENTITY,

            wish_orbit_distance: 5.0,
            orbit_distance: 5.0,
        };

        cam.calc_view_proj_mat();

        return cam;
    }

    fn set_position_calc_view_proj_mat(&mut self, pos: Vec3) {
        self.position = pos;
        self.calc_view_proj_mat();
    }

    fn set_rotation_calc_view_proj_mat(&mut self, rot: Quat) {
        self.rotation = rot;
        self.calc_view_proj_mat();
    }

    fn update(&mut self, ui: &Ui) {
        self.rotate(&ui);
        self.move_lookat(&ui);

        let yaw = self.euler_rot.x.to_radians();
        let pitch = self.euler_rot.y.to_radians();

        unsafe {
            Window::new(im_str!("Orbit camera Debug"))
                .size([300.0, 110.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(format!("Mouse pos: {}", INPUT.mouse.position));
                    ui.text(format!("Mouse delta: {}", INPUT.mouse.delta));

                    // ui.text(format!("Camera pos: {:.1}", self.position));
                    // ui.text(format!(
                    //     "Camera rot euler: {}",
                    //     serde_json::to_string(&self.eulerRot).unwrap()
                    // ));

                    ui.text(format!("Camera fov: {:.1}", self.fov));
                    ui.text(format!("Camera wish fov: {:.1}", self.wish_fov));

                    ui.text(format!("Look at: {}", self.look_at));

                    ui.text(format!("Pitch, yaw: {:.1} {:.1}", pitch, yaw));

                    if ui.button(im_str!("Reset look at"), [0.0, 0.0]) {
                        self.look_at = Vec3::new(0.0, -1.0, 0.0);
                    }
                });
        }

        unsafe {
            // Set position
            self.wish_fov -= INPUT.mouse.wheel * 5.0;
            self.wish_fov = self.wish_fov.clamp(50f32, 110f32);

            self.fov = self.fov.lerp(self.wish_fov, TIME.delta * 10.0);
            self.fov = self.fov.clamp(50f32, 110f32);

            self.wish_orbit_distance -= INPUT.mouse.wheel * 2.0;
            self.wish_orbit_distance = self.wish_orbit_distance.clamp(4.0, 5.0);

            self.orbit_distance = self
                .orbit_distance
                .lerp(self.wish_orbit_distance, TIME.delta * 10.0);
            self.orbit_distance = self.orbit_distance.clamp(4.0, 5.0);
        }

        self.position = self.look_at
            + vec3(
                yaw.sin() * pitch.cos() * self.orbit_distance,
                yaw.cos() * pitch.cos() * self.orbit_distance,
                pitch.sin() * self.orbit_distance,
            );

        self.set_position_calc_view_proj_mat(self.position);
    }

    fn move_lookat(&mut self, ui: &Ui) {
        let forward = vec3(
            self.euler_rot.x.to_radians().sin() * self.euler_rot.y.to_radians().cos(),
            self.euler_rot.x.to_radians().cos() * self.euler_rot.y.to_radians().cos(),
            self.euler_rot.y.to_radians().sin(),
        );

        let up = vec3(
            self.euler_rot.x.to_radians().sin() * self.euler_rot.y.to_radians().sin(),
            self.euler_rot.x.to_radians().cos() * self.euler_rot.y.to_radians().sin(),
            self.euler_rot.y.to_radians().cos(),
        );

        let right = forward.cross(up);

        unsafe {
            if INPUT.mouse.right {
                ui.set_mouse_cursor(Some(MouseCursor::Hand));

                self.look_at += up * INPUT.mouse.delta.y * 0.005;
                self.look_at += right * INPUT.mouse.delta.x * 0.005;
            }
        }
    }

    fn rotate(&mut self, ui: &Ui) {
        unsafe {
            if INPUT.mouse.left {
                ui.set_mouse_cursor(Some(MouseCursor::Hand));

                self.euler_rot.x += INPUT.mouse.delta.x * 0.25;
                self.euler_rot.y += INPUT.mouse.delta.y * 0.25;
            } else {
                self.euler_rot.y = self.euler_rot.y.lerp(0.0, 10.0 * TIME.delta);
            }
        }

        self.euler_rot.y %= 360.0;
        self.euler_rot.y = self.euler_rot.y.clamp(-89f32, 89f32);
    }

    // TODO: Floating point depth buffer
    fn create_perspective_reversed_z(fov_radians: f32, aspect_ratio: f32, z_near: f32) -> Mat4 {
        let f = 1.0 / (fov_radians / 2.0).tan();
        return Mat4::from_cols(
            Vec4::new(f / aspect_ratio, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, -1.0),
            Vec4::new(0.0, 0.0, z_near, 0.0),
        );
    }

    fn calc_view_proj_mat(&mut self) {
        let screen_size = get_screen().size;
        let aspect_ratio = (screen_size.x as f32) / (screen_size.y as f32);

        self.view_mat = Mat4::look_at_rh(self.position, self.look_at, Vec3::Z);
        self.proj_mat =
            Camera::create_perspective_reversed_z(self.fov.to_radians(), aspect_ratio, self.z_near);
        // Mat4::perspective_rh(self.fov.to_radians(), aspect_ratio, self.z_near, self.z_far);

        self.proj_view_mat = self.proj_mat * self.view_mat;
    }
}
