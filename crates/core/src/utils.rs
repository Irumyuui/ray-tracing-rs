#[inline(always)]
pub const fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * core::f32::consts::PI / 180.0
}

#[inline(always)]
pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
