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
}
