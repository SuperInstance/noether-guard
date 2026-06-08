/// Renormalization-group-style analysis: coarse-grain the time series at
/// different scales to find where symmetry breaking first appears.
#[derive(Debug, Clone)]
pub struct RenormalizationGroup<'a> {
    data: &'a [(f64, f64)],
}

impl<'a> RenormalizationGroup<'a> {
    /// Create an RG analyser for a drift time series `(time, drift)`.
    pub fn new(data: &'a [(f64, f64)]) -> Self {
        Self { data }
    }

    /// Coarse-grain the data by averaging over blocks of `block_size`.
    pub fn coarse_grain(&self, block_size: usize) -> Vec<(f64, f64)> {
        if block_size == 0 || self.data.is_empty() {
            return Vec::new();
        }
        self.data
            .chunks(block_size)
            .map(|chunk| {
                let n = chunk.len() as f64;
                let avg_t: f64 = chunk.iter().map(|(t, _)| t).sum::<f64>() / n;
                let avg_d: f64 = chunk.iter().map(|(_, d)| d).sum::<f64>() / n;
                (avg_t, avg_d)
            })
            .collect()
    }

    /// Find the coarse-graining scale (as block size) where drift becomes
    /// monotonically increasing, indicating symmetry breaking.
    pub fn find_breaking_scale(&self) -> Option<f64> {
        if self.data.len() < 4 {
            return None;
        }

        // Try increasingly coarse scales
        for block_log in 0.. {
            let block_size = 1usize << block_log;
            if block_size >= self.data.len() {
                break;
            }
            let coarse = self.coarse_grain(block_size);
            if coarse.len() < 3 {
                continue;
            }
            // Check if drift is monotonically increasing
            let monotonic = coarse
                .windows(2)
                .all(|w| w[1].1 >= w[0].1 - f64::EPSILON);
            let exceeds = coarse.iter().any(|(_, d)| *d > 0.0);

            if monotonic && exceeds {
                // Return the time scale (average time span per block)
                if let (Some(first), Some(last)) = (coarse.first(), coarse.last()) {
                    let span = last.0 - first.0;
                    return Some(span / coarse.len() as f64);
                }
            }
        }
        None
    }

    /// Compute the "beta function" — how the drift coupling changes with scale.
    pub fn beta_function(&self, scales: &[usize]) -> Vec<(usize, f64)> {
        scales
            .iter()
            .map(|&block_size| {
                let coarse = self.coarse_grain(block_size);
                if coarse.len() < 2 {
                    return (block_size, 0.0);
                }
                // Rate of change of average drift between consecutive scales
                let total_drift: f64 = coarse.iter().map(|(_, d)| d).sum();
                let avg_drift = total_drift / coarse.len() as f64;
                (block_size, avg_drift)
            })
            .collect()
    }
}
