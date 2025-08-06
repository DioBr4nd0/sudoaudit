use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Must match the version in installer.rs
const SOLC_VERSION: &str = "0.8.20";

pub fn run_slither_analysis(contract_path: &Path, binary_path: &Path) -> Result<String, String> {
    // Get the directory containing the slither binary
    let venv_bin_path = binary_path
        .parent()
        .ok_or("Could not get parent directory of the slither binary")?;

    // Get the current system PATH and prepend our venv path
    let current_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", venv_bin_path.display(), current_path);

    // Create a path for a temporary JSON output file.
    let temp_file_path = env::temp_dir().join(format!(
        "sudoaudit-slither-output-{}.json",
        std::process::id()
    ));

    // Run the command with the modified environment.
    let output = Command::new(binary_path)
        .env("PATH", new_path)
        .arg(contract_path)
        .arg("--json")
        .arg(&temp_file_path)
        .arg("--solc-solcs-select")
        .arg(SOLC_VERSION)
        .output()
        .map_err(|e| format!("Failed to execute Slither: {}", e))?;

    // --- START OF THE FIX ---
    //
    // Check if the JSON output file was actually created.
    // Slither exits with a non-zero status code if vulnerabilities are found,
    // so we can't just check for `output.status.success()`.
    // The most reliable check is to see if it produced the file we asked for.
    //
    if !temp_file_path.exists() || fs::metadata(&temp_file_path).unwrap().len() == 0 {
        // If the file doesn't exist or is empty, it was a REAL error.
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Slither failed with a fatal error and did not produce an output file. Error: {}", error_message));
    }
    // --- END OF THE FIX ---


    // Read the JSON output from the temporary file.
    let json_output = fs::read_to_string(&temp_file_path)
        .map_err(|e| format!("Failed to read slither output file: {}", e))?;

    // Clean up by deleting the temporary file.
    let _ = fs::remove_file(&temp_file_path);

    Ok(json_output)
}