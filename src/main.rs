use std::io;

mod message;
mod repo;
mod util;

use message::*;
use repo::*;
use util::*;

fn main() {
    
    let current_path = current_path();
    let repo = Repository::new(current_path);

    let message = Messager::new().build();
    repo.commit(message.as_str());
}
