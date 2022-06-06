use std::env;
use std::process::Command;

fn main() {
    let mut args = env::args();
    if args.len() > 2 {
        panic!("Args should not exceed length 2")
    }

    let title = args.nth(1).expect("No exercise name given");

    let mut characters = title.chars().enumerate();

    let mut number = String::new();
    let mut name = String::new();
    while let Some((index, character)) = characters.next() {
        match character {
            '0'..='9' => continue,
            '.' => {
                number = format!("{:0>4}", &title[0..index]);
                name = title[index + 2..]
                    .to_lowercase()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join("-");
                break;
            }
            _ => panic!("Unexpected character at start {} {}", index, character),
        }
    }

    let path = format!("{}_{}", &number, &name);

    Command::new("cargo")
        .arg("new")
        .arg(&path)
        .arg("--name")
        .arg(name)
        .arg("--lib")
        .output()
        .expect("Failed to run cargo new");

    Command::new(r"C:\Program Files\Microsoft VS Code\Code.exe")
        .arg(format!(r".\{}\", &path))
        .output()
        .expect("Failed to open code");
}
