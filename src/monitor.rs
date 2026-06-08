use crate::conservation::ConservationLaw;
use crate::drift::DriftDetector;
use crate::error::Result;
use crate::report::Report;
use crate::symmetry::Symmetry;

/// Snapshot of all conserved quantities at a point in time.
#[derive(Debug, Clone)]
pub struct MonitorSnapshot {
    pub time: f64,
    pub values: Vec<f64>,
    pub violations: Vec<usize>,
}

/// Monitors multiple conservation laws simultaneously.
#[derive(Debug, Clone)]
pub struct ConservationMonitor {
    pub laws: Vec<ConservationLaw>,
    pub history: Vec<MonitorSnapshot>,
    pub drift_detectors: Vec<DriftDetector>,
}

impl ConservationMonitor {
    /// Create an empty monitor.
    pub fn new() -> Self {
        Self {
            laws: Vec::new(),
            history: Vec::new(),
            drift_detectors: Vec::new(),
        }
    }

    /// Pre-built Newtonian monitor: energy + momentum (3) + angular momentum (3) = 7 values.
    pub fn newtonian(tolerance: f64) -> Self {
        let mut m = Self::new();
        m.laws.push(ConservationLaw::new(
            Symmetry::TimeTranslation,
            0.0,
            tolerance,
        ));
        m.laws.push(ConservationLaw::new(
            Symmetry::SpatialTranslation,
            0.0,
            tolerance,
        ));
        m.laws.push(ConservationLaw::new(Symmetry::Rotation, 0.0, tolerance));
        m.drift_detectors = m
            .laws
            .iter()
            .enumerate()
            .map(|(i, law)| DriftDetector::new(i, law.tolerance * 10.0))
            .collect();
        m
    }

    /// Pre-built Hamiltonian monitor: energy only.
    pub fn hamiltonian(tolerance: f64) -> Self {
        let mut m = Self::new();
        m.laws.push(ConservationLaw::new(
            Symmetry::TimeTranslation,
            0.0,
            tolerance,
        ));
        m.drift_detectors = m
            .laws
            .iter()
            .enumerate()
            .map(|(i, law)| DriftDetector::new(i, law.tolerance * 10.0))
            .collect();
        m
    }

    /// Custom monitor with user-supplied laws.
    pub fn custom(laws: Vec<ConservationLaw>) -> Self {
        let drift_detectors = laws
            .iter()
            .enumerate()
            .map(|(i, law)| DriftDetector::new(i, law.tolerance * 10.0))
            .collect();
        Self {
            laws,
            history: Vec::new(),
            drift_detectors,
        }
    }

    /// Number of scalar values expected per tick.
    pub fn expected_value_count(&self) -> usize {
        self.laws.iter().map(|l| l.symmetry.component_count()).sum()
    }

    /// Add a conservation law.
    pub fn add_law(&mut self, law: ConservationLaw) {
        let idx = self.laws.len();
        self.laws.push(law);
        self.drift_detectors
            .push(DriftDetector::new(idx, self.laws.last().unwrap().tolerance * 10.0));
    }

    /// Record a snapshot: check all conservation laws at time `t`.
    ///
    /// `values` layout: for each law in order, emit `component_count()` values.
    /// For Newtonian that is [energy, px, py, pz, lx, ly, lz].
    pub fn tick(&mut self, time: f64, values: &[f64]) -> Result<()> {
        let expected = self.expected_value_count();
        if values.len() != expected {
            return Err(crate::error::Error::ValueCountMismatch {
                expected,
                got: values.len(),
            });
        }

        let mut violations = Vec::new();
        let mut offset = 0;
        for (i, law) in self.laws.iter().enumerate() {
            let count = law.symmetry.component_count();
            // Check the first component (scalar summary) against tolerance
            if !law.is_conserved(values[offset]) {
                violations.push(i);
            }
            // Update drift detector
            if let Some(detector) = self.drift_detectors.get_mut(i) {
                detector.record_drift(time, law.drift(values[offset]));
            }
            offset += count;
        }

        self.history.push(MonitorSnapshot {
            time,
            values: values.to_vec(),
            violations: violations.clone(),
        });

        Ok(())
    }

    /// Compute overall health: fraction of snapshots with no violations.
    pub fn health(&self) -> f64 {
        if self.history.is_empty() {
            return 1.0;
        }
        let clean = self.history.iter().filter(|s| s.violations.is_empty()).count();
        clean as f64 / self.history.len() as f64
    }

    /// Total number of violations across all snapshots.
    pub fn total_violations(&self) -> usize {
        self.history.iter().map(|s| s.violations.len()).sum()
    }

    /// Generate a report of the monitoring session.
    pub fn report(&self) -> Report {
        let violated_law_indices: Vec<usize> = self
            .history
            .iter()
            .flat_map(|s| s.violations.iter().copied())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let mut violations = Vec::new();
        for &idx in &violated_law_indices {
            if let Some(law) = self.laws.get(idx) {
                let times: Vec<f64> = self
                    .history
                    .iter()
                    .filter(|s| s.violations.contains(&idx))
                    .map(|s| s.time)
                    .collect();
                let breaking_scale = self
                    .drift_detectors
                    .get(idx)
                    .and_then(|d| d.breaking_scale);
                violations.push(crate::report::Violation {
                    law_name: law.name.clone(),
                    law_index: idx,
                    first_violation: times.first().copied(),
                    last_violation: times.last().copied(),
                    violation_count: times.len(),
                    breaking_scale,
                });
            }
        }

        Report {
            total_ticks: self.history.len(),
            health: self.health(),
            total_violations: self.total_violations(),
            violations,
        }
    }
}

impl Default for ConservationMonitor {
    fn default() -> Self {
        Self::new()
    }
}
