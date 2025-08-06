use std::fs;

use crate::components::serialize::SlitherAnalysis;

// (Assuming the structs from Step 3 are in scope)

// This function will be part of your "reporter" module
pub fn generate_markdown_report(analysis: &SlitherAnalysis) -> String {
    let mut report = String::new();

    report.push_str("# Smart Contract Security Analysis Report\n\n");

    if analysis.results.detectors.is_empty() {
        report.push_str("✅ No vulnerabilities detected.\n");
        return report;
    }

    for detector in &analysis.results.detectors {
        report.push_str(&format!("## 🚨 {}\n\n", detector.check));
        report.push_str(&format!("- **Impact:** {}\n", detector.impact));
        report.push_str(&format!("- **Confidence:** {}\n\n", detector.confidence));
        report.push_str("### Description\n");
        // The description often contains markdown already, so we can just append it.
        report.push_str(&format!("{}\n\n", detector.description));
        report.push_str("---\n\n");
    }

    report
}