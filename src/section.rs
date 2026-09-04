//! Plane ∩ double cone → Conway–Hatcher color class.
//!
//! **Theorem (cited):** a plane cutting a right circular double cone traces
//! ellipse, parabola, hyperbola, or a degenerate pair of lines.
//! **Model:** those four types are the inner_cone flux kinds / palette.
//! No fifth color.

use glam::Vec3;
use qga_math::IslandType;

/// Semi-vertical angle of \(x^2+y^2=z^2\). Parabola when the plane–axis
/// angle equals this.
const GENERATOR: f32 = std::f32::consts::FRAC_1_SQRT_2; // sin(π/4) = √2/2
const APEX_EPS: f32 = 1e-4;
const PARA_EPS: f32 = 0.02;

/// Four persistence regimes. Same bins as `inner_cone::FluxKind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SectionKind {
    Elliptic = 0,
    Parabolic = 1,
    Hyperbolic = 2,
    /// Degenerate (plane through apex) / Magic-Island hold.
    FlatPockets = 3,
}

impl SectionKind {
    pub fn from_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0 => Self::Elliptic,
            1 => Self::Parabolic,
            2 => Self::Hyperbolic,
            _ => Self::FlatPockets,
        }
    }

    /// `GpuParticle.pad` hue. Software fact of the particle shader's wheel,
    /// copied from inner_cone `FluxKind::hue`. Not a theorem.
    pub fn hue(self) -> f32 {
        match self {
            Self::Elliptic => 0.55,
            Self::Hyperbolic => 0.30,
            Self::Parabolic => 0.10,
            Self::FlatPockets => 0.80,
        }
    }

    pub fn island(self) -> IslandType {
        match self {
            Self::Elliptic => IslandType::Elliptic,
            Self::Parabolic => IslandType::Parabolic,
            Self::Hyperbolic => IslandType::Hyperbolic,
            Self::FlatPockets => IslandType::ZeroHyperbolic,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Elliptic => "elliptic",
            Self::Parabolic => "parabolic",
            Self::Hyperbolic => "hyperbolic",
            Self::FlatPockets => "flat-pockets",
        }
    }
}

/// Classify plane `n · x = offset` against a right double cone along `axis`.
///
/// `axis` is the local cone: visual \(+Z\) (odd field) or feeling \(+Y\)
/// (even field). `n` need not be unit.
///
/// Let α be the angle between the plane and the axis. For a 45° cone:
/// α > 45° ellipse, α = 45° parabola, α < 45° hyperbola, through apex
/// degenerate. α = arcsin(|n̂ · axis|).
pub fn classify_section(n: Vec3, offset: f32, axis: Vec3) -> SectionKind {
    if offset.abs() < APEX_EPS {
        return SectionKind::FlatPockets;
    }
    let n = n.normalize_or_zero();
    let axis = axis.normalize_or_zero();
    if n.length_squared() < 1e-12 || axis.length_squared() < 1e-12 {
        return SectionKind::FlatPockets;
    }
    let s = n.dot(axis).abs();
    if (s - GENERATOR).abs() < PARA_EPS {
        SectionKind::Parabolic
    } else if s > GENERATOR {
        SectionKind::Elliptic
    } else {
        SectionKind::Hyperbolic
    }
}
