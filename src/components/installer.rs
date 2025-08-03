use rip::install::InstallOptions;
use rip::venv::VirtualEnvironment;
use std::process::Command;
use std::path::PathBuf;
use std::env;

pub async fn install(project_dir:&PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let venv_path = project_dir.join("slither-env");

    println!("🔧 Attempting rip-based setup...");

    match setup_with_rip(&venv_path).await {
        Ok(path) => {
            println!("✅ Rip-based setup successful!");
            println!("📍 Slither binary: {}", path.display());
            return Ok(());
        }
        Err(e) => {
            println!("⚠️  Rip approach failed: {}", e);
            println!("🔄 Falling back to system command approach...");
        }
    }

    match setup_with_setup_commands(&project_dir, &venv_path) {
        Ok(path) => {
            println!("✅ System command setup successful!");
            println!("📍 Slither binary: {}", path.display());
        }
        Err(e) => {
            eprintln!("❌ All approaches failed: {}", e);
            return Err(e.into());
        }
    }
    Ok(())
}

async fn setup_with_rip(venv_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let venv = VirtualEnvironment::create(venv_path).await?;
    let install_options = InstallOptions::default();
    venv.install_package("slither-analyzer", &install_options).await?;
    Ok(get_slither_binary_path(venv_path))
}

fn setup_with_system_commands(project_dir: &PathBuf, venv_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {

    println!("🔧 Creating virtual environment using system commands...");
    let venv_name = venv_path.file_name()
        .ok_or("Invalid venv path")?
        .to_str()
        .ok_or("Invalid venv name")?;
    
    let output = Command::new("python3")
        .args(&["-m", "venv", venv_name])
        .current_dir(project_dir)
        .output()?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to create virtual environment: {}", error_msg).into());
    }
    
    println!("✅ Virtual environment created successfully");
    
    let (python_exe, pip_exe, slither_exe) = if cfg!(windows) {
        (
            venv_path.join("Scripts").join("python.exe"),
            venv_path.join("Scripts").join("pip.exe"),
            venv_path.join("Scripts").join("slither.exe"),
        )
    } else {
        (
            venv_path.join("bin").join("python"),
            venv_path.join("bin").join("pip"),
            venv_path.join("bin").join("slither"),
        )
    };
    
    println!("📦 Upgrading pip...");
    let output = Command::new(&pip_exe)
        .args(&["install", "--upgrade", "pip"])
        .output()?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        println!("⚠️  Warning: Failed to upgrade pip: {}", error_msg);
        // Don't fail here, continue with installation
    } else {
        println!("✅ Pip upgraded successfully");
    }
    
    println!("🔧 Installing slither-analyzer...");
    let output = Command::new(&pip_exe)
        .args(&["install", "slither-analyzer"])
        .output()?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to install slither-analyzer: {}", error_msg).into());
    }
    
    println!("✅ Slither-analyzer installed successfully");
    
    println!("🔍 Verifying Slither installation...");
    let output = Command::new(&slither_exe)
        .args(&["--version"])
        .output()?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Slither verification failed: {}", error_msg).into());
    }
    
    let version_output = String::from_utf8_lossy(&output.stdout);
    println!("✅ Slither verified successfully: {}", version_output.trim());
    
    Ok(slither_exe)
}

fn get_slither_binary_path(venv_path: &PathBuf) -> PathBuf {
    match env::consts::OS {
        "windows" => venv_path.join("Scripts").join("slither.exe"),
        "macos" | "linux" => venv_path.join("bin").join("slither"),
        _ => {
            println!("⚠️  Unknown OS, assuming Unix-like structure");
            venv_path.join("bin").join("slither")
        }
    }
}