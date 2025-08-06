use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const SOLC_VERSION: &str = "0.8.20";

pub fn run_slither_analysis(contract_path: &Path, venv_path: &Path) -> Result<String, String> {
    let slither_executable = if cfg!(target_os = "windows") { venv_path.join("Scripts").join("slither.exe") } else { venv_path.join("bin").join("slither") };
    let venv_bin_path = slither_executable.parent().ok_or("Could not get parent dir")?;
    let current_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", venv_bin_path.display(), current_path);
    let temp_file_path = env::temp_dir().join(format!("slither-out-{}.json", std::process::id()));

    let output = Command::new(&slither_executable)
        .env("PATH", new_path)
        .arg(contract_path)
        .arg("--json")
        .arg(&temp_file_path)
        .arg("--solc-solcs-select")
        .arg(SOLC_VERSION)
        .output()
        .map_err(|e| format!("Failed to execute Slither: {}", e))?;

    if !temp_file_path.exists() || fs::metadata(&temp_file_path).unwrap().len() == 0 {
        return Err(format!("Slither failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let json_output = fs::read_to_string(&temp_file_path).map_err(|e| format!("Failed to read slither output file: {}", e))?;
    let _ = fs::remove_file(&temp_file_path);
    Ok(json_output)
}

pub fn run_mythril_analysis(bytecode: &str, venv_path: &Path) -> Result<String, String> {
    let mythril_executable = if cfg!(target_os = "windows") { venv_path.join("Scripts").join("myth.exe") } else { venv_path.join("bin").join("myth") };
    let venv_bin_path = mythril_executable.parent().ok_or("Could not get parent dir")?;
    let current_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", venv_bin_path.display(), current_path);

    let output = Command::new(&mythril_executable)
        .env("PATH", new_path)
        .arg("analyze")
        .arg("-c")
        .arg(bytecode)
        .arg("-o")
        .arg("json")
        .output()
        .map_err(|e| format!("Failed to execute Mythril: {}", e))?;

    if !output.status.success() && !String::from_utf8_lossy(&output.stderr).contains("issues were found") {
        return Err(format!("Mythril failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    String::from_utf8(output.stdout).map_err(|e| format!("Failed to read Mythril output: {}", e))
}