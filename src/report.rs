use serde::{Deserialize, Serialize};

/// A single conservation violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub law_name: String,
    pub law_index: usize,
    pub first_violation: Option<f64>,
    pub last_violation: Option<f64>,
    pub violation_count: usize,
    pub breaking_scale: Option<f64>,
}

/// Full monitoring report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub total_ticks: usize,
    pub health: f64,
    pub total_violations: usize,
    pub violations: Vec<Violation>,
}

impl Report {
    /// Generate a human-readable text report.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str("=== NoetherGuard Report ===\n");
        out.push_str(&format!("Ticks:    {}\n", self.total_ticks));
        out.push_str(&format!("Health:   {:.1}%\n", self.health * 100.0));
        out.push_str(&format!("Violations: {}\n", self.total_violations));

        if self.violations.is_empty() {
            out.push_str("\nAll conservation laws satisfied. ✓\n");
        } else {
            out.push_str("\nViolations:\n");
            for v in &self.violations {
                out.push_str(&format!(
                    "  [{}] {} — {} violations",
                    v.law_index, v.law_name, v.violation_count
                ));
                if let (Some(first), Some(last)) = (v.first_violation, v.last_violation) {
                    out.push_str(&format!(" (t={:.3}..{:.3})", first, last));
                }
                if let Some(scale) = v.breaking_scale {
                    out.push_str(&format!(" breaking_scale={:.4}", scale));
                }
                out.push('\n');
            }
        }
        out
    }

    /// Generate JSON report.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_text())
    }
}
