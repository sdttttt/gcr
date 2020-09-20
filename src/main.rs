use std::process::Command;
use std::io;
use std::fs;

mod repo;
mod util;

use repo::*;
use util::*;

fn main() {
    show_type();
    
    let current_path = current_path();
    let repo = Repository::new(current_path);
}

fn show_type() {
    println!("test:       Adding missing tests");
    println!("feat:       A new feature");
    println!("fix:        A bug fix");
    println!("chore:      Build process or auxiliary tool changes");
    println!("docs:       Documentation only changes");
    println!("refactor:   A code change that neither fixes a bug or adds a feature");
    println!("style:      Markup, white-space, formatting, missing semi-colons...");
    println!("perf:       A code change that improves performance");
    println!("ci:         CI related changes");
}

fn type_input() -> String {
    println!("Message Type ---------------------------");
    get_input()
}

fn scope_input() -> String {
    println!("Scope ---------------------------------");
    get_input()
}

fn msg_input() -> String {
    println!("Commit Message ------------------------");
    get_input()
}

fn get_input() -> String {
    let mut result = String::new();
    io::stdin().read_line(&mut result).expect("not string");
    result.pop();
    result
}