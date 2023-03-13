pub mod bezier;
pub mod eases;

pub fn generate(
    text: &str,
    width: i32,
    height: i32,
    ease: String,
    bezier_params: Vec<f64>,
) -> String {
    let mut spaces: Vec<String> = vec![];
    for i in 0..height {
        // width of each row, between 0 and 1
        let row_width_normalized: f64 = ((i as f64) / (height as f64)).abs();
        let eased_normalized: f64 =
            find_and_ease(row_width_normalized, &ease, bezier_params.clone());
        let eased_row_width: i32 = (eased_normalized * width as f64).floor() as i32;

        let spaces_row = " ".repeat(eased_row_width.try_into().unwrap());
        spaces.push(spaces_row);
    }
    // append spaces' mirror
    let mut mirror: Vec<String> = spaces.clone();
    mirror.reverse();

    spaces = spaces.into_iter().chain(mirror).collect();

    let join_separator = format!("{}\n", text);
    return spaces.join(&join_separator);
}

fn find_and_ease(row_width: f64, ease: &str, bezier_params: Vec<f64>) -> f64 {
    match ease {
        "linear" => eases::linear(row_width),
        "sine" => eases::sine(row_width),
        "quadratic" => eases::quadratic(row_width),
        "cubic" => eases::cubic(row_width),
        "exponential" => eases::exponential(row_width),
        "quart_in" => eases::quart_in(row_width),
        "quart_out" => eases::quart_out(row_width),
        "custom_bezier" => bezier::calculate(row_width, bezier_params),
        &_ => eases::quadratic(row_width),
    }
}
