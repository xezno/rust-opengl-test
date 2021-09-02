// ============================================================================
//
// material.rs
//
// Purpose: Basic materials
//
// ============================================================================

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    pub diffuse: String,
    pub specular: f32,
}

impl Default for Material {
    fn default() -> Material {
        return Material {
            diffuse: "content/textures/missing.png".to_string(),
            specular: 0.0,
        };
    }
}
