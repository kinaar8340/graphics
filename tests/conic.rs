//! Table-driven plane ∩ double cone. Theorem (cited) + Model (four bins).

use glam::Vec3;
use shellscan::{classify_section, Field, SectionKind};

const Z: Vec3 = Vec3::Z;
const Y: Vec3 = Vec3::Y;

#[test]
fn four_classical_cuts_on_visual_axis() {
    let ellipse = classify_section(Vec3::Z, 0.5, Z);
    let hyperbola = classify_section(Vec3::X, 0.5, Z);
    let parabola = classify_section(Vec3::new(0.0, 1.0, 1.0), 0.5, Z);
    let degenerate = classify_section(Vec3::Z, 0.0, Z);

    assert_eq!(ellipse, SectionKind::Elliptic);
    assert_eq!(hyperbola, SectionKind::Hyperbolic);
    assert_eq!(parabola, SectionKind::Parabolic);
    assert_eq!(degenerate, SectionKind::FlatPockets);
}

#[test]
fn feeling_axis_is_orthogonal() {
    // Plane ⊥ +Y is elliptic in the feeling cone and hyperbolic in the visual cone.
    let n = Vec3::Y;
    assert_eq!(classify_section(n, 0.5, Y), SectionKind::Elliptic);
    assert_eq!(classify_section(n, 0.5, Z), SectionKind::Hyperbolic);
}

#[test]
fn field_axes_match_inner_cone_sketch() {
    assert_eq!(Field::Odd.cone_axis(), Vec3::Z);
    assert_eq!(Field::Even.cone_axis(), Vec3::Y);
    assert!(Field::Odd.is_visual());
    assert!(!Field::Even.is_visual());
}

#[test]
fn no_fifth_palette() {
    let hues: Vec<f32> = [
        SectionKind::Elliptic,
        SectionKind::Parabolic,
        SectionKind::Hyperbolic,
        SectionKind::FlatPockets,
    ]
    .iter()
    .map(|k| k.hue())
    .collect();
    assert_eq!(hues, vec![0.55, 0.10, 0.30, 0.80]);
}

#[test]
fn island_type_map() {
    use qga_math::IslandType;
    assert_eq!(SectionKind::Elliptic.island(), IslandType::Elliptic);
    assert_eq!(SectionKind::Parabolic.island(), IslandType::Parabolic);
    assert_eq!(SectionKind::Hyperbolic.island(), IslandType::Hyperbolic);
    assert_eq!(
        SectionKind::FlatPockets.island(),
        IslandType::ZeroHyperbolic
    );
}
