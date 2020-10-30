use std::fmt::Display;

use console::Style;

// GCR logger.
pub fn gcr_println(content: impl Display) {
    println!("GRC: {}", content)
}

// GCR error logger.
pub fn gcr_err_println(content: impl Display) {
    let color = Style::new().red();
    let output = format!("GRC(ERROR): {}", content);

    println!("{}", color.apply_to(output))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gcr_println() {
        gcr_println("TEST CONTENT.");
    }

    #[test]
    fn test_gcr_err_println() {
        gcr_err_println("TEST ERROR CONTENT.");
    }
}
