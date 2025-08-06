use std::path::PathBuf;
use std::process::Command;

// A specific, known-good version of the Solidity compiler.
const SOLC_VERSION: &str = "0.8.20";

/// Installs Slither and Solc into a managed virtual environment.
pub fn install(project_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let venv_path = project_dir.join("slither-env");

    println!("🔧 Setting up Slither and Solc environment...");

    match setup_environment(&venv_path) {
        Ok(path) => {
            println!("\n✅ Setup successful!");
            println!("📍 Slither binary is available at: {}", path.display());
            Ok(())
        }
        Err(e) => {
            eprintln!("\n❌ Setup failed: {}", e);
            Err(e)
        }
    }
}

/// Creates a Python venv, installs packages, and sets up solc.
fn setup_environment(venv_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // 1. Create the virtual environment.
    println!("   - Creating virtual environment at '{}'...", venv_path.display());
    // Try `python3` first, then fall back to `python`.
    let venv_cmd = if Command::new("python3").arg("--version").output().is_ok() {
        "python3"
    } else {
        "python"
    };
    let venv_output = Command::new(venv_cmd)
        .args(["-m", "venv", venv_path.to_str().unwrap()])
        .output()?;
    if !venv_output.status.success() {
        return Err(format!(
            "Failed to create venv: {}",
            String::from_utf8_lossy(&venv_output.stderr)
        ).into());
    }
    println!("   - Virtual environment created.");

    // 2. Determine paths for pip and solc-select executables.
    let (pip_exe, solc_select_exe) = if cfg!(target_os = "windows") {
        (
            venv_path.join("Scripts").join("pip.exe"),
            venv_path.join("Scripts").join("solc-select.exe"),
        )
    } else {
        (
            venv_path.join("bin").join("pip"),
            venv_path.join("bin").join("solc-select"),
        )
    };

    // 3. Install packages using the virtual environment's pip.
    println!("   - Installing 'slither-analyzer' and 'solc-select'...");
    let pip_output = Command::new(&pip_exe)
        .args([
            "install",
            "--upgrade",
            "pip",
            "slither-analyzer",
            "solc-select",
        ])
        .output()?;
    if !pip_output.status.success() {
        return Err(format!(
            "Failed to install packages: {}",
            String::from_utf8_lossy(&pip_output.stderr)
        ).into());
    }
    println!("   - Packages installed.");

    // 4. Install the required solc version using solc-select.
    println!("   - Installing Solidity compiler version {}...", SOLC_VERSION);
    let solc_install_output = Command::new(&solc_select_exe)
        .args(["install", SOLC_VERSION])
        .output()?;
    if !solc_install_output.status.success() {
        return Err(format!(
            "Failed to install solc {}: {}",
            SOLC_VERSION,
            String::from_utf8_lossy(&solc_install_output.stderr)
        ).into());
    }
    println!("   - Solidity compiler installed.");

    // 5. Verify the installation and return the path to the slither executable.
    let slither_exe = get_slither_binary_path(venv_path);
    if !slither_exe.exists() {
        return Err("Slither executable not found after installation.".into());
    }

    Ok(slither_exe)
}

/// Helper function to determine the path to the Slither executable.
pub fn get_slither_binary_path(venv_path: &PathBuf) -> PathBuf {
    if cfg!(target_os = "windows") {
        venv_path.join("Scripts").join("slither.exe")
    } else {
        venv_path.join("bin").join("slither")
    }
}