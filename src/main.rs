use std::env;

pub mod colors;
pub mod wiggle;

const USAGE: &'static str = "Usage: wiggle [width_int] [height_int] [text]";

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

    let wiggle = wiggle::generate(&text, width, height, ease.to_string());
    println!(
        "{} {} \n{}",
        colors::ansi_color("Generated wiggle!", vec![32, 4]),
        colors::ansi_color(&format!("({} characters)", wiggle.len()), vec![2]),
        wiggle
    );
}
