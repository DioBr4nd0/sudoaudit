use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const SOLC_VERSION: &str = "0.8.20";

pub fn compile_to_bytecode(contract_path: &Path, venv_path: &Path) -> Result<String, String> {
    let solc_executable = if cfg!(target_os = "windows") { venv_path.join("Scripts").join("solc.exe") } else { venv_path.join("bin").join("solc") };
    let venv_bin_path = solc_executable.parent().ok_or("Could not get parent directory of solc")?;
    let current_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", venv_bin_path.display(), current_path);

    let output = Command::new(&solc_executable)
        .env("PATH", new_path)
        .env("SOLC_VERSION", SOLC_VERSION)
        .arg("--bin")
        .arg("--optimize")
        .arg(contract_path)
        .output()
        .map_err(|e| format!("Failed to execute solc: {}", e))?;

    if !output.status.success() {
        return Err(format!("solc failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to read solc output: {}", e))?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .last()
        .ok_or("Could not find bytecode in solc output.".to_string())
        .map(|s| s.to_string())
}