# Contributing to noether-guard

Thank you for your interest in physics simulation verification!

## Getting Started

```bash
git clone https://github.com/SuperInstance/noether-guard.git
cd noether-guard
cargo test
```

## Architecture Decisions

### Why tick-based monitoring instead of automatic instrumentation?

Explicit `tick()` calls give the user full control over when and how often conservation is checked. This avoids the complexity of trait-based instrumentation and works with any simulation framework.

### Why RG analysis for breaking-scale detection?

The renormalization group provides a principled way to detect *at what scale* a signal emerges from noise. In physics, RG flow reveals whether a coupling constant grows or shrinks under coarse-graining. Here, the "coupling constant" is the drift rate, and the "scale" is the time resolution. The breaking scale tells you the characteristic time of the symmetry violation — useful for choosing integration step sizes.

### Why separate ConservationLaw and ConservedQuantity?

`ConservationLaw` is a *specification* (what should be conserved, with what tolerance). `ConservedQuantity` is a *measurement* (what was actually observed over time). The law checks the measurement; the quantity records the history.

### Why pre-built Newtonian and Hamiltonian monitors?

Most physics simulations fall into one of two categories:
1. **Hamiltonian**: only energy conservation matters (e.g., molecular dynamics)
2. **Newtonian**: energy + momentum + angular momentum (e.g., N-body problems)

Pre-built monitors handle these common cases. Custom monitors support gauge symmetries and other exotic conservation laws.

## How to Add a New Symmetry Type

1. Add the variant to the `Symmetry` enum in `symmetry.rs`
2. Implement `conserved_quantity()` and `component_count()`
3. Update `Display`
4. Write tests verifying the name and component count

## How to Add a New Analysis Method

The `renormalize` module can be extended with:
- **Fourier analysis**: detect periodic drift (common with symplectic integrators)
- **Lyapunov exponents**: measure chaos in the drift
- **CUSUM charts**: statistical process control for drift detection
- **Wavelet decomposition**: multi-scale drift analysis

## Testing

```bash
cargo test                    # All tests
cargo test test_symmetry      # Symmetry tests
cargo test test_monitor       # Monitor tests
cargo test test_drift         # Drift detector tests
cargo test test_rg            # RG analysis tests
cargo test test_report        # Report generation tests
```

The integration test `double_pendulum_energy_drift` is particularly important — it verifies that the Euler method (which is known to violate energy conservation) is correctly detected.

Key test patterns:
- Perfect conservation → 0 violations, health = 100%
- Known drift → correct number of violations
- Threshold boundary → violations start exactly when expected
- Euler vs symplectic → Euler detected, symplectic not (or much less)

## Code Style

- `cargo fmt` — no debate
- `cargo clippy` — warnings are errors in CI
- Doc comments on all `pub` items
- Physics terms should match standard physics notation

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` new features (new symmetries, new analyses)
- `fix:` bug fixes
- `docs:` documentation changes

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
