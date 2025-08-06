use serde::Deserialize;

// --- Slither Structs ---
#[derive(Deserialize, Debug)]
pub struct SlitherAnalysis {
    pub success: bool,
    pub results: SlitherResults,
}
#[derive(Deserialize, Debug)]
pub struct SlitherResults {
    pub detectors: Vec<SlitherDetector>,
}
#[derive(Deserialize, Debug)]
pub struct SlitherDetector {
    pub check: String,
    pub impact: String,
    pub confidence: String,
    pub description: String,
}
pub fn parse_slither_output(json_string: &str) -> Result<SlitherAnalysis, serde_json::Error> {
    serde_json::from_str(json_string)
}

// --- Mythril Structs ---
#[derive(Deserialize, Debug)]
pub struct MythrilAnalysis {
    pub success: bool,
    #[serde(default)] // Use default (empty vec) if 'issues' is null or missing
    pub issues: Vec<MythrilIssue>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MythrilIssue {
    pub title: String,
    pub description: String,
    pub severity: String,
    // Using Option in case the field is missing from the JSON
    pub address: Option<u32>,
}
pub fn parse_mythril_output(json_string: &str) -> Result<MythrilAnalysis, serde_json::Error> {
    serde_json::from_str(json_string)
}