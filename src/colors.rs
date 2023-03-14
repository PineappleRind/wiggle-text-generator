pub fn ansi_color(text: &str, colors: &[i32]) -> String {
    let mut output = String::new();
    for color in colors {
        output.push_str(&format!("\x1b[{}m", color));
    }
    output.push_str(&format!("{}\x1b[0m", text));

    output
}
