// ============================================================================
//
// material.rs
//
// Purpose: Basic materials
//
// ============================================================================

use glam::Vec4;

#[derive(
    Default, Debug, Clone, Copy, PartialEq, serde_derive::Serialize, serde_derive::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    pub diffuse: Vec4,
}

impl Material {
    pub fn new() -> Material {
        Material { diffuse: Vec4::ONE }
    }
}
