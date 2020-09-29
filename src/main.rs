mod arguments;
mod log;
mod message;
mod repo;
mod util;

use arguments::*;
use log::*;
use message::*;
use repo::*;
use util::*;

fn main() {
    let arg = Arguments::collect();

    let current_path = current_path();
    let repo = {
        match Repository::new(current_path, arg) {
            Ok(r) => r,
            Err(e) => {
                gcr_err_println(e.message());
                return;
            }
        }
    };

    if let Err(e) = repo.pre_commit() {
        gcr_err_println(e.message());
        return;
    }

    let message = Messager::new().build();
    gcr_println(&message);

    if let Err(e) = repo.commit(message.as_str()) {
        gcr_err_println(e.message());
    }

    if let Err(e) = repo.after_commit() {
        gcr_err_println(e.message());
    }
}
