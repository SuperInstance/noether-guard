use std::fmt;

/// Represents a gauge symmetry group (user-defined).
#[derive(Debug, Clone, PartialEq)]
pub struct SymmetryGroup {
    pub name: String,
    pub generators: usize,
}

/// Physical symmetry that implies a conservation law (Noether's theorem).
#[derive(Debug, Clone, PartialEq)]
pub enum Symmetry {
    /// Time translation → energy conservation.
    TimeTranslation,
    /// Spatial translation → momentum conservation (x, y, z).
    SpatialTranslation,
    /// Rotation → angular momentum conservation (x, y, z).
    Rotation,
    /// Gauge symmetry → custom conservation law.
    Gauge(SymmetryGroup),
}

impl Symmetry {
    /// Return the human-readable name of the conserved quantity.
    pub fn conserved_quantity(&self) -> &str {
        match self {
            Self::TimeTranslation => "Energy",
            Self::SpatialTranslation => "Momentum",
            Self::Rotation => "Angular Momentum",
            Self::Gauge(g) => &g.name,
        }
    }

    /// Number of scalar components tracked for this symmetry.
    pub fn component_count(&self) -> usize {
        match self {
            Self::TimeTranslation => 1,
            Self::SpatialTranslation => 3,
            Self::Rotation => 3,
            Self::Gauge(g) => g.generators,
        }
    }
}

impl fmt::Display for Symmetry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeTranslation => write!(f, "Time Translation → Energy"),
            Self::SpatialTranslation => write!(f, "Spatial Translation → Momentum"),
            Self::Rotation => write!(f, "Rotation → Angular Momentum"),
            Self::Gauge(g) => write!(f, "Gauge({}) → {}", g.name, g.name),
        }
    }
}

impl fmt::Display for SymmetryGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} generators)", self.name, self.generators)
    }
}
