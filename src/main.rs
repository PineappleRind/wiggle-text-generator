use std::env;

pub mod colors;
pub mod wiggle;

const USAGE: &'static str = "\n\nUsage: \x1b[36mwiggle \"text\" width height ease bezier_params\x1b[0m\n\x1b[2mSee docs for more information (\x1b[4mhttps://github.com/PineappleRind/wiggle-text-generator\x1b[0m\x1b[2m)\x1b[0m\n\n";

fn main() {
    let args: Vec<String> = env::args().collect();

    let text = args.get(1).expect(USAGE);
    // do I really have to expect for both arguments and invalid integers?
    let width = args
        .get(2)
        .expect(USAGE)
        .parse()
        .expect(&format!("Invalid integer! {}", USAGE));

    let height = args
        .get(3)
        .expect(USAGE)
        .parse()
        .expect(&format!("Invalid integer! {}", USAGE));

    let ease = match args.get(4) {
        Some(v) => v,
        None => "quadratic",
    };

    let bezier_params = match args.get(5) {
        Some(v) => {
            let split_vec = v.split(",").collect::<Vec<&str>>();
            split_vec
                .into_iter()
                .map(|f| f.parse::<f64>().unwrap())
                .collect()
        }
        None => vec![0.6, 0.0, 0.4, 1.0],
    };

    println!(
        "{}\n{} \"{}\"\n{} {}\n{} {}\n{} {}\n",
        colors::ansi_color("Generating wiggle...", vec![35, 4]),
        colors::ansi_color("Text:", vec![2]),
        text,
        colors::ansi_color("Width:", vec![2]),
        width,
        colors::ansi_color("Height:", vec![2]),
        height,
        colors::ansi_color("Ease:", vec![2]),
        ease
    );

    let wiggle = wiggle::generate(&text, width, height, ease.to_string(), bezier_params);
    println!(
        "{} {} \n{}",
        colors::ansi_color("Generated wiggle!", vec![32, 4]),
        colors::ansi_color(&format!("({} characters)", wiggle.len()), vec![2]),
        wiggle
    );
}
