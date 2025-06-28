use std::process::Command;

#[test]
fn test_hello() {
    let out = Command::new("cargo")
        .args(["run", "--bin=rs_001"])
        .output()
        .expect("command failed");

    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "Hello Rust!");
}

pub fn print_hello() {
    println!("Hello Rust!");
}

fn main() {
    print_hello();
}

