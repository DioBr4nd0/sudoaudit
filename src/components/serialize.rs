use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SlitherAnalysis {
    pub success: bool,
    pub results: SlitherResults,
}

#[derive(Deserialize, Debug)]
pub struct SlitherResults {
    pub detectors: Vec<Detector>
}

#[derive(Deserialize, Debug)]
pub struct Detector {
    pub check: String,
    pub impact : String,
    pub confidence: String,
    pub description: String,
}

pub fn parse_slither_output(json_string: &str) -> Result<SlitherAnalysis, serde_json::Error> {
    serde_json::from_str(json_string)
}