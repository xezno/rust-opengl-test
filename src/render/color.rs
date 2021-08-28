// ============================================================================
//
// color.rs
//
// Purpose: Color conversion helpers
//
// ============================================================================

pub fn from_rgb(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    return (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
}

pub fn from_hex(hex: &str) -> (f32, f32, f32) {
    let hex = hex.trim_start_matches("#");
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    return from_rgb(r, g, b);
}
