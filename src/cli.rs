use clap::{arg, error::ErrorKind, Command};

use crate::wiggle;

fn parse_dimensions(arg: &str) -> Result<(u32, u32), clap::Error> {
    let split = arg.split_once("x");
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

pub fn new() -> Command {
    Command::new("wiggle")
        .version("0.2.0")
        .author("PineappleRind (https://github.com/PineappleRind)")
        .about("\x1b[36mGenerate text wiggles (for no reason)!\x1b[0m")
        .color(clap::ColorChoice::Always)
        .arg(
            arg!(-d --dimensions <WIDTHxHEIGHT>)
                .default_value("14x7")
                .value_parser(clap::builder::ValueParser::new(parse_dimensions)),
        )
        .arg(arg!(-e --ease <EASE> "The name of an easing function. Cannot be used in combination with cubic_bezier").value_parser(wiggle::eases::ALL).conflicts_with("cubic_bezier"))
        .arg(arg!(-b --cubic_bezier <BEZIER_PARAMS> "4 comma-separated values. Cannot be used in combination with ease").allow_hyphen_values(true).conflicts_with("ease"))
        .arg(arg!(-q --quiet "Quiet mode. Only outputs the wiggle and nothing else"))
        // .arg(arg!(-o --output "Write to a file"))
        .arg(arg!(<TEXT> "Wiggle text"))
        .arg_required_else_help(true)
}
