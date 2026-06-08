# noether-guard

[![crates.io](https://img.shields.io/crates/v/noether-guard.svg)](https://crates.io/crates/noether-guard)
[![docs.rs](https://docs.rs/noether-guard/badge.svg)](https://docs.rs/noether-guard)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## The Problem

Every physics simulation drifts. A molecular dynamics simulation that should conserve energy slowly heats up. An orbital mechanics integrator bleeds angular momentum. A multi-agent system that should conserve total "fleet energy" develops phantom forces.

Most simulation code doesn't check. By the time you notice the orbit is wrong, the conservation violation happened millions of timesteps ago. **Noether-guard** catches violations as they happen and — more importantly — tells you the *scale* at which your simulation breaks.

## The Idea

Emmy Noether proved that every continuous symmetry implies a conserved quantity:

| If your system is symmetric under... | ...then this is conserved |
|---|---|
| Time translation | Total energy |
| Spatial translation | Linear momentum (3 components) |
| Rotation | Angular momentum (3 components) |
| Gauge transformation | Charge / particle number |

This crate instruments your simulation loop to monitor any or all of these. But it goes further: it uses **renormalization-group-style coarse-graining** to find the breaking scale. If your simulation conserves energy at Δt = 0.001 but violates it at Δt = 0.01, noether-guard tells you exactly where the transition happens and how fast the violation grows.

## How It Works

You create a `ConservationMonitor`, add the conservation laws you care about, and call `.tick()` at each timestep with the current values:

```rust
use noether_guard::ConservationMonitor;

// Preset: Newtonian mechanics = energy + 3 momentum + 3 angular momentum
let mut monitor = ConservationMonitor::newtonian(tolerance);
monitor.tick(t, &[energy, px, py, pz, lx, ly, lz]).unwrap();
```

Internally, each tick:
1. **DriftDetector** records the deviation from the initial value
2. Flags violations that exceed the tolerance
3. Optionally, **RenormalizationGroup** coarse-grains the drift history at multiple scales (block-averaging, similar to RG flow in statistical mechanics) and computes "beta functions" — how the violation scales changes as you zoom out

The RG analysis is the key insight: a violation that appears random at fine scales but systematic at coarse scales indicates a *structural* bug (wrong integrator, missing force term). A violation that's consistent across scales indicates a *numerical* issue (timestep too large, precision loss).

## Breaking Scale Detection

```rust
use noether_guard::{ConservationMonitor, RenormalizationGroup};

let rg = monitor.renormalize();
for (scale, beta) in &rg.beta_functions {
    println!("Scale {}: drift grows as {:.4}", scale, beta);
}
// If beta jumps sharply at some scale, that's your breaking point
```

## Presets

| Preset | Laws | Components |
|---|---|---|
| `newtonian()` | Energy + Momentum + Angular Momentum | 7 |
| `hamiltonian()` | Energy only | 1 |
| Custom | Any `ConservationLaw` you define | any |

## Custom Laws

```rust
use noether_guard::{ConservationMonitor, ConservationLaw, Symmetry};

let mut monitor = ConservationMonitor::new();
monitor.add_law(ConservationLaw::new(
    Symmetry::Custom("fleet_gamma".into()),
    initial_value,
    tolerance,
));
```

## Module Map

| Module | What's in it |
|---|---|
| `conservation` | `ConservationLaw`, `ConservedQuantity` — what you're monitoring |
| `monitor` | `ConservationMonitor` — the main tick-based API |
| `drift` | `DriftDetector` — per-law drift tracking and violation flagging |
| `renormalize` | `RenormalizationGroup` — multi-scale breaking-scale analysis |
| `symmetry` | `Symmetry`, `SymmetryGroup` — classify which Noether symmetry |
| `report` | `Report`, `Violation` — human-readable and JSON output |
| `error` | `NoetherError` |

## When To Use This

- You're writing a physics simulation and want conservation-law violations detected immediately
- You're debugging an integrator and need to know *where* it breaks, not just *that* it breaks
- You're running multi-agent fleet simulations with conservation constraints (total fleet energy, total charge, etc.)
- You need a CI check: "does my simulation conserve energy to 6 significant figures?"

## Links

- [Documentation](https://docs.rs/noether-guard)
- [Repository](https://github.com/SuperInstance/noether-guard)
- [crates.io](https://crates.io/crates/noether-guard)

## License

MIT
