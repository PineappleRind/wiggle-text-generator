use indicatif::{ProgressBar, ProgressStyle};
use std::{process::exit, sync::mpsc, thread, time::Duration};

pub mod cli;
pub mod colors;
pub mod wiggle;

fn main() {
    let matches = cli::new().get_matches();

    let text = matches
        .get_one::<String>("TEXT")
        .expect("Text is required!")
        .clone();

    let dimensions: (u32, u32) = *matches.get_one("dimensions").expect("No dimensions");

    let bezier_params: Vec<f64> = (*matches
        .get_one::<Vec<f64>>("bezier")
        .or(Some(&vec![]))
        .expect("Unreachable"))
    .clone();

    let ease: String = match matches.get_one::<String>("ease") {
        Some(v) => v,
        None => {
            if bezier_params.is_empty() {
                "quadratic"
            } else {
                "custom_bezier"
            }
        }
    }
    .to_owned();

    let raw_mode: bool = matches.get_flag("raw");
    let quiet_mode: bool = matches.get_flag("quiet");
    let show_progress: bool = matches.get_flag("progress");

    if !raw_mode {
        println!(
            "{}\n{} \"{}\"\n{} {}\n{} {}\n{} {}",
            colors::ansi_color("Generating wiggle...", &[35, 4]),
            colors::ansi_color("Text:", &[2]),
            text,
            colors::ansi_color("Width:", &[2]),
            dimensions.0,
            colors::ansi_color("Height:", &[2]),
            dimensions.1,
            colors::ansi_color("Ease:", &[2]),
            ease
        );
    }

    let (tx, rx) = mpsc::channel::<String>();

    let wiggle_thread =
        thread::spawn(move || wiggle::generate(&text, dimensions, &ease, &bezier_params, &tx));

    if show_progress && !raw_mode {
        let mut progress_bar = ProgressBar::new_spinner();
        for received in rx {
            progress_bar.finish();
            if received.to_lowercase().contains("finished") {
                break;
            }
            progress_bar = ProgressBar::new_spinner().with_message(received);
            progress_bar.enable_steady_tick(Duration::from_millis(120));
            progress_bar.set_style(
                ProgressStyle::with_template("{spinner:.magenta} {msg}")
                    .unwrap()
                    .tick_strings(cli::LOADER),
            );
        }
    }

    let wiggle = match wiggle_thread.join() {
        Ok(wiggle) => wiggle,
        Err(error) => {
            println!("{:?}", error);
            exit(1);
        }
    };

    if !raw_mode {
        println!(
            "{} {} \n",
            colors::ansi_color("Generated wiggle!", &[32, 4]),
            colors::ansi_color(&format!("({} characters)", wiggle.len()), &[2])
        );
    }

    if !quiet_mode {
        println!("{}", &wiggle);
    }
}
