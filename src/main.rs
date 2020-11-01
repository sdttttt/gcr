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
            Ok(a) => a,
            Err(e) => {
                gcr_err_println(e.message());
                return;
            }
        };

    // repository path.
    let path = current_path();

    // repository Object instance.
    let repo = match Repository::new(path, arg) {
            Ok(r) => r,
            Err(e) => {
                gcr_err_println(e.message());
                return;
            }
        };
	
	// extends types.
	let mut types: Vec<String> = vec![];
	
	// parse configuration file to Extensions struct.
    if let Ok(extends) = Extensions::from_agreement() {
		types = extends.types().clone();
    }
	
    // commit message.
    let message = Messager::new().load_ext_td(&types).ask().build();
    gcr_println(&message);
	
	// before commit hook.
	if let Err(e) = repo.pre_commit() {
		gcr_err_println(e.message());
		return;
	}

    // Git commit
    if let Err(e) = repo.commit(message.as_str()) {
        gcr_err_println(e.message());
    }

    // after commit hook.
    if let Err(e) = repo.after_commit() {
        gcr_err_println(e.message());
    }
}
