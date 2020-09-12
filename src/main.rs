use std::process::Command;
use std::io::{self, Write};


fn main() {
    println!("Type:");
    show_type();
    
    let msgType = type_input();
    let scope = scope_input();
    let msg = msg_input();
    
    println!("{} {} {}", msgType, scope, msg);
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