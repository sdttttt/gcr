use std::fmt::Display;

use console::{Style, Term};

/// GCR logger.
pub fn grc_println(content: impl Display) {
	let stdout = Term::stdout();
	stdout.write_line(content.to_string().as_str()).expect("should output str..");
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

pub fn grc_success_println(content: impl Display) {
	let color = Style::new().green();

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

	#[test]
	fn it_grc_success_println() {
		grc_success_println("TEST SUCCESS CONTENT.");
	}
}
