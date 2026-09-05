//! Animation A: persist peak on even occupancy sites. CPU software fact.

use shellscan::scan::{
    apply_scan, expected_tail_energy, head_index, measure, odd_sites_dark, SCAN_DELTA,
    SCAN_EVEN_LAP, SCAN_I0, SCAN_K_TAIL, SCAN_STRIDE,
};
use shellscan::{Field, Phosphor, SectionKind, Trench, N_OCCUPANCY, RAIL_EPS};

fn sheet(n_frames: u32) -> Phosphor {
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    for t in 0..n_frames {
        apply_scan(&mut ph, head_index(t, SCAN_I0));
    }
    ph
}

#[test]
fn occupancy_and_rails_untouched() {
    assert_eq!(N_OCCUPANCY, 256);
    assert!((RAIL_EPS - 0.02).abs() < 1e-8);
    assert_eq!(SCAN_STRIDE, 2);
    assert_eq!(SCAN_EVEN_LAP, 128);
}

#[test]
fn head_walks_even_sites_only() {
    for t in 0..320 {
        let i = head_index(t, SCAN_I0);
        assert_eq!(i % 2, 0);
        assert!(i < N_OCCUPANCY);
    }
    assert_eq!(head_index(0, SCAN_I0), 0);
    assert_eq!(head_index(1, SCAN_I0), 2);
    assert_eq!(head_index(SCAN_EVEN_LAP as u32, SCAN_I0), SCAN_I0);
}

#[test]
fn persist_peak_and_tail() {
    let ph = sheet(1);
    let head = head_index(0, SCAN_I0);
    assert!((ph.pixels[head].persist - 1.0).abs() < 1e-6);
    let mut acc = 1.0;
    for k in 1..=SCAN_K_TAIL {
        acc *= SCAN_DELTA;
        let i = (head + N_OCCUPANCY - SCAN_STRIDE * k) % N_OCCUPANCY;
        assert!(
            (ph.pixels[i].persist - acc).abs() < 1e-5,
            "tail k={k} persist={} want {acc}",
            ph.pixels[i].persist
        );
        assert_eq!(i % 2, 0);
    }
    assert!(odd_sites_dark(&ph));
}

#[test]
fn energy_peak_is_a_peak() {
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    apply_scan(&mut ph, head_index(0, SCAN_I0));
    let e = measure(&ph, 0, head_index(0, SCAN_I0), SCAN_K_TAIL);
    assert!((e.energy_head - 1.0).abs() < 1e-5);
    let want_tail = expected_tail_energy(SCAN_K_TAIL, SCAN_DELTA);
    assert!((e.energy_tail - want_tail).abs() < 1e-4);
    assert!(e.energy_other.abs() < 1e-6);
    assert!(e.other_frac() < 1e-5);
    assert!(e.peak_is_peak(SCAN_K_TAIL));
}

#[test]
fn elliptic_and_shell_s_frozen() {
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    let s0: Vec<f32> = ph.pixels.iter().map(|p| p.shell_s).collect();
    apply_scan(&mut ph, head_index(17, SCAN_I0));
    assert!(ph.elliptic_only());
    for (i, p) in ph.pixels.iter().enumerate() {
        assert_eq!(p.section(), SectionKind::Elliptic);
        assert!((p.shell_s - s0[i]).abs() < 1e-8);
        assert!((p.shell_s - i as f32 / N_OCCUPANCY as f32).abs() < 1e-5);
    }
}

#[test]
fn head_stays_on_the_trench() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shell_trench.bin");
    let trench = Trench::load(path).expect("run make export-shell");
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    for t in [0u32, 3, 40, 127] {
        let hi = head_index(t, SCAN_I0);
        apply_scan(&mut ph, hi);
        let p = ph.pixels[hi];
        assert!((p.shell_s - hi as f32 / N_OCCUPANCY as f32).abs() < 1e-5);
        let g = trench.gamma(p.shell_s) + Field::Even.cone_axis() * RAIL_EPS;
        assert!(p.bind_shell(&trench).distance(g) < 1e-5);
        for k in 0..=SCAN_K_TAIL {
            let i = if k == 0 {
                hi
            } else {
                (hi + N_OCCUPANCY - SCAN_STRIDE * k) % N_OCCUPANCY
            };
            let q = ph.pixels[i];
            let pos = q.bind_shell(&trench);
            let expect = trench.gamma(q.shell_s) + q.field().cone_axis() * RAIL_EPS;
            assert!(
                pos.distance(expect) < 1e-5,
                "tail left rail t={t} k={k}"
            );
            let radial = (pos - trench.gamma(q.shell_s)).length();
            assert!(radial < RAIL_EPS + 1e-4);
        }
    }
}

#[test]
fn even_lap_repeats_energy() {
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    apply_scan(&mut ph, head_index(0, SCAN_I0));
    let a: Vec<f32> = ph.pixels.iter().map(|p| p.persist).collect();
    let e0 = measure(&ph, 0, head_index(0, SCAN_I0), SCAN_K_TAIL);
    apply_scan(&mut ph, head_index(SCAN_EVEN_LAP as u32, SCAN_I0));
    let b: Vec<f32> = ph.pixels.iter().map(|p| p.persist).collect();
    let e1 = measure(
        &ph,
        SCAN_EVEN_LAP as u32,
        head_index(SCAN_EVEN_LAP as u32, SCAN_I0),
        SCAN_K_TAIL,
    );
    assert_eq!(head_index(SCAN_EVEN_LAP as u32, SCAN_I0), SCAN_I0);
    for (x, y) in a.iter().zip(&b) {
        assert!((x - y).abs() < 1e-6);
    }
    assert!((e0.energy_head - e1.energy_head).abs() < 1e-6);
    assert!((e0.energy_tail - e1.energy_tail).abs() < 1e-6);
    assert!(e1.energy_other.abs() < 1e-6);
}

#[test]
fn nest_layers_stay_dark() {
    let mut ph = Phosphor::even_layers(N_OCCUPANCY, 3);
    apply_scan(&mut ph, 0);
    assert!(ph
        .pixels
        .iter()
        .filter(|p| p.layer() != 0)
        .all(|p| p.persist <= 1e-8));
    assert!(ph.layer_energy(1).abs() < 1e-8);
    assert!(ph.layer_energy(2).abs() < 1e-8);
    assert!(ph.layer_energy(0) > 1.0);
}

#[test]
fn does_not_call_field_write() {
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    let live0 = ph.live_writes;
    apply_scan(&mut ph, 0);
    apply_scan(&mut ph, 2);
    assert_eq!(ph.live_writes, live0);
    assert_eq!(ph.live_fiber_writes, 0);
    assert_eq!(ph.static_uploads, 1);
}
