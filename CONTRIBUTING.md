# Contributing to noether-guard

Thank you for your interest in contributing! This guide covers the basics.

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Running Examples

```bash
cargo run --example basic
```

## Code Quality

Before submitting a PR, ensure all checks pass:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes with clear commit messages
4. Ensure CI passes (fmt, clippy, test)
5. Open a pull request against `main`

## Adding New Symmetry Types

When adding a new symmetry, implement:
- A new variant in `Symmetry` enum
- The `conserved_quantity()` and `component_count()` methods
- A corresponding test

## Adding New Analysis Methods

New drift/analysis methods should:
- Work with the existing `ConservationMonitor` pipeline
- Include unit tests with known-good physics scenarios
- Be documented with mathematical references
