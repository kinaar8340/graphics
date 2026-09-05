//! 32-byte slot contract. Software fact.

use shellscan::QgaPixel;
use std::mem::{align_of, size_of};

#[test]
fn qga_pixel_is_gpu_slot() {
    assert_eq!(size_of::<QgaPixel>(), 32);
    assert_eq!(size_of::<QgaPixel>() % 8, 0);
    assert_eq!(size_of::<QgaPixel>() % 4, 0);
    assert!(align_of::<QgaPixel>() >= 4);
}

#[test]
fn packed_field_and_section_roundtrip() {
    use shellscan::{Field, SectionKind};
    let mut p = QgaPixel::new(0.3, 0.0, 0.2, 0.5, 1.0, Field::Odd, 1.5);
    assert_eq!(p.field(), Field::Odd);
    assert_eq!(p.section(), SectionKind::Elliptic);
    p.set_field(Field::Even);
    assert_eq!(p.field(), Field::Even);
    // Even classifies against +Y; n ≈ (0.3 sin, 0, 0.3 cos) is hyperbolic vs Y.
    assert_eq!(p.section(), SectionKind::Hyperbolic);
    p.set_layer(2);
    assert_eq!(p.layer(), 2);
    assert_eq!(p.field(), Field::Even);
    assert_eq!(p.section(), SectionKind::Hyperbolic);
}

#[test]
fn antipode_psi_is_invisible_to_rgb() {
    use shellscan::{Field, SectionKind};
    let mut p = QgaPixel::new(0.3, 0.0, 0.2, 0.4, 1.0, Field::Odd, 0.5);
    p.persist = 1.0;
    let rgb0 = p.rgb_preview();
    let hopf = (p.theta, p.phi);
    p.psi = (p.psi + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU);
    p.reclassify();
    assert_eq!((p.theta, p.phi), hopf);
    assert_eq!(p.section(), SectionKind::Elliptic);
    assert_eq!(p.field(), Field::Odd);
    assert_eq!(p.layer(), 0);
    assert_eq!(p.rgb_preview(), rgb0);
}
