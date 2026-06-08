# noether-guard

[![crates.io](https://img.shields.io/crates/v/noether-guard.svg)](https://crates.io/crates/noether-guard)
[![docs.rs](https://docs.rs/noether-guard/badge.svg)](https://docs.rs/noether-guard)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Static analysis for physics simulations — verify conservation laws at runtime.**

Noether's theorem states that every continuous symmetry of a system implies a
conserved quantity:

| Symmetry | Conserved Quantity |
|---|---|
| Time translation | Energy |
| Spatial translation | Momentum |
| Rotation | Angular Momentum |
| Gauge | Charge |

`noether-guard` instruments your simulation to check these conservation laws at
every timestep, pinpoint exactly where they break, and analyze the scale at which
drift becomes significant using renormalization-group-style coarse-graining.

## Features

- **Conservation monitoring** — track energy, momentum, angular momentum, and
  custom conserved quantities across simulation timesteps
- **Prebuilt presets** — `ConservationMonitor::newtonian()` for full Newtonian
  mechanics (7 components), `ConservationMonitor::hamiltonian()` for Hamiltonian
  systems (energy only)
- **Drift detection** — `DriftDetector` records drift samples, flags violations,
  and reports the first violation time
- **Renormalization group analysis** — `RenormalizationGroup` coarse-grains drift
  data across scales, computes beta functions, and finds the breaking scale where
  conservation fails
- **Rich reporting** — text and JSON reports with per-law violation counts,
  health scores (0–1), and breaking scales
- **Custom symmetry groups** — define gauge symmetries with arbitrary generators
- **Zero-allocation monitoring** — lightweight tick-based API for hot loops

## Quick Start

```rust
use noether_guard::ConservationMonitor;

// Monitor a Newtonian system (energy + momentum + angular momentum = 7 values)
let mut monitor = ConservationMonitor::newtonian(0.01);

// Feed [energy, px, py, pz, lx, ly, lz] at each timestep
monitor.tick(0.0, &[10.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0]).unwrap();
monitor.tick(0.1, &[10.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0]).unwrap();

// Check system health
println!("Health: {:.1}%", monitor.health() * 100.0);
println!("Violations: {}", monitor.total_violations());

// Full report
let report = monitor.report();
println!("{report}");
```

## Custom Conservation Laws

```rust
use noether_guard::{ConservationMonitor, ConservationLaw, Symmetry};

let mut monitor = ConservationMonitor::new();
monitor.add_law(ConservationLaw::new(Symmetry::TimeTranslation, 100.0, 0.05));
monitor.add_law(ConservationLaw::new(Symmetry::SpatialTranslation, 0.0, 0.01));
```

## Module Overview

| Module | Description |
|---|---|
| `conservation` | `ConservationLaw` and `ConservedQuantity` types |
| `monitor` | `ConservationMonitor` — main tick-based API |
| `drift` | `DriftDetector` — per-law drift tracking |
| `renormalize` | `RenormalizationGroup` — multi-scale drift analysis |
| `symmetry` | `Symmetry` and `SymmetryGroup` — symmetry classification |
| `report` | `Report` and `Violation` — text/JSON output |
| `error` | Error types |

## Links

- [Documentation](https://docs.rs/noether-guard)
- [Repository](https://github.com/nightshift-crates/noether-guard)
- [Crates.io](https://crates.io/crates/noether-guard)

## License

MIT
