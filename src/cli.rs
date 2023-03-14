use clap::{arg, value_parser, ArgGroup, Command};

use crate::wiggle;

pub fn new() -> Command {
    Command::new("wiggle")
        .version("0.2.0")
        .author("PineappleRind (https://github.com/PineappleRind)")
        .about("Generate wiggles")
        .arg(
            arg!(-t --height <HEIGHT>)
                .value_parser(value_parser!(u32))
                .default_value("7"),
        )
        .arg(
            arg!(-w --width <WIDTH>)
                .value_parser(value_parser!(u32))
                .default_value("14"),
        )
        .arg(arg!(-e --ease <EASE> "The name of an easing function. Cannot be used in combination with cubic_bezier").value_parser(wiggle::eases::ALL))
        .arg(arg!(-b --cubic_bezier <BEZIER_PARAMS> "4 comma-separated values. Cannot be used in combination with ease").allow_hyphen_values(true))
        .group(ArgGroup::new("ease_or_bezier").args(["ease", "cubic_bezier"]))
        .arg(arg!(-q --quiet "Quiet mode. Only outputs the wiggle and nothing else"))
        // .arg(arg!(-o --output "Write to a file"))
        .arg(arg!(<TEXT> "Wiggle text"))
        .arg_required_else_help(true)
}
