use std::path::PathBuf;
use std::process::Command;

/// Installs Slither into a managed virtual environment.
/// This is the only function you need to call.
pub fn install(project_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let venv_path = project_dir.join("slither-env");

    println!("🔧 Setting up Slither environment...");

    // There is no fallback. This is the only method.
    match setup_environment(&project_dir, &venv_path) {
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

/// Creates a Python virtual environment and installs slither-analyzer into it.
fn setup_environment(
    project_dir: &PathBuf,
    venv_path: &PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // 1. Create the virtual environment using Python's built-in `venv` module.
    println!("   - Creating virtual environment at '{}'...", venv_path.display());
    let venv_name = venv_path
        .file_name()
        .ok_or("Invalid venv path")?
        .to_str()
        .ok_or("Invalid venv name")?;

    // Try `python3` first, then fall back to `python`.
    let output = Command::new("python3")
        .args(&["-m", "venv", venv_name])
        .current_dir(project_dir)
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        println!("   - 'python3' failed. Trying 'python'...");
        let fallback_output = Command::new("python")
            .args(&["-m", "venv", venv_name])
            .current_dir(project_dir)
            .output()?;

        if !fallback_output.status.success() {
            let fallback_error_msg = String::from_utf8_lossy(&fallback_output.stderr);
            return Err(format!(
                "Failed to create venv with 'python3' (Error: {}) and 'python' (Error: {})",
                error_msg.trim(),
                fallback_error_msg.trim()
            )
            .into());
        }
    }
    println!("   - Virtual environment created successfully.");

    // 2. Determine the path to the `pip` executable.
    let pip_exe = if cfg!(target_os = "windows") {
        venv_path.join("Scripts").join("pip.exe")
    } else {
        venv_path.join("bin").join("pip")
    };

    if !pip_exe.exists() {
        return Err(format!("Could not find pip executable at {}", pip_exe.display()).into());
    }

    // 3. Install the package using the virtual environment's pip.
    println!("   - Installing 'slither-analyzer' with pip...");
    let output = Command::new(&pip_exe)
        .args(&["install", "--upgrade", "pip", "slither-analyzer"])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to install slither-analyzer: {}", error_msg.trim()).into());
    }
    println!("   - Slither-analyzer installed successfully.");

    // 4. Verify the installation and return the path to the executable.
    let slither_exe = get_slither_binary_path(venv_path);
    if !slither_exe.exists() {
        return Err("Slither executable not found after installation.".into());
    }
    
    Ok(slither_exe)
}

/// Helper function to determine the path to the Slither executable based on the OS.
fn get_slither_binary_path(venv_path: &PathBuf) -> PathBuf {
    if cfg!(target_os = "windows") {
        venv_path.join("Scripts").join("slither.exe")
    } else {
        venv_path.join("bin").join("slither")
    }
}