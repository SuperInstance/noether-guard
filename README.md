# noether-guard

**A physics linter for simulations — verify that your numerical integrators preserve conservation laws, and pinpoint exactly where they break.**

## The Problem

Every physics simulation has conserved quantities: energy, momentum, angular momentum. These are guaranteed by Noether's theorem: every continuous symmetry implies a conserved quantity. But numerical integrators (Euler, Verlet, RK4) **don't** preserve these quantities — they drift. Energy leaks. Momentum shifts. Angular momentum wobbles.

The problem isn't that drift happens — it's that **you can't see it**. Most simulation code doesn't check conservation laws. By the time you notice something's wrong, the simulation has been diverging for thousands of steps, and you have no idea where or why.

## The Key Insight

**Noether's theorem is a compile-time concept applied at runtime.** You declare what symmetries your system has (time translation → energy, spatial translation → momentum, rotation → angular momentum), and NoetherGuard instruments your simulation to check them every step. But it goes further:

When a conservation law is violated, the **drift detector** records when it started, how fast it's growing, and — using **renormalization-group-style scale analysis** — at what time scale the symmetry breaking first appears. This tells you not just *that* your integrator is wrong, but *where* and *how fast*.

The RG analysis works by coarse-graining the drift time series at different scales (like blocking renormalization in statistical physics). At fine scales, noise obscures the drift. At coarse scales, the drift signal emerges. The scale where it first becomes monotonically increasing is the **breaking scale** — the characteristic time of the symmetry violation.

This crate implements:
- **Symmetry types**: TimeTranslation, SpatialTranslation, Rotation, Gauge (custom)
- **Conservation monitoring**: tick-by-tick tracking with violation detection
- **Drift detection**: threshold-based alerts with breaking-scale analysis
- **RG analysis**: coarse-graining at multiple scales to find where drift starts
- **Reporting**: text and JSON reports with per-law violation details
- **Pre-built monitors**: Newtonian (7 values: E + p⃗ + L⃗) and Hamiltonian (energy only)

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                 ConservationMonitor                       │
│ (laws, history of snapshots, drift detectors, reporting) │
└─────────────────────┬───────────────────────────────────┘
                      │
     ┌────────────────┼────────────────┐
     │                │                │
┌────▼─────┐  ┌──────▼──────┐  ┌──────▼──────┐
│ Symmetry │  │Conservation │  │    Drift    │
│          │  │    Law      │  │  Detector   │
│ TimeTrans│  │(symmetry,   │  │(threshold,  │
│ SpaceTrans│ │ initial_val,│  │ breaking_   │
│ Rotation │  │ tolerance)  │  │ scale,      │
│ Gauge    │  │             │  │ history)    │
└──────────┘  └─────────────┘  └──────┬──────┘
                                       │
                              ┌────────▼────────┐
                              │ Renormalization │
                              │     Group       │
                              │ (coarse-grain,  │
                              │  breaking scale,│
                              │  beta function) │
                              └─────────────────┘
                                       │
                              ┌────────▼────────┐
                              │     Report      │
                              │ (text/JSON,     │
                              │  violations,    │
                              │  health %)      │
                              └─────────────────┘
```

### Module Overview

| Module | Purpose |
|--------|---------|
| `symmetry` | Symmetry types → conserved quantities (Noether's theorem) |
| `conservation` | ConservationLaw checking, ConservedQuantity time series |
| `monitor` | ConservationMonitor — the main API, tick-based tracking |
| `drift` | DriftDetector — threshold alerts with RG analysis |
| `renormalize` | RG-style coarse-graining and breaking-scale detection |
| `report` | Report generation (text and JSON) |
| `error` | Error types |

## The Math: Noether's Theorem

### The Theorem

> **For every continuous symmetry of the action, there exists a conserved quantity.**

| Symmetry | Conserved Quantity | Components |
|----------|-------------------|------------|
| Time translation (∂L/∂t = 0) | Energy (H) | 1 scalar |
| Spatial translation (∂L/∂x⃗ = 0) | Linear momentum (p⃗) | 3 components |
| Rotation (∂L/∂θ⃗ = 0) | Angular momentum (L⃗) | 3 components |
| Gauge symmetry | Charge (Q) | 1+ per gauge group |

### Why Numerical Integrators Break Conservation

Consider a simple harmonic oscillator with Euler integration:

```
v_{n+1} = v_n - ω²x_n · dt
x_{n+1} = x_n + v_{n+1} · dt
```

The Euler method has **energy drift**: H(t) increases monotonically. After 500 steps with dt=0.01, energy can grow by 10%+. This is because Euler doesn't preserve the symplectic structure of Hamiltonian mechanics.

**Symplectic integrators** (Verlet, leapfrog) preserve energy on average but oscillate around the true value. **Noether-guard** detects both types of drift.

### Renormalization-Group Analysis

The RG analysis answers: "at what time scale does the drift become systematic?"

1. Coarse-grain the drift time series at scales 1, 2, 4, 8, 16, ...
2. At each scale, check if the coarse-grained drift is monotonically increasing
3. The first scale where monotonicity appears is the **breaking scale**

This is analogous to block-spin renormalization in statistical physics: you're "integrating out" short-time fluctuations to reveal the long-time drift trend.

## Quick Start

```rust
use noether_guard::ConservationMonitor;

