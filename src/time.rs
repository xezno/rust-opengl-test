// ============================================================================
//
// time.rs
//
// Purpose: Keeps track of time.
//
// ============================================================================

pub struct Time {
    pub total: f32,
    pub delta: f32,
}

impl Clone for Time {
    fn clone(&self) -> Self {
        Self {
            total: self.total.clone(),
            delta: self.delta.clone(),
        }
    }
}
impl Copy for Time {}

static mut TIME: Time = Time {
    total: 0.0,
    delta: 0.0,
};

pub fn get_time() -> Time {
    unsafe {
        return TIME;
    }
}

pub fn update_time(dt: f32) {
    unsafe {
        TIME.delta = dt;
        TIME.total += dt;
    }
}
