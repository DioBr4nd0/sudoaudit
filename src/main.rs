use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub mod components;
use components::{
    analyzer, cli::Cli, compiler, installer, reporter, serialize,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. --- SETUP & INSTALL ---
    let cli = Cli::parse();
    let app_dir = dirs::home_dir().ok_or("Could not find home directory")?.join(".sudoaudit");
    fs::create_dir_all(&app_dir)?;

    // Define paths for TWO separate environments
    let slither_venv_path = app_dir.join("slither-env");
    let mythril_venv_path = app_dir.join("mythril-env");

    // Check if both environments are set up.
    if !installer::is_env_setup(&slither_venv_path) || !installer::is_env_setup(&mythril_venv_path) {
        println!("🔧 One or more analysis environments are not found.");
        print!("   Would you like to install them now? (This may take a few minutes) (Y/n) ");
        io::stdout().flush()?;
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        if user_input.trim().to_lowercase() == "y" {
            installer::install_all_tools(&app_dir)?;
        } else {
            println!("❌ Installation cancelled.");
            return Ok(());
        }
    }

    // --- INTERACTIVE ANALYSIS CHOICE ---
    let choice = loop {
        println!("\nChoose the analysis type:");
        println!("  1. Slither (Fast Static Analysis)");
        println!("  2. Mythril (Deep Symbolic Analysis)");
        println!("  3. Both (Comprehensive)");
        print!("Enter your choice (1, 2, or 3): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "1" | "2" | "3" => break input.trim().to_string(),
            _ => println!("Invalid choice. Please enter 1, 2, or 3."),
        }
    };

    // --- ANALYSIS EXECUTION ---
    let contract_name = cli.contract_path.file_stem().unwrap().to_str().unwrap();
    let reports_base_dir = PathBuf::from(contract_name);

    if choice == "1" || choice == "3" {
        run_slither(&cli.contract_path, &slither_venv_path, &reports_base_dir, contract_name)?;
    }
    if choice == "2" || choice == "3" {
        // Mythril's compilation needs its OWN solc from its OWN environment.
        run_mythril(&cli.contract_path, &mythril_venv_path, &reports_base_dir, contract_name)?;
    }

    println!("\n✅ All analyses complete! Reports are saved in the '{}' directory.", reports_base_dir.display());
    Ok(())
}


// --- Helper Functions ---

fn run_slither(contract_path: &Path, venv_path: &Path, reports_base_dir: &Path, contract_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔎 Running Slither analysis...");
    let json_output = analyzer::run_slither_analysis(contract_path, venv_path)?;
    let analysis_results = serialize::parse_slither_output(&json_output)?;
    
    let report_dir = reports_base_dir.join("slither-report");
    fs::create_dir_all(&report_dir)?;

    let md_report = reporter::generate_slither_report_md(&analysis_results.results);
    fs::write(report_dir.join(format!("{}_slither_report.md", contract_name)), md_report)?;
    
    let html_report = reporter::generate_slither_report_html(&analysis_results.results);
    fs::write(report_dir.join(format!("{}_slither_report.html", contract_name)), html_report)?;

    println!("   - Slither analysis complete.");
    Ok(())
}

fn run_mythril(contract_path: &Path, venv_path: &Path, reports_base_dir: &Path, contract_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚙️  Compiling contract to bytecode...");
    let bytecode = compiler::compile_to_bytecode(contract_path, venv_path)?;
    println!("   - Compilation complete.");
    
    println!("\n🔎 Running Mythril analysis...");
    let json_output = analyzer::run_mythril_analysis(&bytecode, venv_path)?;
    let analysis_results = serialize::parse_mythril_output(&json_output)?;

    let report_dir = reports_base_dir.join("mythril-report");
    fs::create_dir_all(&report_dir)?;

    let md_report = reporter::generate_mythril_report_md(&analysis_results);
    fs::write(report_dir.join(format!("{}_mythril_report.md", contract_name)), md_report)?;

    let html_report = reporter::generate_mythril_report_html(&analysis_results);
    fs::write(report_dir.join(format!("{}_mythril_report.html", contract_name)), html_report)?;

    println!("   - Mythril analysis complete.");
    Ok(())
}