use leet::{get_problem_from, Problem};
use std::process::Command;

use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(help = "Problem title copied from LeetCode. Of the form \"XXXX. Problem Name Here\"")]
    problem_title: String,
}

fn main() {
    let args = Cli::parse();

    let Problem { number, name } = get_problem_from(args.problem_title);

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
