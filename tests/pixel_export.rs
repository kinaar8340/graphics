//! 32-byte dump is the picker deliverable. Software fact of layout.

use shellscan::{Field, QgaPixel};

#[test]
fn roundtrip_bytes() {
    let p = QgaPixel::new(0.3, 0.2, 1.1, 0.4, 1.0, Field::Odd, 0.18);
    let b = p.to_bytes();
    assert_eq!(b.len(), 32);
    let q = QgaPixel::from_bytes(b);
    assert!((q.theta - p.theta).abs() < 1e-6);
    assert!((q.phi - p.phi).abs() < 1e-6);
    assert_eq!(q.field(), p.field());
    assert_eq!(q.section(), p.section());
    assert_eq!(q.layer(), 0);
    assert!((q.shell_s - 0.18).abs() < 1e-6);
}

#[test]
fn json_names_section_and_field_bit() {
    let p = QgaPixel::new(0.3, 0.0, 0.0, 0.4, 1.0, Field::Odd, 0.5);
    let j = p.to_json();
    assert!(j.contains("\"section\":\"elliptic\""));
    assert!(j.contains("\"field\":1"));
    assert!(j.contains("\"layer\":0"));
    assert!(j.contains("\"shell_s\":0.500000"));
}
