use std::fmt::Display;

use console::Style;

/// GCR logger.
pub fn grc_println(content: impl Display) {
	println!("{}", content)
}

/// GCR error logger.
pub fn grc_err_println(content: impl Display) {
	let color = Style::new().red();

	println!("{}", color.apply_to(content))
}

/// GCR error logger.
pub fn grc_warn_println(content: impl Display) {
	let color = Style::new().yellow();

	println!("{}", color.apply_to(content))
}


#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_grc_println() {
		grc_println("TEST CONTENT.");
	}

	#[test]
	fn it_grc_err_println() {
		grc_err_println("TEST ERROR CONTENT.");
	}

	#[test]
	fn it_grc_warn_println() {
		grc_warn_println("TEST WARN CONTENT.");
	}
}
