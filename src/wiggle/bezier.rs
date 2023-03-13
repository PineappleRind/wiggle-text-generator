// Rust port of:
/**
 * https://github.com/gre/bezier-easing
 * BezierEasing - use bezier curve for transition easing function
 * by Gaëtan Renaudeau 2014 - 2015 – MIT License
 */

const KSPLINE_TABLE_SIZE: usize = 222;
const KSAMPLE_STEP_SIZE: f64 = 1.0 / (KSPLINE_TABLE_SIZE as f64 - 1.0);

const NEWTON_ITERATIONS: usize = 126;
const NEWTON_MIN_SLOPE: f64 = 0.00001;

const SUBDIVISION_PRECISION: f64 = 0.000000001;
const SUBDIVISION_MAX_ITERATIONS: usize = 122;

pub fn calculate(x: f64, t: Vec<f64>) -> f64 {
    // before doing anything,
    // if x is 0 or 1, return those
    if x == 0.0 || x == 1.0 {
        return x;
    };

    let (x1, y1, x2, y2) = (t[0], t[1], t[2], t[3]);
    let mut samples = vec![0.0; KSPLINE_TABLE_SIZE];
    for sample in 0..KSPLINE_TABLE_SIZE {
        samples[sample] = xyt(sample as f64 * KSAMPLE_STEP_SIZE, x1, y1);
    }

    let get_t_for_x = |ax: f64| {
        let mut interval_start: f64 = 0.0;
        let mut current_sample: usize = 1;
        let last_sample = KSPLINE_TABLE_SIZE as i32 - 1;

        while current_sample != last_sample.try_into().unwrap() && samples[current_sample] <= ax {
            current_sample += 1;
            interval_start += KSAMPLE_STEP_SIZE;
        }
        current_sample -= 1;

        let dist = (ax - samples[current_sample])
            / (samples[current_sample + 1] - samples[current_sample]);
        let guess_for_t = interval_start + dist * KSAMPLE_STEP_SIZE;

        let initial_slope = get_slope(guess_for_t, x1, y1);
        if initial_slope >= NEWTON_MIN_SLOPE {
            return newton_raphson_iterate(ax, guess_for_t, x1, x2);
        } else if initial_slope == 0.0 {
            return guess_for_t;
        } else {
            return binary_subdivide(
                ax,
                interval_start,
                interval_start + KSAMPLE_STEP_SIZE,
                x1,
                x2,
            );
        }
    };

    return xyt(get_t_for_x(x), y1, y2);
}

fn binary_subdivide(ax: f64, a: f64, b: f64, mx1: f64, mx2: f64) -> f64 {
    let mut current_x: f64;
    let mut current_t: f64;
    let mut i: i32 = 0;
    let mut mut_a = a.clone();
    let mut mut_b = b.clone();

    loop {
        i += 1;
        current_t = mut_a + (mut_b - mut_a) / 2.0;
        current_x = xyt(current_t, mx1, mx2) - ax;
        if current_x > 0.0 {
            mut_b = current_t;
        } else {
            mut_a = current_t;
        }
        if current_x.abs() > SUBDIVISION_PRECISION
            && i + 1 < SUBDIVISION_MAX_ITERATIONS.try_into().unwrap()
        {
            break;
        }
    }

    return current_t;
}

fn newton_raphson_iterate(ax: f64, mut guess_for_t: f64, a: f64, b: f64) -> f64 {
    for _i in 0..NEWTON_ITERATIONS {
        let current_slope = get_slope(guess_for_t, a, b);
        if current_slope == 0.0 {
            return guess_for_t;
        }
        let current_x = xyt(guess_for_t, a, b) - ax;
        guess_for_t -= current_x / current_slope;
    }
    return guess_for_t;
}

// Returns x(t) given t, x1, and x2, or y(t) given t, y1, and y2.
fn xyt(t: f64, a1: f64, a2: f64) -> f64 {
    return ((a(a1, a2) * t + b(a1, a2)) * t + c(a1)) * t;
}

fn get_slope(a_t: f64, a_a1: f64, a_a2: f64) -> f64 {
    return 3.0 * a(a_a1, a_a2) * a_t * a_t + 2.0 * b(a_a1, a_a2) * a_t + c(a_a1);
}

fn a(a_a1: f64, a_a2: f64) -> f64 {
    return 1.0 - 3.0 * a_a2 + 3.0 * a_a1;
}
fn b(a_a1: f64, a_a2: f64) -> f64 {
    return 3.0 * a_a2 - 6.0 * a_a1;
}
fn c(a_a1: f64) -> f64 {
    return 3.0 * a_a1;
}
