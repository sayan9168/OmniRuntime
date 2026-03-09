use clap::{Parser, Subcommand};
use colored::*;
use notify::{Watcher, RecursiveMode, Config, RecommendedWatcher};
use std::{fs, path::Path, process::{Command, Stdio}, sync::mpsc::channel};
use serde_json::json;

#[derive(Parser)]
#[command(name = "OmniRuntime", version = "6.0", about = "Universal AI-Powered Runtime")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a file with real AI error fixing
    Run { file: String },
    /// Watch a file and re-run on every save
    Watch { file: String },
    /// Set your Gemini API Key
    Config { key: String },
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
            execute_with_ai(file);
        }
        Commands::Watch { file } => {
            println!("{} Starting Hot-Reload mode for: {}", "👀".cyan(), file.bold());
            if let Err(e) = watch_file(file) {
                println!("{} Watch error: {:?}", "✘".red(), e);
            }
        }
        Commands::Config { key } => {
            fs::write(".api_key", key).expect("Failed to save API Key");
            println!("{} API Key saved successfully!", "✔".green());
        }
        Commands::Setup => run_setup(),
    }
}

// --- Real Gemini AI Engine ---
fn ask_ai(error_message: String, code_content: String) {
    let api_key = fs::read_to_string(".api_key").unwrap_or_default().trim().to_string();
    if api_key.is_empty() {
        println!("{} No API Key found. Use 'config <key>' to enable Gemini AI fixing.", "⚠️".yellow());
        return;
    }

    println!("{} Consulting Gemini AI for a fix...", "🧠".magenta());

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let client = reqwest::blocking::Client::new();
    let prompt = format!(
        "I have a coding error. Here is the code:\n\n{}\n\nError Message:\n{}\n\nPlease explain why it failed and provide the corrected code.",
        code_content, error_message
    );

    let body = json!({
        "contents": [{ "parts": [{ "text": prompt }] }]
    });

    match client.post(url).json(&body).send() {
        Ok(response) => {
            let json: serde_json::Value = response.json().unwrap_or_default();
            if let Some(suggestion) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                println!("\n{} AI Solution:\n{}", "✨".cyan().bold(), suggestion);
            } else {
                println!("{} AI was silent. Check your API key or connection.", "✘".red());
            }
        }
        Err(_) => println!("{} Network error. Could not reach Gemini AI.", "✘".red()),
    }
}

// --- Execution Core ---
fn execute_with_ai(file: &str) {
    let content = fs::read_to_string(file).unwrap_or_default();
    let ext = Path::new(file).extension().and_then(|s| s.to_str()).unwrap_or_else(|| detect_language(&content));

    println!("{} Running as: {}", "🚀".blue(), ext.to_uppercase().bold());

    let output = match ext {
        "py" => Command::new("python3").arg(file).stderr(Stdio::piped()).stdout(Stdio::inherit()).output(),
        "rs" => {
            let res = Command::new("rustc").arg(file).arg("-o").arg("temp_bin").stderr(Stdio::piped()).output();
            if let Ok(ref out) = res {
                if out.status.success() { let _ = Command::new("./temp_bin").status(); }
            }
            res
        },
        "cpp" => {
            let res = Command::new("clang++").arg(file).arg("-o").arg("temp_bin").stderr(Stdio::piped()).output();
             if let Ok(ref out) = res {
                if out.status.success() { let _ = Command::new("./temp_bin").status(); }
            }
            res
        },
        _ => {
            println!("{} Unsupported language.", "✘".red());
            return;
        }
    };

    if let Ok(out) = output {
        if !out.status.success() {
            let err_msg = String::from_utf8_lossy(&out.stderr).to_string();
            println!("\n{} Execution Failed!", "✘".red());
            ask_ai(err_msg, content);
        }
    }
}

// --- Watcher (Hot-Reload) ---
fn watch_file(file: &str) -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(Path::new(file), RecursiveMode::NonRecursive)?;

    execute_with_ai(file);

    for res in rx {
        match res {
            Ok(_) => {
                println!("\n{} Change detected! Re-running...", "🔄".green());
                execute_with_ai(file);
            },
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
    Ok(())
}

// --- Helpers ---
fn detect_language(content: &str) -> &'static str {
    if content.contains("fn main()") { "rs" }
    else if content.contains("import ") || content.contains("print(") { "py" }
    else if content.contains("#include") { "cpp" }
    else { "py" }
}

fn run_setup() {
    let tools = vec![("python3", "Python"), ("rustc", "Rust"), ("clang++", "C++")];
    for (cmd, name) in tools {
        let status = Command::new(cmd).arg("--version").stdout(Stdio::null()).status();
        if status.is_ok() { println!("{} {} is ready.", "✔".green(), name); }
        else { println!("{} {} is missing!", "✘".red(), name); }
    }
        }
