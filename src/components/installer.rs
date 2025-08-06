use std::path::{Path, PathBuf};
use std::process::Command;

const SOLC_VERSION: &str = "0.8.20";

/// The main installation function called from main.rs
pub fn install_all_tools(project_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Setting up Slither Environment ---");
    setup_tool_env(
        &project_dir.join("slither-env"),
        &["slither-analyzer", "solc-select"],
    )?;

    println!("\n--- Setting up Mythril Environment ---");
    setup_tool_env(
        &project_dir.join("mythril-env"),
        &["mythril", "solc-select"],
    )?;

    println!("\n✅ All environments created successfully!");
    Ok(())
}

/// A generic function to set up a virtual environment for a specific toolset.
fn setup_tool_env(venv_path: &Path, packages: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    println!("   - Creating virtual environment at '{}'...", venv_path.display());
    let python_cmd = find_compatible_python()?;
    let venv_output = Command::new(python_cmd).args(["-m", "venv", venv_path.to_str().unwrap()]).output()?;
    if !venv_output.status.success() { return Err(format!("Failed to create venv: {}", String::from_utf8_lossy(&venv_output.stderr)).into()); }

    let pip_exe = if cfg!(target_os = "windows") { venv_path.join("Scripts").join("pip.exe") } else { venv_path.join("bin").join("pip") };
    
    println!("   - Installing build dependencies (setuptools)...");
    let build_tools_output = Command::new(&pip_exe).args(["install", "--upgrade", "pip", "setuptools"]).output()?;
    if !build_tools_output.status.success() { return Err(format!("Failed to install build tools: {}", String::from_utf8_lossy(&build_tools_output.stderr)).into()); }

    println!("   - Installing packages: {:?}...", packages);
    let mut install_cmd = Command::new(&pip_exe);
    install_cmd.arg("install");
    for &pkg in packages {
        install_cmd.arg(pkg);
    }
    let tools_output = install_cmd.output()?;
    if !tools_output.status.success() { return Err(format!("Failed to install packages: {}", String::from_utf8_lossy(&tools_output.stderr)).into()); }

    let solc_select_exe = if cfg!(target_os = "windows") { venv_path.join("Scripts").join("solc-select.exe") } else { venv_path.join("bin").join("solc-select") };
    println!("   - Installing Solidity compiler v{} into this environment...", SOLC_VERSION);
    let solc_install_output = Command::new(&solc_select_exe).args(["install", SOLC_VERSION]).output()?;
    if !solc_install_output.status.success() { return Err(format!("Failed to install solc {}: {}", SOLC_VERSION, String::from_utf8_lossy(&solc_install_output.stderr)).into()); }

    println!("   - Environment setup complete.");
    Ok(())
}

/// Checks if a specific tool environment is already set up.
pub fn is_env_setup(venv_path: &Path) -> bool {
    // We can just check for the existence of the directory itself now.
    venv_path.exists()
}

/// Finds a compatible Python version on the system.
fn find_compatible_python() -> Result<&'static str, Box<dyn std::error::Error>> {
    // THE FIX IS ON THIS LINE: Added "python3" to the list of versions to check.
    let versions = ["python3.11", "python3.10", "python3", "python"];
    println!("   - Searching for a compatible Python version...");
    for &version in &versions {
        if Command::new(version).arg("--version").output().is_ok() {
            println!("   - Found compatible version: {}", version);
            return Ok(version);
        }
    }
    Err("Could not find a compatible Python version (3.10 or 3.11 recommended).".into())
}