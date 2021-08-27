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

    distance: f32,
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

            distance: 5.0,
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

        let x = sin_time * self.distance;
        let y = cos_time * self.distance;

        self.set_position_calc_view_proj_mat(Vec3::new(x, y, 0.0));
    }

    fn calc_view_proj_mat(&mut self) {
        self.view_mat = Mat4::look_at_rh(self.position, Vec3::ZERO, Vec3::Z);
        self.proj_mat = Mat4::perspective_rh(
            self.fov.to_radians(),
            1280.0 / 720.0, // TODO: Get the screen size properly
            self.z_near,
            self.z_far,
        );

        self.proj_view_mat = self.proj_mat * self.view_mat;
    }
}
