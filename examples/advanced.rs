//! Advanced: Newtonian monitoring, custom symmetries, RG analysis.
//!
//! Run with: cargo run --example advanced

use noether_guard::{
    ConservationMonitor, ConservationLaw, ConservedQuantity,
    DriftDetector, RenormalizationGroup, Symmetry, SymmetryGroup,
};

fn main() {
    println!("=== Advanced NoetherGuard ===\n");

    // ── 1. Full Newtonian monitoring ──
    println!("1. Newtonian monitoring (7 values: E, px, py, pz, Lx, Ly, Lz)");
    let mut monitor = ConservationMonitor::newtonian(0.01);
    println!("   Expected values per tick: {}", monitor.expected_value_count());
    println!("   Laws: {} conservation laws", monitor.laws.len());
    for (i, law) in monitor.laws.iter().enumerate() {
        println!("     [{}] {} ({} components)",
            i, law.name, law.symmetry.component_count());
    }

    // Set initial values
    monitor.laws[0].initial_value = 10.0;  // energy
    monitor.laws[1].initial_value = 1.0;   // px
    monitor.laws[4].initial_value = 0.5;   // Lx

    // Run with perfect conservation
    for i in 0..100 {
        let t = i as f64 * 0.01;
        monitor.tick(t, &[10.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0]).unwrap();
    }
    println!("   Perfect conservation: health={:.1}%, violations={}",
        monitor.health() * 100.0, monitor.total_violations());

    // ── 2. Custom gauge symmetry ──
    println!("\n2. Custom gauge symmetries");
    let mut custom = ConservationMonitor::new();
    custom.add_law(ConservationLaw::new(
        Symmetry::Gauge(SymmetryGroup {
            name: "Electric Charge".to_string(),
            generators: 1,
        }),
        0.0,
        0.001,
    ));
    custom.add_law(ConservationLaw::new(
        Symmetry::Gauge(SymmetryGroup {
            name: "Baryon Number".to_string(),
            generators: 1,
        }),
        1.0,
        0.001,
    ));
    println!("   Custom laws:");
    for law in &custom.laws {
        println!("     {} (tolerance={})", law.name, law.tolerance);
    }

    // ── 3. ConservedQuantity tracking ──
    println!("\n3. ConservedQuantity time series");
    let mut q = ConservedQuantity::new("Energy");
    for i in 0..10 {
        let t = i as f64;
        let val = 100.0 + (i as f64 * 0.01).sin() * 0.5; // small oscillation
        q.record(t, val);
    }
    println!("   Data points: {}", q.len());
    println!("   Total drift: {:.4}", q.total_drift());
    println!("   Drift rate: {:.6}", q.current_drift_rate());

    // ── 4. Drift detector ──
    println!("\n4. Drift detector");
    let mut detector = DriftDetector::new(0, 0.5);
    for i in 0..20 {
        let t = i as f64 * 0.1;
        let drift = i as f64 * 0.05; // linearly increasing drift
        detector.record_drift(t, drift);
    }
    println!("   Is drifting: {}", detector.is_drifting());
    println!("   Max drift: {:.2}", detector.max_drift());
    println!("   First violation: t={:?}",
        detector.first_violation_time());
    println!("   Breaking scale: {:?}", detector.breaking_scale);

    // ── 5. RG analysis ──
    println!("\n5. Renormalization-group analysis");
    let data: Vec<(f64, f64)> = (0..100)
        .map(|i| (i as f64 * 0.01, i as f64 * 0.001)) // monotonically increasing
        .collect();
    let rg = RenormalizationGroup::new(&data);

    // Coarse-grain at different scales
    for block_size in [1, 2, 4, 8, 16] {
        let coarse = rg.coarse_grain(block_size);
        println!("   Block size {}: {} coarse points", block_size, coarse.len());
    }

    // Find breaking scale
    let breaking = rg.find_breaking_scale();
    println!("   Breaking scale: {:?}", breaking);

    // Beta function
    let beta = rg.beta_function(&[1, 2, 4, 8, 16]);
    println!("   Beta function:");
    for (scale, avg_drift) in &beta {
        println!("     scale={}: avg_drift={:.4}", scale, avg_drift);
    }

    // ── 6. Symmetry display ──
    println!("\n6. Symmetry information");
    let symmetries = [
        Symmetry::TimeTranslation,
        Symmetry::SpatialTranslation,
        Symmetry::Rotation,
        Symmetry::Gauge(SymmetryGroup {
            name: "Electric Charge".to_string(),
            generators: 1,
        }),
    ];
    for sym in &symmetries {
        println!("   {} → {} ({} components)",
            sym, sym.conserved_quantity(), sym.component_count());
    }

    // ── 7. Report with multiple violations ──
    println!("\n7. Multi-law monitoring with violations");
    let mut multi = ConservationMonitor::newtonian(0.001);
    // Energy drifts, momentum is conserved
    multi.laws[0].initial_value = 10.0;
    multi.laws[1].initial_value = 1.0;
    multi.laws[4].initial_value = 0.5;

    for i in 0..100 {
        let t = i as f64 * 0.01;
        let energy = 10.0 + i as f64 * 0.01; // drifting!
        multi.tick(t, &[energy, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0]).unwrap();
    }
    let report = multi.report();
    println!("{}", report.to_text());
}
