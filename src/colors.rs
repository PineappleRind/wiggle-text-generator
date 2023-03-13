pub fn ansi_color(text: &str, colors: Vec<i32>) -> String {
    let mut output = "".to_string();
    for color in colors.iter() {
        output.push_str(&format!("\x1b[{}m", color.to_string()));
    }
    output.push_str(&format!("{}\x1b[0m", text));

    return output;
}
