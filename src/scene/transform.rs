// ============================================================================
//
// transform.rs
//
// Purpose:
//
// ============================================================================

use glam::*;

#[derive(
    Default, Debug, Clone, Copy, PartialEq, serde_derive::Serialize, serde_derive::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Transform {
        Transform {
            position,
            rotation,
            scale,
        }
    }

    pub const IDENTITY: Self = Transform {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };
}
