// ============================================================================
//
// lerp.rs
//
// Purpose: Extension trait for lerp
//
// ============================================================================

pub trait Lerp<T> {
    fn lerp(&self, other: T, t: f32) -> T;
}

impl Lerp<f32> for f32 {
    fn lerp(&self, other: f32, t: f32) -> f32 {
        *self * (1.0 - t) + other * t
    }
}
