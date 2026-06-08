//! # noether-guard
//!
//! A static analysis tool for physics simulations that verifies conservation laws.
//!
//! Noether's theorem states that every continuous symmetry implies a conserved quantity:
//! - **Time translation symmetry** → energy conservation
//! - **Spatial translation symmetry** → momentum conservation
//! - **Rotational symmetry** → angular momentum conservation
//!
//! NoetherGuard instruments simulation code to check these at runtime and pinpoint
//! where they break, using renormalization-group-style scale analysis.
//!
//! # Quick start
//!
//! ```
//! use noether_guard::ConservationMonitor;
//!
//! let mut monitor = ConservationMonitor::newtonian(0.01);
//! // tick with [energy, px, py, pz, lx, ly, lz]
//! monitor.tick(0.0, &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
//! monitor.tick(0.1, &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).unwrap();
//! let report = monitor.report();
//! println!("{report}");
//! ```

pub mod conservation;
pub mod drift;
pub mod error;
pub mod monitor;
pub mod renormalize;
pub mod report;
pub mod symmetry;

pub use conservation::{ConservationLaw, ConservedQuantity};
pub use drift::DriftDetector;
pub use error::{Error, Result};
pub use monitor::{ConservationMonitor, MonitorSnapshot};
pub use renormalize::RenormalizationGroup;
pub use report::{Report, Violation};
pub use symmetry::{Symmetry, SymmetryGroup};

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Symmetry tests ----

    #[test]
    fn symmetry_conserved_quantity_names() {
        assert_eq!(Symmetry::TimeTranslation.conserved_quantity(), "Energy");
        assert_eq!(Symmetry::SpatialTranslation.conserved_quantity(), "Momentum");
        assert_eq!(Symmetry::Rotation.conserved_quantity(), "Angular Momentum");
    }

    #[test]
    fn symmetry_component_counts() {
        assert_eq!(Symmetry::TimeTranslation.component_count(), 1);
        assert_eq!(Symmetry::SpatialTranslation.component_count(), 3);
        assert_eq!(Symmetry::Rotation.component_count(), 3);
    }

    #[test]
    fn gauge_symmetry() {
        let g = SymmetryGroup {
            name: "Charge".to_string(),
            generators: 1,
        };
        let sym = Symmetry::Gauge(g);
        assert_eq!(sym.conserved_quantity(), "Charge");
        assert_eq!(sym.component_count(), 1);
    }

    #[test]
    fn symmetry_display() {
        assert!(format!("{}", Symmetry::TimeTranslation).contains("Energy"));
    }

    // ---- ConservationLaw tests ----

    #[test]
    fn conservation_law_is_conserved() {
        let law = ConservationLaw::new(Symmetry::TimeTranslation, 100.0, 0.1);
        assert!(law.is_conserved(100.05));
        assert!(!law.is_conserved(100.2));
    }

    #[test]
    fn conservation_law_drift() {
        let law = ConservationLaw::new(Symmetry::TimeTranslation, 10.0, 1.0);
        assert!((law.drift(12.0) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn conservation_law_drift_rate() {
        let law = ConservationLaw::new(Symmetry::TimeTranslation, 10.0, 1.0);
        let rate = law.drift_rate(10.0, 1.0);
        assert!((rate - 0.0).abs() < 1e-10);
    }

    // ---- ConservedQuantity tests ----

    #[test]
    fn conserved_quantity_record_and_drift() {
        let mut q = ConservedQuantity::new("Energy");
        q.record(0.0, 100.0);
        q.record(1.0, 101.0);
        q.record(2.0, 103.0);
        assert!((q.total_drift() - 3.0).abs() < 1e-10);
        assert!((q.current_drift_rate() - 2.0).abs() < 1e-10);
        assert_eq!(q.len(), 3);
    }

    #[test]
    fn conserved_quantity_empty() {
        let q = ConservedQuantity::new("Energy");
        assert!(q.is_empty());
        assert_eq!(q.total_drift(), 0.0);
    }

    // ---- ConservationMonitor tests ----

    #[test]
    fn monitor_newtonian_expected_values() {
        let m = ConservationMonitor::newtonian(0.01);
        // 1 + 3 + 3 = 7
        assert_eq!(m.expected_value_count(), 7);
    }

    #[test]
    fn monitor_hamiltonian_expected_values() {
        let m = ConservationMonitor::hamiltonian(0.01);
        assert_eq!(m.expected_value_count(), 1);
    }

    #[test]
    fn monitor_tick_clean() {
        let mut m = ConservationMonitor::hamiltonian(0.1);
        m.laws[0].initial_value = 100.0;
        m.tick(0.0, &[100.0]).unwrap();
        m.tick(1.0, &[100.05]).unwrap();
        assert_eq!(m.history.len(), 2);
        assert!(m.history.iter().all(|s| s.violations.is_empty()));
        assert!((m.health() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn monitor_tick_violation() {
        let mut m = ConservationMonitor::hamiltonian(0.1);
        m.laws[0].initial_value = 100.0;
        m.tick(0.0, &[100.0]).unwrap();
        m.tick(1.0, &[120.0]).unwrap();
        assert_eq!(m.history[1].violations, vec![0]);
        assert!((m.health() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn monitor_value_mismatch() {
        let mut m = ConservationMonitor::hamiltonian(0.1);
        let result = m.tick(0.0, &[1.0, 2.0]);
        assert!(result.is_err());
    }

    #[test]
    fn monitor_custom() {
        let laws = vec![ConservationLaw::new(Symmetry::TimeTranslation, 50.0, 0.5)];
        let m = ConservationMonitor::custom(laws);
        assert_eq!(m.expected_value_count(), 1);
        assert_eq!(m.laws.len(), 1);
    }

    #[test]
    fn monitor_add_law() {
        let mut m = ConservationMonitor::new();
        m.add_law(ConservationLaw::new(Symmetry::TimeTranslation, 0.0, 0.01));
        m.add_law(ConservationLaw::new(Symmetry::SpatialTranslation, 0.0, 0.01));
        assert_eq!(m.expected_value_count(), 4);
    }

    #[test]
    fn monitor_default() {
        let m = ConservationMonitor::default();
        assert_eq!(m.laws.len(), 0);
    }

    // ---- DriftDetector tests ----

    #[test]
    fn drift_detector_no_drift() {
        let mut d = DriftDetector::new(0, 1.0);
        d.record_drift(0.0, 0.1);
        d.record_drift(1.0, 0.2);
        assert!(!d.is_drifting());
        assert_eq!(d.sample_count(), 2);
    }

    #[test]
    fn drift_detector_with_drift() {
        let mut d = DriftDetector::new(0, 0.5);
        d.record_drift(0.0, 0.1);
        d.record_drift(1.0, 0.6);
        assert!(d.is_drifting());
        assert!((d.max_drift() - 0.6).abs() < 1e-10);
    }

    #[test]
    fn drift_detector_first_violation() {
        let mut d = DriftDetector::new(0, 0.5);
        d.record_drift(0.0, 0.1);
        d.record_drift(1.0, 0.6);
        assert!((d.first_violation_time().unwrap() - 1.0).abs() < 1e-10);
    }

    // ---- RenormalizationGroup tests ----

    #[test]
    fn rg_coarse_grain() {
        let data: Vec<(f64, f64)> = (0..10)
            .map(|i| (i as f64, (i % 2) as f64))
            .collect();
        let rg = RenormalizationGroup::new(&data);
        let coarse = rg.coarse_grain(2);
        assert_eq!(coarse.len(), 5);
        // Each pair averaged: (0+1)/2=0.5, (1+0)/2=0.5, etc.
        assert!((coarse[0].1 - 0.5).abs() < 1e-10);
    }

    #[test]
    fn rg_coarse_grain_empty() {
        let rg = RenormalizationGroup::new(&[]);
        let coarse = rg.coarse_grain(2);
        assert!(coarse.is_empty());
    }

    #[test]
    fn rg_find_breaking_scale() {
        // Monotonically increasing drift
        let data: Vec<(f64, f64)> = (0..20).map(|i| (i as f64, i as f64 * 0.1)).collect();
        let rg = RenormalizationGroup::new(&data);
        let scale = rg.find_breaking_scale();
        assert!(scale.is_some());
    }

    #[test]
    fn rg_no_breaking_constant() {
        let data: Vec<(f64, f64)> = (0..20).map(|i| (i as f64, 0.0)).collect();
        let rg = RenormalizationGroup::new(&data);
        let scale = rg.find_breaking_scale();
        // Constant zero drift — no breaking
        assert!(scale.is_none());
    }

    #[test]
    fn rg_beta_function() {
        let data: Vec<(f64, f64)> = (0..10).map(|i| (i as f64, i as f64)).collect();
        let rg = RenormalizationGroup::new(&data);
        let beta = rg.beta_function(&[1, 2, 4]);
        assert_eq!(beta.len(), 3);
    }

    // ---- Report tests ----

    #[test]
    fn report_text_clean() {
        let report = Report {
            total_ticks: 100,
            health: 1.0,
            total_violations: 0,
            violations: vec![],
        };
        let text = report.to_text();
        assert!(text.contains("satisfied"));
    }

    #[test]
    fn report_text_with_violations() {
        let report = Report {
            total_ticks: 100,
            health: 0.9,
            total_violations: 10,
            violations: vec![Violation {
                law_name: "Energy".to_string(),
                law_index: 0,
                first_violation: Some(5.0),
                last_violation: Some(50.0),
                violation_count: 10,
                breaking_scale: Some(0.25),
            }],
        };
        let text = report.to_text();
        assert!(text.contains("Energy"));
        assert!(text.contains("90.0%"));
    }

    #[test]
    fn report_json_roundtrip() {
        let report = Report {
            total_ticks: 50,
            health: 1.0,
            total_violations: 0,
            violations: vec![],
        };
        let json = report.to_json();
        let parsed: Report = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.total_ticks, 50);
    }

    // ---- Integration test: double pendulum-like scenario ----

    #[test]
    fn double_pendulum_energy_drift() {
        // Simulate a simple harmonic oscillator with Euler method (which drifts)
        let mut monitor = ConservationMonitor::hamiltonian(0.001);
        let dt = 0.01;
        let mut x = 1.0_f64;
        let mut v = 0.0_f64;
        let omega = 2.0_f64;

        // Initial energy
        let e0 = 0.5 * v * v + 0.5 * omega * omega * x * x;
        monitor.laws[0].initial_value = e0;

        for i in 0..500 {
            let t = i as f64 * dt;
            // Euler step (energy-drifting)
            v += -omega * omega * x * dt;
            x += v * dt;
            let energy = 0.5 * v * v + 0.5 * omega * omega * x * x;
            monitor.tick(t, &[energy]).unwrap();
        }

        let report = monitor.report();
        // Euler method on SHO should drift significantly
        assert!(report.total_violations > 0, "Euler method should violate energy conservation");
    }

    #[test]
    fn newtonian_system_conserved() {
        let mut monitor = ConservationMonitor::newtonian(0.1);
        // Set initial values to match what we'll feed
        monitor.laws[0].initial_value = 10.0; // energy
        monitor.laws[1].initial_value = 1.0;  // momentum (x)
        monitor.laws[2].initial_value = 0.5;  // angular momentum (x)
        // Perfect conservation: all values constant
        for i in 0..100 {
            let t = i as f64 * 0.01;
            monitor.tick(t, &[10.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0]).unwrap();
        }
        assert!((monitor.health() - 1.0).abs() < 1e-10);
        assert_eq!(monitor.total_violations(), 0);
    }
}
