use crate::components::serialize::{MythrilAnalysis, SlitherResults};

// --- Slither Report Generators ---
pub fn generate_slither_report_md(results: &SlitherResults) -> String {
    if results.detectors.is_empty() { return "✅ No vulnerabilities detected by Slither.\n".to_string(); }
    let mut report = "# Slither Security Analysis Report\n\n".to_string();
    for detector in &results.detectors {
        report.push_str(&format!("## 🚨 {}\n\n- **Impact:** {}\n- **Confidence:** {}\n\n### Description\n{}\n\n---\n\n",
            detector.check, detector.impact, detector.confidence, detector.description));
    }
    report
}

pub fn generate_slither_report_html(results: &SlitherResults) -> String {
    let body = if results.detectors.is_empty() {
        "<p class='no-issues'>✅ No vulnerabilities detected by Slither.</p>".to_string()
    } else {
        results.detectors.iter().map(|d| format!(
            "<div class='issue {}'><h2>{}</h2><div class='details'><span class='tag'>Impact: {}</span><span class='tag'>Confidence: {}</span></div><div class='description'><h3>Description</h3>{}</div></div>",
            d.impact.to_lowercase(), d.check, d.impact, d.confidence, markdown::to_html(&d.description)
        )).collect::<String>()
    };
    generate_html_scaffold("Slither Analysis Report", &body)
}

// --- Mythril Report Generators ---
pub fn generate_mythril_report_md(analysis: &MythrilAnalysis) -> String {
    if !analysis.success || analysis.issues.is_empty() { return "✅ No vulnerabilities detected by Mythril.\n".to_string(); }
    let mut report = "# Mythril Security Analysis Report\n\n".to_string();
    for issue in &analysis.issues {
        report.push_str(&format!("## 🔥 {}\n\n- **Severity:** {}\n\n### Description\n{}\n\n---\n\n",
            issue.title, issue.severity, issue.description));
    }
    report
}

pub fn generate_mythril_report_html(analysis: &MythrilAnalysis) -> String {
    let body = if !analysis.success || analysis.issues.is_empty() {
        "<p class='no-issues'>✅ No vulnerabilities detected by Mythril.</p>".to_string()
    } else {
        analysis.issues.iter().map(|i| format!(
            "<div class='issue {}'><h2>{}</h2><div class='details'><span class='tag'>Severity: {}</span></div><div class='description'><h3>Description</h3><p>{}</p></div></div>",
            i.severity.to_lowercase(), i.title, i.severity, i.description
        )).collect::<String>()
    };
    generate_html_scaffold("Mythril Analysis Report", &body)
}

// --- HTML Scaffold ---
fn generate_html_scaffold(title: &str, body: &str) -> String {
    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; line-height: 1.6; color: #333; max-width: 800px; margin: 2rem auto; padding: 0 1rem; background-color: #f9fafb; }}
        h1 {{ color: #111827; border-bottom: 2px solid #e5e7eb; padding-bottom: 0.5rem; }}
        h2 {{ color: #1f2937; }}
        .issue {{ background-color: #fff; border: 1px solid #e5e7eb; border-radius: 8px; margin-bottom: 1.5rem; padding: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.05); }}
        .issue.high, .issue.critical {{ border-left: 5px solid #ef4444; }}
        .issue.medium {{ border-left: 5px solid #f97316; }}
        .issue.low, .issue.informational {{ border-left: 5px solid #3b82f6; }}
        .details {{ margin-bottom: 1rem; }}
        .tag {{ display: inline-block; background-color: #e5e7eb; color: #4b5563; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500; margin-right: 0.5rem; }}
        .description {{ background-color: #f9fafb; border: 1px solid #e5e7eb; border-radius: 4px; padding: 0.5rem 1rem; }}
        .description h3 {{ margin-top: 0; color: #374151; }}
        .no-issues {{ font-size: 1.2rem; color: #16a34a; text-align: center; padding: 2rem; }}
        code {{ background-color: #e5e7eb; padding: 0.2rem 0.4rem; border-radius: 4px; font-family: "SF Mono", "Fira Code", monospace; }}
    </style>
</head>
<body>
    <h1>{}</h1>
    {}
</body>
</html>
"#, title, title, body)
}