use std::rc::Rc;

mod arguments;
mod extensions;
mod log;
mod message;
mod metadata;
mod repo;
mod util;

use arguments::*;
use extensions::*;
use log::*;
use message::*;
use repo::*;
use util::*;

fn main() {
	// input parameters.
	let arg = match Arguments::collect() {
		| Ok(a) => Rc::new(a),
		| Err(e) => {
			grc_err_println(e.message());
			return;
		}
	};

	// repository path.
	let path = current_path();

	// repository Object instance.
	let repo = match Repository::new(path, Rc::clone(&arg)) {
		| Ok(r) => r,
		| Err(e) => {
			grc_err_println(e.message());
			return;
		}
	};

	// extends types.
	let mut types: Vec<String> = vec![];

	// parse configuration file to Extensions struct.
	if arg.has_specified_config() {
		if let Ok(extends) = Extensions::from(arg.config_file()) {
			types = extends.types().clone();
		}
	} else {
		if let Ok(extends) = Extensions::from_agreement() {
			types = extends.types().clone();
		}
	}

	// commit message.
	let message = Messager::new().load_ext_td(&types).ask().build();
	grc_println(&message);

	// Git commit
	if let Err(e) = repo.commit(message.as_str()) {
		grc_err_println(e.message());
	}
}
