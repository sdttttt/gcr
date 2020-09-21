use std::fmt::Display;

pub fn gcr_println(content: impl Display) {
    println!("GCR: {}", content)
}

pub fn gcr_err_println(content: impl Display) {
    println!("GCR(ERROR): {}", content)
}