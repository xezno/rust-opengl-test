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

    fn calc_view_proj_mat(&mut self) {
        // Calculate view / projection mats
        let forward = Vec3::X;
        self.view_mat = Mat4::look_at_rh(self.position, Vec3::ZERO, Vec3::Z);
        self.proj_mat = Mat4::perspective_rh(
            self.fov.to_radians(),
            1280.0 / 720.0,
            self.z_near,
            self.z_far,
        );

        self.proj_view_mat = self.proj_mat * self.view_mat;

        // println!("View matrix: {}", self.view_mat);
        // println!("Proj matrix: {}", self.proj_mat);
        // println!("Proj view matrix: {}", self.proj_view_mat);
    }
}
