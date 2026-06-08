# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-08

### Added
- Core conservation law verification via Noether's theorem
- Symmetry types: time translation (energy), spatial translation (momentum), rotation (angular momentum), gauge
- `ConservationMonitor` for tracking multiple laws simultaneously
- Pre-built Newtonian and Hamiltonian monitor configurations
- `DriftDetector` with threshold-based drift detection
- `RenormalizationGroup` for scale analysis of symmetry breaking
- `Report` generation in text and JSON formats
- `ConservedQuantity` time-series tracker
- Comprehensive test suite including double-pendulum integration test
