use crate::symmetry::Symmetry;

/// A conservation law derived from a symmetry via Noether's theorem.
#[derive(Debug, Clone)]
pub struct ConservationLaw {
    pub symmetry: Symmetry,
    pub name: String,
    pub initial_value: f64,
    pub tolerance: f64,
}

impl ConservationLaw {
    /// Create a new conservation law.
    pub fn new(symmetry: Symmetry, initial_value: f64, tolerance: f64) -> Self {
        Self {
            name: symmetry.conserved_quantity().to_string(),
            symmetry,
            initial_value,
            tolerance,
        }
    }

    /// Check whether `value` is conserved within tolerance.
    pub fn is_conserved(&self, value: f64) -> bool {
        (value - self.initial_value).abs() <= self.tolerance
    }

    /// Compute the absolute drift from the initial value.
    pub fn drift(&self, value: f64) -> f64 {
        (value - self.initial_value).abs()
    }

    /// Compute the drift rate given a previous drift and elapsed time.
    pub fn drift_rate(&self, previous_drift: f64, dt: f64) -> f64 {
        if dt > 0.0 {
            (self.initial_value - previous_drift).abs() / dt
        } else {
            0.0
        }
    }
}

/// Tracks a single conserved quantity over time.
#[derive(Debug, Clone)]
pub struct ConservedQuantity {
    pub name: String,
    pub values: Vec<(f64, f64)>,
    pub drift_rate: f64,
}

impl ConservedQuantity {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            values: Vec::new(),
            drift_rate: 0.0,
        }
    }

    /// Record a (time, value) pair.
    pub fn record(&mut self, time: f64, value: f64) {
        self.values.push((time, value));
        if self.values.len() >= 2 {
            let (t0, v0) = self.values[self.values.len() - 2];
            let (t1, v1) = self.values[self.values.len() - 1];
            let dt = t1 - t0;
            if dt > 0.0 {
                self.drift_rate = (v1 - v0) / dt;
            }
        }
    }

    /// Current drift rate.
    pub fn current_drift_rate(&self) -> f64 {
        self.drift_rate
    }

    /// Total accumulated drift from first recorded value.
    pub fn total_drift(&self) -> f64 {
        if self.values.len() < 2 {
            return 0.0;
        }
        let first = self.values.first().unwrap().1;
        let last = self.values.last().unwrap().1;
        (last - first).abs()
    }

    /// Number of recorded data points.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether any data has been recorded.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
