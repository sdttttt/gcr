mod arguments;
mod log;
mod message;
mod metadata;
mod repo;
mod util;

use arguments::*;
use log::*;
use message::*;
use repo::*;
use util::*;

fn main() {
    // input parameters.
    let arg = {
        match Arguments::collect() {
            Ok(a) => a,
            Err(e) => {
                gcr_err_println(e.message());
                return;
            }
        }
    };

    // repository path.
    let path = current_path();

    // repository Object instance.
    let repo = {
        match Repository::new(path, arg) {
            Ok(r) => r,
            Err(e) => {
                gcr_err_println(e.message());
                return;
            }
        }
    };

    // before commit hook.
    if let Err(e) = repo.pre_commit() {
        gcr_err_println(e.message());
        return;
    }

    // commit message.
    let message = Messager::new().build();
    gcr_println(&message);

    // Git commit
    if let Err(e) = repo.commit(message.as_str()) {
        gcr_err_println(e.message());
    }

    // after commit hook.
    if let Err(e) = repo.after_commit() {
        gcr_err_println(e.message());
    }
}
