use crate::renormalize::RenormalizationGroup;

/// Detects when and where conservation breaks.
#[derive(Debug, Clone)]
pub struct DriftDetector {
    pub law_index: usize,
    pub threshold: f64,
    /// Scale at which symmetry breaking first appears (from RG analysis).
    pub breaking_scale: Option<f64>,
    drift_history: Vec<(f64, f64)>, // (time, drift)
}

impl DriftDetector {
    /// Create a new drift detector for law at `law_index`.
    pub fn new(law_index: usize, threshold: f64) -> Self {
        Self {
            law_index,
            threshold,
            breaking_scale: None,
            drift_history: Vec::new(),
        }
    }

    /// Record a drift measurement at a given time.
    pub fn record_drift(&mut self, time: f64, drift: f64) {
        self.drift_history.push((time, drift));
        if drift > self.threshold && self.breaking_scale.is_none() {
            self.detect_breaking_scale();
        }
    }

    /// Whether drift has exceeded threshold.
    pub fn is_drifting(&self) -> bool {
        self.drift_history
            .last()
            .is_some_and(|(_, d)| *d > self.threshold)
    }

    /// Maximum drift recorded.
    pub fn max_drift(&self) -> f64 {
        self.drift_history
            .iter()
            .map(|(_, d)| *d)
            .fold(0.0_f64, f64::max)
    }

    /// Time of first threshold violation.
    pub fn first_violation_time(&self) -> Option<f64> {
        self.drift_history
            .iter()
            .find(|(_, d)| *d > self.threshold)
            .map(|(t, _)| *t)
    }

    /// Run RG-like analysis to find the scale of symmetry breaking.
    fn detect_breaking_scale(&mut self) {
        if self.drift_history.len() < 4 {
            return;
        }
        let times: Vec<f64> = self.drift_history.iter().map(|(t, _)| *t).collect();
        let durations: Vec<f64> = times.windows(2).map(|w| w[1] - w[0]).collect();
        if durations.is_empty() {
            return;
        }
        let base_dt: f64 = durations.iter().sum::<f64>() / durations.len() as f64;
        if base_dt <= 0.0 {
            return;
        }

        let rg = RenormalizationGroup::new(&self.drift_history);
        self.breaking_scale = rg.find_breaking_scale();
    }

    /// Number of drift samples recorded.
    pub fn sample_count(&self) -> usize {
        self.drift_history.len()
    }
}
