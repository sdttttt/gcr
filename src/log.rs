use std::fmt::Display;

use console::Style;

/// GCR logger.
pub fn grc_log(content: impl Display) {
	println!("{}", content)
}

/// GCR error logger.
pub fn grc_err(content: impl Display) {
	let color = Style::new().red();

	println!("{}", color.apply_to(content))
}

/// GCR error logger.
pub fn grc_warn(content: impl Display) {
	let color = Style::new().yellow();

	println!("{}", color.apply_to(content))
}

/// GCR success logger.
pub fn grc_succ(content: impl Display) {
	let color = Style::new().green();

	println!("{}", color.apply_to(content))
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_grc_println() {
		grc_log("TEST CONTENT.");
	}

	#[test]
	fn it_grc_err_println() {
		grc_err("TEST ERROR CONTENT.");
	}

	#[test]
	fn it_grc_warn_println() {
		grc_warn("TEST WARN CONTENT.");
	}

	#[test]
	fn it_grc_succ_println() {
		grc_succ("TEST SUCCESS CONTENT.");
	}
}
