// ============================================================================
//
// screen.rs
//
// Purpose:
//
// ============================================================================
use glam::IVec2;

#[derive(Clone, Copy)]
pub struct Screen {
    pub size: IVec2,
}

static mut SCREEN: Screen = Screen { size: IVec2::ZERO };

pub fn get_screen() -> Screen {
    unsafe {
        return SCREEN;
    }
}

pub fn update_screen(size: IVec2) {
    unsafe {
        SCREEN.size = size;
    }
}
