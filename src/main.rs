use clap::{Parser, Subcommand};
use colored::*;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "OmniRuntime", version = "2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run any language or hybrid scripts
    Run { file: String },
    /// Check and install missing compilers
    Setup,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { file } => {
            if !Path::new(file).exists() {
                println!("{} File not found!", "✘".red());
                return;
            }
            universal_executor(file);
        }
        Commands::Setup => {
            run_setup();
        }
    }
}

fn universal_executor(file: &str) {
    let ext = Path::new(file).extension().and_then(|s| s.to_str()).unwrap_or("");
    println!("{} {} Omni-Bridge identifying engine...", "🚀".cyan(), "OmniRuntime:".bold());

    match ext {
        "py" => {
            println!("{} Logic: Python Interpreter", "◆".yellow());
            run_cmd("python3", vec![file]);
        }
        "rs" => {
            println!("{} Logic: Rust Native Compiler", "◆".orange());
            if run_cmd("rustc", vec![file, "-o", "temp_bin"]) {
                run_cmd("./temp_bin", vec![]);
            }
        }
        "cpp" | "c" => {
            println!("{} Logic: LLVM/Clang Backend", "◆".blue());
            let compiler = if ext == "cpp" { "clang++" } else { "clang" };
            if run_cmd(compiler, vec![file, "-o", "temp_bin"]) {
                run_cmd("./temp_bin", vec![]);
            }
        }
        "go" => {
            println!("{} Logic: Go Runtime", "◆".cyan());
            run_cmd("go", vec!["run", file]);
        }
        _ => println!("{} Unknown language extension: .{}", "✘".red(), ext),
    }
}

fn run_setup() {
    let tools = vec!["python3", "rustc", "clang++", "go"];
    println!("{} Checking system dependencies...", "🔍".blue());

    for tool in tools {
        let check = Command::new(tool).arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).status();
        match check {
            Ok(_) => println!("{} {} is already installed.", "✔".green(), tool),
            Err(_) => println!("{} {} is MISSING!", "✘".red(), tool),
        }
    }
}

fn run_cmd(cmd: &str, args: Vec<&str>) -> bool {
    let status = Command::new(cmd).args(args).status();
    match status {
        Ok(s) => s.success(),
        Err(e) => {
            println!("{} Error: {}", "✘".red(), e);
            false
        }
    }
}
use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "OmniRuntime", version = "3.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run any code file (auto-detects language if extension is missing)
    Run { file: String },
    /// Check system dependencies
    Setup,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run { file } => {
            if !Path::new(file).exists() {
                println!("{} File not found!", "✘".red());
                return;
            }
            smart_executor(file);
        }
        Commands::Setup => run_setup(),
    }
}

fn detect_language(content: &str) -> &'static str {
    if content.contains("fn main()") || content.contains("println! (") {
        "rs"
    } else if content.contains("import ") || content.contains("def ") || content.contains("print(") {
        "py"
    } else if content.contains("#include <iostream>") || content.contains("int main()") {
        "cpp"
    } else if content.contains("package main") && content.contains("func ") {
        "go"
    } else {
        "unknown"
    }
}

fn smart_executor(file: &str) {
    let path = Path::new(file);
    let mut ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    // যদি এক্সটেনশন না থাকে, তবে কন্টেন্ট পড়ে ডিটেক্ট করবে
    let content = fs::read_to_string(file).unwrap_or_default();
    if ext == "" {
        println!("{} No extension found. Analyzing code content...", "🔍".yellow());
        ext = detect_language(&content);
    }

    println!("{} {} Target Engine: {}", "🚀".cyan(), "OmniRuntime:".bold(), ext.to_uppercase().green());

    match ext {
        "py" => { run_cmd("python3", vec![file]); }
        "rs" => {
            if run_cmd("rustc", vec![file, "-o", "temp_bin"]) {
                run_cmd("./temp_bin", vec![]);
            }
        }
        "cpp" => {
            if run_cmd("clang++", vec![file, "-o", "temp_bin"]) {
                run_cmd("./temp_bin", vec![]);
            }
        }
        "go" => { run_cmd("go", vec!["run", file]); }
        _ => println!("{} Could not identify the language automatically.", "✘".red()),
    }
}

fn run_setup() {
    let tools = vec![("python3", "Python"), ("rustc", "Rust"), ("clang++", "C++"), ("go", "Go")];
    println!("{} Checking system dependencies...", "🔍".blue());
    for (cmd, name) in tools {
        let check = Command::new(cmd).arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).status();
        match check {
            Ok(_) => println!("{} {} is ready.", "✔".green(), name),
            Err(_) => println!("{} {} is MISSING!", "✘".red(), name),
        }
    }
}

fn run_cmd(cmd: &str, args: Vec<&str>) -> bool {
    let status = Command::new(cmd).args(args).status();
    match status {
        Ok(s) => s.success(),
        Err(e) => {
            println!("{} Error: {}", "✘".red(), e);
            false
        }
    }
                              }
