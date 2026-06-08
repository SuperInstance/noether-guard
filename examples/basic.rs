use noether_guard::{ConservationMonitor, Symmetry, ConservationLaw};

fn main() {
    // Monitor a Hamiltonian system (energy conservation only).
    let mut monitor = ConservationMonitor::hamiltonian(0.001);

    // Simulate a simple harmonic oscillator using Euler integration.
    // Euler's method is NOT symplectic, so energy will drift — NoetherGuard catches it.
    let dt = 0.01;
    let omega = 2.0_f64;
    let mut x = 1.0_f64;
    let mut v = 0.0_f64;

    // Initial energy: E = ½v² + ½ω²x²
    let e0 = 0.5 * v * v + 0.5 * omega * omega * x * x;
    monitor.laws[0].initial_value = e0;

    for i in 0..200 {
        let t = i as f64 * dt;
        // Euler step (intentionally non-symplectic to demonstrate drift detection)
        v += -omega * omega * x * dt;
        x += v * dt;
        let energy = 0.5 * v * v + 0.5 * omega * omega * x * x;
        monitor.tick(t, &[energy]).unwrap();
    }

    let report = monitor.report();
    println!("=== NoetherGuard Drift Detection ===");
    println!("{}", report.to_text());

    // Now try a symplectic (leapfrog) integrator for comparison
    let mut monitor2 = ConservationMonitor::hamiltonian(0.001);
    let mut x = 1.0_f64;
    let mut v = 0.0_f64;
    let e0 = 0.5 * v * v + 0.5 * omega * omega * x * x;
    monitor2.laws[0].initial_value = e0;

    for i in 0..200 {
        let t = i as f64 * dt;
        // Leapfrog / Störmer-Verlet (symplectic)
        v += -omega * omega * x * dt / 2.0;
        x += v * dt;
        v += -omega * omega * x * dt / 2.0;
        let energy = 0.5 * v * v + 0.5 * omega * omega * x * x;
        monitor2.tick(t, &[energy]).unwrap();
    }

    let report2 = monitor2.report();
    println!("=== Symplectic Integrator ===");
    println!("{}", report2.to_text());
}
