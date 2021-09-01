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

    pub euler_rot: Vec3,

    pub fov: f32,
    pub(super) wish_fov: f32,

    pub(super) look_at: Vec3,

    pub z_near: f32,
    pub z_far: f32,

    pub view_mat: Mat4,
    pub proj_mat: Mat4,

    pub proj_view_mat: Mat4,

    pub(super) wish_orbit_distance: f32,
    pub(super) orbit_distance: f32,
}
