use clap::{Parser, Subcommand};
use colored::*;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "OmniRuntime")]
#[command(about = "Universal Meta-Compiler & VM", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a script or source file (Python, Rust, C++, etc.)
    Run { file: String },
    /// Build a project into a standalone binary
    Build { file: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { file } => {
            run_file(file);
        }
        Commands::Build { file } => {
            println!("{} Building {}...", "●".blue(), file);
            // Build logic using LLVM would go here
        }
    }
}

fn run_file(file: &str) {
    let path = Path::new(file);
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    println!("{} Detecting environment for: .{}", "✔".green(), extension);

    match extension {
        "py" => {
            println!("{} Executing Python Script...", "➜".yellow());
            let status = Command::new("python3").arg(file).status();
            check_status(status);
        }
        "rs" => {
            println!("{} Compiling and Running Rust...", "➜".yellow());
            let status = Command::new("rustc").arg(file).arg("-o").arg("temp_bin").status();
            if status.is_ok() {
                let _ = Command::new("./temp_bin").status();
            }
        }
        "cpp" | "c" => {
            println!("{} Running C++ via Clang/LLVM JIT...", "➜".yellow());
            let status = Command::new("clang++").arg(file).arg("-o").arg("temp_bin").status();
             if status.is_ok() {
                let _ = Command::new("./temp_bin").status();
            }
        }
        _ => println!("{} Unsupported file extension: .{}", "✘".red(), extension),
    }
}

fn check_status(status: std::io::Result<std::process::ExitStatus>) {
    if let Err(e) = status {
        println!("{} Error: {}", "✘".red(), e);
    }
}
