pub mod cli;
pub mod colors;
pub mod wiggle;

fn main() {
    let matches = cli::new().get_matches();

    let text = matches
        .get_one::<String>("TEXT")
        .expect("Text is required!");

    let width: u32 = *matches.get_one("width").expect("unreachable");
    let height: u32 = *matches.get_one("height").expect("unreachable");
    let ease: &str = match matches.get_one::<String>("ease") {
        Some(v) => {
            if wiggle::eases::ALL.contains(&v.as_str()) {
                v
            } else {
                "quadratic"
            }
        }
        None => "custom_bezier",
    };

    let quiet_mode: bool = matches.get_flag("quiet");

    let bezier_params = match matches.get_one::<String>("cubic_bezier") {
        Some(v) => {
            let split_vec = v.split(',');
            split_vec
                .into_iter()
                .map(|f| f.parse::<f64>().unwrap())
                .collect::<Vec<f64>>()
        }
        None => vec![0.6, 0.0, 0.4, 1.0],
    };

    print_info(
        &format!(
            "{}\n{} \"{}\"\n{} {}\n{} {}\n{} {}\n\n",
            colors::ansi_color("Generating wiggle...", &[35, 4]),
            colors::ansi_color("Text:", &[2]),
            text,
            colors::ansi_color("Width:", &[2]),
            width,
            colors::ansi_color("Height:", &[2]),
            height,
            colors::ansi_color("Ease:", &[2]),
            ease
        ),
        quiet_mode,
        None,
    );

    let wiggle = wiggle::generate(text, width, height, ease, &bezier_params);

    print_info(
        &format!(
            "{} {} \n",
            colors::ansi_color("Generated wiggle!", &[32, 4]),
            colors::ansi_color(&format!("({} characters)", wiggle.len()), &[2]),
            //wiggle
        ),
        quiet_mode,
        Some(wiggle),
    );
}

fn print_info(text: &str, quiet_mode: bool, alt_text: Option<String>) {
    if !quiet_mode {
        print!("{}", text);
        return;
    }
    if let Some(a) = alt_text {
        print!("{}", a);
    }
}
