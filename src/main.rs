use clap::{Parser, Subcommand};
use colored::*;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "OmniRuntime", version = "1.0", about = "Universal Meta-Compiler & VM")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a script or source file with version check
    Run { file: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { file } => {
            if !Path::new(file).exists() {
                println!("{} {} not found!", "✘".red(), file);
                return;
            }
            process_file(file);
        }
    }
}

fn check_version(cmd: &str) -> Option<String> {
    let output = Command::new(cmd)
        .arg("--version")
        .stdout(Stdio::piped())
        .output();

    match output {
        Ok(out) => Some(String::from_utf8_lossy(&out.stdout).trim().to_string()),
        Err(_) => None,
    }
}

fn process_file(file: &str) {
    let ext = Path::new(file).extension().and_then(|s| s.to_str()).unwrap_or("");
    
    match ext {
        "py" => {
            match check_version("python3") {
                Some(ver) => {
                    println!("{} Using: {}", "ℹ".blue(), ver.cyan());
                    run_cmd("python3", vec![file]);
                }
                None => println!("{} Python3 is not installed!", "✘".red()),
            }
        }
        "rs" => {
            match check_version("rustc") {
                Some(ver) => {
                    println!("{} Using: {}", "ℹ".blue(), ver.cyan());
                    if run_cmd("rustc", vec![file, "-o", "temp_bin"]) {
                        run_cmd("./temp_bin", vec![]);
                    }
                }
                None => println!("{} Rust compiler (rustc) not found!", "✘".red()),
            }
        }
        "cpp" | "c" => {
            let compiler = if ext == "cpp" { "clang++" } else { "clang" };
            match check_version(compiler) {
                Some(ver) => {
                    println!("{} Using: {}", "ℹ".blue(), ver.cyan());
                    if run_cmd(compiler, vec![file, "-o", "temp_bin"]) {
                        run_cmd("./temp_bin", vec![]);
                    }
                }
                None => println!("{} {} not found!", "✘".red(), compiler),
            }
        }
        _ => println!("{} Unknown file type: .{}", "✘".red(), ext),
    }
}

fn run_cmd(cmd: &str, args: Vec<&str>) -> bool {
    let status = Command::new(cmd).args(args).status();
    match status {
        Ok(s) => s.success(),
        Err(e) => {
            println!("{} Failed: {}", "✘".red(), e);
            false
        }
    }
                                                               }
