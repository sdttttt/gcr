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
    println!("Commit Message: {}", message);
    if let Err(e) = repo.commit(message.as_str()) {
        println!("{}", e.message());
    } 
}
