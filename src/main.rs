use std::process::Command;

fn main() {
    
    let output = Command::new("git").arg("help").output().expect("Execute Failed.");
    
    println!("Status: {}", output.status);
    let result = String::from_utf8_lossy(&output.stdout);

    println!("{}", result)
}
