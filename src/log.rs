use std::fmt::Display;

use console::Style;

pub fn gcr_println(content: impl Display) {
    println!("GRC: {}", content)
}

pub fn gcr_err_println(content: impl Display) {
    let color = Style::new().red();
    let output = format!("GRC(ERROR): {}", content);

    println!("{}", color.apply_to(output))
}