use clap::{arg, error::ErrorKind, Command};

use crate::wiggle;

fn parse_dimensions(arg: &str) -> Result<(u32, u32), clap::Error> {
    let split = arg.split_once('x');
    if let Some((width, height)) = split {
        let width = width
            .parse::<u32>()
            .map_err(|x| format!("error: {}", x))
            .unwrap();

        let height = height
            .parse::<u32>()
            .map_err(|x| format!("error: {}", x))
            .unwrap();
        Ok((width, height))
    } else {
        Err(clap::Error::new(ErrorKind::ValueValidation).with_cmd(&new()))
    }
}

fn parse_bezier_params(arg: &str) -> Result<Vec<f64>, clap::Error> {
    let split_vec = arg.split(',').collect::<Vec<&str>>();
    if !split_vec.len() == 4 {
        return Err(clap::Error::new(ErrorKind::ValueValidation).with_cmd(&new()));
    }
    let split_vec: Result<Vec<f64>, _> = split_vec.into_iter().map(str::parse::<f64>).collect();

    match split_vec {
        Ok(vec) => Ok(vec),
        Err(_) => Err(clap::Error::new(ErrorKind::ValueValidation)),
    }
}

pub fn new() -> Command {
    Command::new("wiggle")
        .version("0.2.0")
        .author("PineappleRind (https://github.com/PineappleRind)")
        .about("\x1b[36mGenerate text wiggles (for no good reason)!\x1b[0m")
        .color(clap::ColorChoice::Always)
        .arg(
            arg!(-b --bezier <CUBIC_BEZIER_PARAMS> "4 comma-separated values. Cannot be used in combination with --ease.")
            .allow_hyphen_values(true)
            .value_parser(clap::builder::ValueParser::new(parse_bezier_params))
            .conflicts_with("ease")
        )
        .arg(
            arg!(-d --dimensions <WIDTHxHEIGHT>)
                .default_value("14x7")
                .value_parser(clap::builder::ValueParser::new(parse_dimensions)),
        )
        .arg(arg!(-e --ease <EASE> "The name of an easing function. Cannot be used in combination with --bezier.").value_parser(wiggle::eases::ALL).conflicts_with("bezier"))
        .arg(arg!(-p --progress "Show progress. You should use this if you're generating a really big wiggle for some reason."))
        .arg(arg!(-q --quiet "Quiet mode. Doesn't output anything. Currently, for measuring performance"))
        .arg(arg!(-r --raw "Raw mode. Only outputs the wiggle, and nothing else. Overrides --progress."))
        .arg(arg!(<TEXT> "Wiggle text"))
        .arg_required_else_help(true)
}

pub const LOADER: &[&str] = &["◜", "◝", "◞", "◟", "\x1b[32m✓\x1b[0m"];
