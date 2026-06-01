const OVERSHOOT: f32 = 1.70158;

pub fn ease_in_out_back(t: f32, s: Option<f32>) -> f32 {
    let s = s.unwrap_or(OVERSHOOT) * 1.525; // standard tweak for in-out
    if t < 0.5 {
        (t * 2.).powi(2) * ((s + 1.) * t * 2. - s) / 2.
    } else {
        ((t * 2.) - 2.).powi(2) * ((s + 1.) * (t * 2. - 2.) + s) / 2. + 1.
    }
}

pub fn ease_in_back(t: f32, s: Option<f32>) -> f32 {
    let s = s.unwrap_or(OVERSHOOT);
    t * t * ((s + 1.0) * t - s)
}
