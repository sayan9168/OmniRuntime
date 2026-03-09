use clap::{Parser, Subcommand};
use colored::*;
use notify::{Watcher, RecursiveMode, Config, RecommendedWatcher};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "OmniRuntime", version = "4.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a file once
    Run { file: String },
    /// Watch a file and re-run on every save
    Watch { file: String },
    /// Check system dependencies
    Setup,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run { file } => {
            if !Path::new(file).exists() { return; }
            smart_executor(file);
        }
        Commands::Watch { file } => {
            println!("{} Starting Hot-Reload mode for: {}", "👀".cyan(), file.bold());
            if let Err(e) = watch_file(file) {
                println!("{} Watch error: {:?}", "✘".red(), e);
            }
        }
        Commands::Setup => run_setup(),
    }
}

fn watch_file(file: &str) -> notify::Result<()> {
    let (tx, rx) = channel();

    // ফাইল ওয়াচার সেটআপ
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(Path::new(file), RecursiveMode::NonRecursive)?;

    // প্রথমবার রান করা
    smart_executor(file);

    println!("\n{} Waiting for changes...", "⏱".yellow());

    for res in rx {
        match res {
            Ok(_) => {
                println!("\n{} Change detected! Re-running...", "🔄".green());
                smart_executor(file);
                println!("\n{} Watching for more changes...", "⏱".yellow());
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}

fn detect_language(content: &str) -> &'static str {
    if content.contains("fn main()") { "rs" }
    else if content.contains("import ") || content.contains("print(") { "py" }
    else if content.contains("#include") { "cpp" }
    else if content.contains("package main") { "go" }
    else { "unknown" }
}

fn smart_executor(file: &str) {
    let content = fs::read_to_string(file).unwrap_or_default();
    let ext = Path::new(file).extension().and_then(|s| s.to_str()).unwrap_or(detect_language(&content));

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
        _ => println!("{} Unknown language.", "✘".red()),
    }
}

fn run_setup() { /* আগের মতো */ }

fn run_cmd(cmd: &str, args: Vec<&str>) -> bool {
    let status = Command::new(cmd).args(args).status();
    match status {
        Ok(s) => s.success(),
        Err(_) => false,
    }
    }
                          
