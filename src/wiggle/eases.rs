use std::f64::consts::PI;

pub fn linear(x: f64) -> f64 {
    x
}
pub fn sine(x: f64) -> f64 {
    -((x * PI).cos() - 1.0) / 2.0
}
pub fn quadratic(x: f64) -> f64 {
    if x < 0.5 {
        x * 2.0 * x
    } else {
        1.0 - ((-2.0 * x + 2.0).powf(2.0)) / 2.0
    }
}
pub fn cubic(x: f64) -> f64 {
    if x < 0.5 {
        4.0 * x * x
    } else {
        1.0 - ((-2.0 * x + 2.0).powf(3.0)) / 2.0
    }
}
pub fn exponential(x: f64) -> f64 {
    if x == 0.0 || (x - 1.0).abs() < f64::EPSILON {
        return x;
    }
    if x < 0.5 {
        (2.0_f64).powf(20.0 * x - 10.0) / 2.0
    } else {
        (2.0 - ((2.0_f64).powf(-20.0 * x + 10.0))) / 2.0
    }
}
pub fn quart_out(x: f64) -> f64 {
    1.0 - (1.0 - x).powf(4.0)
}
pub fn quart_in(x: f64) -> f64 {
    x * x * x * x
}

pub const ALL: [&str; 7] = [
    "linear",
    "sine",
    "quadratic",
    "cubic",
    "exponential",
    "quart_out",
    "quart_in",
];
