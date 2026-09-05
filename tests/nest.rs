//! N1 confocal stack. Same γ, three radii. Ledger before eye.

use shellscan::{
    Field, Phosphor, QgaPixel, SectionKind, Trench, NEST_DELTA_R, NEST_LAYERS, RAIL_EPS, SPLAT_LOCK,
};
use shellscan::nest::{layers_on_radius, measure, shell_s_not_folded, NEST_N};

#[test]
fn still_32_bytes() {
    assert_eq!(std::mem::size_of::<QgaPixel>(), 32);
}

#[test]
fn packed_layer_roundtrip() {
    let mut p = QgaPixel::new(1.57, 1.57, 0.0, 0.4, 1.0, Field::Even, 0.18);
    assert_eq!(p.layer(), 0);
    p.set_layer(2);
    assert_eq!(p.layer(), 2);
    assert_eq!(p.field(), Field::Even);
    assert_eq!(p.section(), SectionKind::Elliptic);
    p.set_field(Field::Even);
    assert_eq!(p.layer(), 2);
    let q = QgaPixel::from_bytes(p.to_bytes());
    assert_eq!(q.layer(), 2);
    assert_eq!(q.to_bytes().len(), 32);
}

#[test]
fn layer_zero_is_old_bind() {
    let trench = Trench::circle(64);
    let p = QgaPixel::new(1.57, 1.57, 0.0, 0.4, 1.0, Field::Even, 0.25);
    assert_eq!(p.layer(), 0);
    let g = trench.gamma(0.25) + Field::Even.cone_axis() * RAIL_EPS;
    assert!(p.bind_shell(&trench).distance(g) < 1e-5);
}

#[test]
fn delta_r_beats_splat_and_not_sqrt3() {
    assert!((NEST_DELTA_R - 0.08).abs() < 1e-8);
    assert_eq!(NEST_LAYERS, 3);
    assert!(NEST_DELTA_R > SPLAT_LOCK);
    assert!(NEST_DELTA_R > RAIL_EPS);
    assert!(NEST_DELTA_R < 0.5);
}

#[test]
fn sep_is_delta_r() {
    let trench = Trench::circle(64);
    let mut ph = Phosphor::even_layers(NEST_N, NEST_LAYERS);
    ph.light_persist(1.0);
    let r = measure(&ph, &trench);
    assert!((r.sep_01 - NEST_DELTA_R).abs() < 1e-4, "sep_01={}", r.sep_01);
    assert!((r.sep_12 - NEST_DELTA_R).abs() < 1e-4, "sep_12={}", r.sep_12);
    assert!(r.sep_01 > SPLAT_LOCK);
    assert!(r.sep_12 > SPLAT_LOCK);
}

#[test]
fn no_bleed_into_shell_s() {
    let trench = Trench::load(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/shell_trench.bin"
    ))
    .expect("export-shell");
    let mut ph = Phosphor::even_layers(NEST_N, NEST_LAYERS);
    ph.light_persist(1.0);
    assert!(shell_s_not_folded(&ph));
    assert!(layers_on_radius(&ph, &trench));
    assert!(ph.elliptic_only());
    for p in &ph.pixels {
        assert_eq!(p.section(), SectionKind::Elliptic);
        assert_eq!(p.field(), Field::Even);
        assert!(p.layer() < NEST_LAYERS);
    }
}

#[test]
fn energy_is_linear() {
    let mut l0 = Phosphor::even_layer(NEST_N, 0);
    let mut l1 = Phosphor::even_layer(NEST_N, 1);
    let mut l2 = Phosphor::even_layer(NEST_N, 2);
    l0.light_persist(1.0);
    l1.light_persist(1.0);
    l2.light_persist(1.0);
    let mut all = Phosphor::even_layers(NEST_N, NEST_LAYERS);
    all.light_persist(1.0);
    let sum = l0.composed_energy() + l1.composed_energy() + l2.composed_energy();
    assert!((all.composed_energy() - sum).abs() < 1e-3);
    assert!((all.layer_energy(0) - l0.composed_energy()).abs() < 1e-3);
}

#[test]
fn json_names_layer() {
    let mut p = QgaPixel::new(0.3, 0.0, 0.0, 0.4, 1.0, Field::Odd, 0.5);
    p.set_layer(0);
    let j = p.to_json();
    assert!(j.contains("\"layer\":0"));
    assert!(j.contains("\"persist\":0.000000"));
}
