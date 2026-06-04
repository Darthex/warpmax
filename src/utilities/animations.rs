const OVERSHOOT: f32 = 1.70158;

pub fn ease_in_back(t: f32, s: Option<f32>) -> f32 {
    let s = s.unwrap_or(OVERSHOOT);
    t * t * ((s + 1.0) * t - s)
}

pub fn ease_out_back(t: f32, s: Option<f32>) -> f32 {
    let s = s.unwrap_or(OVERSHOOT);
    let u = t - 1.0;
    u * u * ((s + 1.0) * u + s) + 1.0
}
