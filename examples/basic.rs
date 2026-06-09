//! Basic noether-guard usage: monitor energy conservation in a simple simulation.
//!
//! Run with: cargo run --example basic

use noether_guard::ConservationMonitor;

fn main() {
    // Create a Hamiltonian (energy-only) monitor with tolerance 0.01
    let mut monitor = ConservationMonitor::hamiltonian(0.01);
    monitor.laws[0].initial_value = 100.0;

    // Simulate perfect energy conservation
    for i in 0..100 {
        let t = i as f64 * 0.01;
        monitor.tick(t, &[100.0]).unwrap();
    }

    println!("After 100 ticks (perfect conservation):");
    println!("  Health: {:.1}%", monitor.health() * 100.0);
    println!("  Violations: {}", monitor.total_violations());

    // Generate a report
    let report = monitor.report();
    println!("\n{}", report.to_text());
}
