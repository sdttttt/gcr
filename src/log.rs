use std::fmt::Display;

pub fn gcr_println(content: impl Display) {
    println!("GRC: {}", content)
}

pub fn gcr_err_println(content: impl Display) {
    println!("GRC(ERROR): {}", content)
}