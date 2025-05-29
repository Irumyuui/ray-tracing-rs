#[inline(always)]
pub const fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * core::f32::consts::PI / 180.0
}
