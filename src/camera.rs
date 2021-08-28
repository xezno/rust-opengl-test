// ============================================================================
//
// camera.rs
//
// Purpose: Scene camera
//
// ============================================================================

use glam::*;

pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,

    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,

    pub view_mat: Mat4,
    pub proj_mat: Mat4,

    pub proj_view_mat: Mat4,

    orbit_distance: f32,
    height: f32,
}

impl Camera {
    pub fn new() -> Camera {
        // Initial camera values
        let mut cam = Camera {
            position: Vec3::new(0.0, -1.0, 0.0),
            rotation: Quat::IDENTITY,

            fov: 45.0,
            z_near: 0.01,
            z_far: 100.0,

            view_mat: Mat4::IDENTITY,
            proj_mat: Mat4::IDENTITY,

            proj_view_mat: Mat4::IDENTITY,

            orbit_distance: 5.0,
            height: 2.0,
        };

        cam.calc_view_proj_mat();

        return cam;
    }

    pub fn set_position_calc_view_proj_mat(&mut self, pos: Vec3) {
        self.position = pos;
        self.calc_view_proj_mat();
    }

    pub fn set_rotation_calc_view_proj_mat(&mut self, rot: Quat) {
        self.rotation = rot;
        self.calc_view_proj_mat();
    }

    pub fn update(&mut self) {
        // sine
        let time = crate::time::get_time();
        let sin_time = (time.total).sin();
        let cos_time = (time.total).cos();

        let x = sin_time * self.orbit_distance;
        let y = cos_time * self.orbit_distance;

        self.set_position_calc_view_proj_mat(Vec3::new(x, y, self.height));
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
        let screen_size = crate::screen::get_screen().size;
        let aspect_ratio = (screen_size.x as f32) / (screen_size.y as f32);

        self.view_mat = Mat4::look_at_rh(self.position, Vec3::ZERO, Vec3::Z);
        self.proj_mat =
            Camera::create_perspective_reversed_z(self.fov.to_radians(), aspect_ratio, self.z_near);
        // Mat4::perspective_rh(self.fov.to_radians(), aspect_ratio, self.z_near, self.z_far);

        self.proj_view_mat = self.proj_mat * self.view_mat;
    }
}
