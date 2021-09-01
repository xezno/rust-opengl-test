// ============================================================================
//
// material.rs
//
// Purpose: Basic materials
//
// ============================================================================

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    pub diffuse: String,
    pub specular: f32,
}

impl Material {
    pub fn new() -> Material {
        return Material {
            diffuse: "content/textures/missing.png".to_string(),
            specular: 0.0,
        };
    }
}
