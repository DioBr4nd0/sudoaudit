use clap::Parser;
use std::fs;
use std::io::{self, Write};

pub mod components;
use components::{analyzer, cli::Cli, generator, installer, serialize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. --- SETUP ---
    let cli = Cli::parse();

    // Determine the application's data directory in the user's home folder
                            let app_dir = dirs::home_dir()
                                .ok_or("Could not find home directory")?
                                .join(".sudoaudit");

    // Ensure the app directory exists
    fs::create_dir_all(&app_dir)?;

    // Determine the expected path for the slither virtual environment and binary
    let venv_path = app_dir.join("slither-env");
    let slither_binary_path = installer::get_slither_binary_path(&venv_path);


    // 2. --- CHECK & INSTALL ---
    if !slither_binary_path.exists() {
        println!("🔧 Slither installation not found.");
        print!("   Would you like to install it now? (Y/n) ");
        io::stdout().flush()?; // Make sure the question is printed before waiting for input

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;

        if user_input.trim().to_lowercase() == "y" {
            // User agreed, run the installer
            installer::install(&app_dir)?;
        } else {
            println!("❌ Installation cancelled. Cannot proceed without Slither.");
            return Ok(()); // Exit gracefully
        }
    }


    // 3. --- ANALYZE ---
    println!("\n🔎 Running analysis on '{}'...", cli.contract_path.display());

    let json_output = analyzer::run_slither_analysis(&cli.contract_path, &slither_binary_path)
        .map_err(|e| format!("Analysis Error: {}", e))?;


    // 4. --- PARSE ---
    println!("📑 Parsing results...");
    let analysis_results = serialize::parse_slither_output(&json_output)
        .map_err(|e| format!("JSON Parsing Error: {}", e))?;

    // 5. --- GENERATE & SAVE REPORT ---
    println!("✍️  Generating report...");
    let report_content = generator::generate_markdown_report(&analysis_results);
    fs::write(&cli.output, report_content)?;

    println!(
        "\n✅ Success! Report saved to '{}'",
        cli.output.display()
    );

    Ok(())
}