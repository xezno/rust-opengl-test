// ============================================================================
//
// input.rs
//
// Purpose:
//
// ============================================================================

use glam::{IVec2, Vec2};

#[derive(Copy, Clone)]
pub struct Mouse {
    pub delta: Vec2,
    pub position: IVec2,

    pub left: bool,
    pub right: bool,

    pub wheel: f32,
}

#[derive(Copy, Clone)]
pub struct Input {
    pub mouse: Mouse,
}

pub static mut INPUT: Input = Input {
    mouse: Mouse {
        delta: Vec2::ZERO,
        position: IVec2::ZERO,

        left: false,
        right: false,

        wheel: 0.0,
    },
};