// Pre-built Hamiltonian monitor (energy only)
let mut monitor = ConservationMonitor::hamiltonian(0.01);

// Set initial energy
monitor.laws[0].initial_value = 100.0;

// Tick with energy values
monitor.tick(0.0, &[100.0]).unwrap();
monitor.tick(0.1, &[100.005]).unwrap();
monitor.tick(0.2, &[99.998]).unwrap();

println!("Health: {:.1}%", monitor.health() * 100.0);
println!("Violations: {}", monitor.total_violations());
```

## Newtonian Monitoring

For full Newtonian mechanics (energy + momentum + angular momentum):

```rust
use noether_guard::ConservationMonitor;

let mut monitor = ConservationMonitor::newtonian(0.01);
// Expects 7 values: [E, px, py, pz, Lx, Ly, Lz]

monitor.laws[0].initial_value = 10.0; // energy
monitor.laws[1].initial_value = 1.0;  // px
monitor.laws[2].initial_value = 0.0;  // py
monitor.laws[3].initial_value = 0.0;  // pz
monitor.laws[4].initial_value = 0.5;  // Lx
monitor.laws[5].initial_value = 0.0;  // Ly
monitor.laws[6].initial_value = 0.0;  // Lz

// Tick with perfectly conserved values
for i in 0..100 {
    let t = i as f64 * 0.01;
    monitor.tick(t, &[10.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0]).unwrap();
}

assert_eq!(monitor.total_violations(), 0);
```

## Custom Conservation Laws

```rust
use noether_guard::{
    ConservationMonitor, ConservationLaw, Symmetry, SymmetryGroup,
};

let mut monitor = ConservationMonitor::new();

// Energy conservation
monitor.add_law(ConservationLaw::new(
    Symmetry::TimeTranslation,
    100.0,
    0.1,
));

// Custom gauge symmetry (e.g., electric charge)
monitor.add_law(ConservationLaw::new(
    Symmetry::Gauge(SymmetryGroup {
        name: "Electric Charge".to_string(),
        generators: 1,
    }),
    0.0,  // net charge
    0.001, // tight tolerance
));
```

## Detecting Euler Drift

```rust
use noether_guard::ConservationMonitor;

let mut monitor = ConservationMonitor::hamiltonian(0.001);
let dt = 0.01;
let mut x = 1.0_f64;
let mut v = 0.0_f64;
let omega = 2.0_f64;

let e0 = 0.5 * v * v + 0.5 * omega * omega * x * x;
monitor.laws[0].initial_value = e0;

for i in 0..500 {
    let t = i as f64 * dt;
    // Euler step (energy-drifting!)
    v += -omega * omega * x * dt;
    x += v * dt;
    let energy = 0.5 * v * v + 0.5 * omega * omega * x * x;
    monitor.tick(t, &[energy]).unwrap();
}

let report = monitor.report();
println!("{}", report.to_text());
// Euler method will show energy violations
```

## Reporting

```rust
let report = monitor.report();

// Human-readable text
println!("{}", report.to_text());
// Output:
// === NoetherGuard Report ===
// Ticks:    500
// Health:   87.4%
// Violations: 63
//
// Violations:
//   [0] Energy — 63 violations (t=0.050..4.990) breaking_scale=0.2500

// Machine-readable JSON
let json = report.to_json();
// {"total_ticks":500,"health":0.874,"total_violations":63,"violations":[...]}
```

## Symmetry Types

```rust
use noether_guard::Symmetry;

// Built-in symmetries
let time = Symmetry::TimeTranslation;       // → Energy (1 component)
let space = Symmetry::SpatialTranslation;    // → Momentum (3 components)
let rot = Symmetry::Rotation;                // → Angular Momentum (3 components)

// Custom gauge symmetry
let charge = Symmetry::Gauge(noether_guard::SymmetryGroup {
    name: "Baryon Number".to_string(),
    generators: 1,
});

println!("{}: {} components", time, time.component_count());
println!("{}: {} components", space, space.component_count());
println!("{}: {} components", rot, rot.component_count());
```

## Performance

- **Per-tick overhead**: O(k) where k = number of conservation laws
- **RG analysis**: O(n log n) triggered only on threshold violation
- **Reporting**: O(n) where n = history length
- **Memory**: O(n) for history storage

The monitor is designed to be zero-cost when there are no violations — the per-tick check is just a comparison against a tolerance. RG analysis only kicks in when a violation is detected.

## Comparison

| Feature | noether-guard | Manual assertions | Checkpoint comparison |
|---------|-------------|-------------------|----------------------|
| Symmetry-aware | ✅ Knows which quantity maps to which symmetry | ❌ | ❌ |
| Automatic drift detection | ✅ | ❌ | Partial |
| Breaking scale | ✅ RG analysis | ❌ | ❌ |
| Per-law tracking | ✅ | Manual | Manual |
| Trend analysis | ✅ | ❌ | ❌ |
| Reports | ✅ Text + JSON | ❌ | ❌ |
| Custom laws | ✅ Gauge symmetries | Manual | Manual |

## SuperInstance Ecosystem

`noether-guard` is the conservation backbone for:
- `spreadsheet-engine` — γ + η = budget conservation for agent cells
- `lotka-beats` — population conservation checks in ecosystem simulations
- `tropical-synth` — energy-like conservation in tropical polynomial evaluation

## License

MIT
