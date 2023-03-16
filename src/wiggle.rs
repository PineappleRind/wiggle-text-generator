use std::process::exit;

pub mod bezier;
pub mod eases;

pub fn generate(text: &str, dimensions: (u32, u32), ease: &str, bezier_params: &[f64]) -> String {
    let mut spaces: Vec<String> = vec![];
    let mut offset: usize = 0;
    let (width, height) = dimensions;

    // Make height half of what the user specified, as we'll be appending the wiggle's reverse later.
    // Note that this loses precision, so a height of 7 and a height of 8 will both end up being 8.
    let height = (height as f64 / 2.0).round() as u32;

    for i in 0..height {
        // width of each row, between 0 and 1; goes up linearly with i
        let row_width_normalized: f64 = (f64::from(i) / f64::from(height)).abs();
        // between 0 and 1; goes up with i, but eased with the user's specified ease function
        let mut eased_normalized: f64 = find_and_ease(row_width_normalized, ease, bezier_params);
        // Move entire wiggle right by eased_normalized if eased_normalized is negative
        // you cannot have negative spaces!
        if eased_normalized.is_sign_negative() {
            offset += eased_normalized.abs().ceil() as usize;
            eased_normalized = 0.0;
            spaces = spaces
                .into_iter()
                .map(|x| format!("{}{}", " ".repeat(offset), x))
                .collect();
        }
        // between 0 and width, width in columns
        let eased_row_width: usize =
            match usize::try_from((eased_normalized * f64::from(width)).round() as i32).ok() {
                Some(width) => width,
                None => exit(1), // This should be unreachable
            };

        let spaces_row = format!("{}{}", " ".repeat(eased_row_width), text);
        spaces.push(spaces_row);
    }
    // append spaces' mirror
    let mut mirror: Vec<String> = spaces.clone();
    mirror.reverse();

    spaces = spaces.into_iter().chain(mirror).collect();

    spaces.join("\n")
}

fn find_and_ease(row_width: f64, ease: &str, bezier_params: &[f64]) -> f64 {
    match ease {
        "linear" => eases::linear(row_width),
        "sine" => eases::sine(row_width),
        "cubic" => eases::cubic(row_width),
        "exponential" => eases::exponential(row_width),
        "quart_in" => eases::quart_in(row_width),
        "quart_out" => eases::quart_out(row_width),
        "custom_bezier" => bezier::calculate(row_width, bezier_params),
        &_ => eases::quadratic(row_width),
    }
}
