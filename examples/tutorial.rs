//! Tutorial: step-by-step conservation monitoring for a harmonic oscillator.
//!
//! Run with: cargo run --example tutorial

use noether_guard::ConservationMonitor;

fn main() {
    println!("=== NoetherGuard Tutorial ===\n");

    // Step 1: Create a monitor
    println!("Step 1: Create Hamiltonian monitor (tolerance=0.001)");
    let mut monitor = ConservationMonitor::hamiltonian(0.001);
    println!("  Expected values per tick: {}", monitor.expected_value_count());

    // Step 2: Simple harmonic oscillator with Euler method
    println!("\nStep 2: Simple harmonic oscillator (Euler method)");
    let dt = 0.01;
    let omega = 2.0;
    let mut x = 1.0_f64;
    let mut v = 0.0_f64;

    let e0 = 0.5 * v * v + 0.5 * omega * omega * x * x;
    monitor.laws[0].initial_value = e0;
    println!("  Initial energy: {:.4}", e0);

    // Step 3: Run simulation
    println!("\nStep 3: Run simulation (500 Euler steps)");
    for i in 0..500 {
        let t = i as f64 * dt;
        // Euler step (intentionally energy-drifting)
        v += -omega * omega * x * dt;
        x += v * dt;
        let energy = 0.5 * v * v + 0.5 * omega * omega * x * x;
        monitor.tick(t, &[energy]).unwrap();
    }

    // Step 4: Check results
    println!("\nStep 4: Results");
    println!("  Final energy: {:.4}", 0.5 * v * v + 0.5 * omega * omega * x * x);
    println!("  Health: {:.1}%", monitor.health() * 100.0);
    println!("  Total violations: {}", monitor.total_violations());

    // Step 5: Report
    println!("\nStep 5: Report");
    let report = monitor.report();
    println!("{}", report.to_text());

    // Step 6: JSON output
    println!("Step 6: JSON report (first 200 chars)");
    let json = report.to_json();
    println!("  {}...", &json[..json.len().min(200)]);

    // Step 7: Compare with symplectic (leapfrog) integrator
    println!("\nStep 7: Symplectic (leapfrog) comparison");
    let mut monitor2 = ConservationMonitor::hamiltonian(0.001);
    let mut x2 = 1.0_f64;
    let mut v2 = 0.0_f64;
    let e0_2 = 0.5 * v2 * v2 + 0.5 * omega * omega * x2 * x2;
    monitor2.laws[0].initial_value = e0_2;

    for i in 0..500 {
        let t = i as f64 * dt;
        // Leapfrog (symplectic — preserves energy on average)
        v2 += -omega * omega * x2 * dt / 2.0;
        x2 += v2 * dt;
        v2 += -omega * omega * x2 * dt / 2.0;
        let energy = 0.5 * v2 * v2 + 0.5 * omega * omega * x2 * x2;
        monitor2.tick(t, &[energy]).unwrap();
    }

    let report2 = monitor2.report();
    println!("  Leapfrog health: {:.1}%", report2.health * 100.0);
    println!("  Leapfrog violations: {}", report2.total_violations);
}
