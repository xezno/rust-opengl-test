// ============================================================================
//
// time.rs
//
// Purpose: Keeps track of time.
//
// ============================================================================

#[derive(Copy, Clone)]
pub struct Time {
    pub total: f32,
    pub delta: f32,
    pub delta_64: f64,
}

pub static mut TIME: Time = Time {
    total: 0.0,
    delta: 0.0,
    delta_64: 0.0,
};

pub fn get_time() -> Time {
    unsafe {
        return TIME;
    }
}

pub fn update_time(dt: f64) {
    unsafe {
        TIME.delta_64 = dt;

        TIME.delta = dt as f32;
        TIME.total += dt as f32;
    }
}
