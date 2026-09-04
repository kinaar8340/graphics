//! Bound pos = γ(shell_s). Hopf fields stay put. Software fact.

use glam::Vec3;
use shellscan::{
    to_gpu_particle, to_gpu_particle_on, Field, Phosphor, QgaPixel, SectionKind, Trench, TwoClock,
    N_OCCUPANCY, RAIL_EPS,
};

#[test]
fn bind_leaves_hopf_alone() {
    let trench = Trench::circle(64);
    let mut p = QgaPixel::new(0.3, 0.1, 1.2, 0.4, 1.0, Field::Odd, 0.25);
    let hopf = (p.theta, p.phi, p.psi);
    p.persist = 1.0;
    let unbound = p.pos_unbound();
    let bound = p.bind_shell(&trench);
    assert_eq!((p.theta, p.phi, p.psi), hopf);
    assert!(unbound.distance(bound) > 0.05);
    let g = trench.gamma(0.25) + Field::Odd.cone_axis() * RAIL_EPS;
    assert!(bound.distance(g) < 1e-5);
}

#[test]
fn motes_lie_on_the_trench() {
    let trench = Trench::circle(128);
    let mut ph = Phosphor::on_trench(64);
    assert!(ph.elliptic_only());
    ph.tick(0, TwoClock::interlaced());
    for p in &ph.pixels {
        let pos = to_gpu_particle_on(*p, Some(&trench)).pos;
        let g = p.bind_shell(&trench);
        let d = Vec3::from_array(pos).distance(g);
        assert!(d < 0.08, "mote left the trench d={d}");
    }
}

#[test]
fn both_fields_brighter_after_gpu_pack() {
    let trench = Trench::circle(256);
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    let clock = TwoClock::interlaced();
    ph.tick(0, clock);
    ph.tick(1, clock);
    let gpu: Vec<_> = ph
        .pixels
        .iter()
        .map(|p| to_gpu_particle_on(*p, Some(&trench)))
        .collect();
    let even: f32 = ph
        .pixels
        .iter()
        .zip(&gpu)
        .filter(|(p, _)| p.field() == Field::Even)
        .map(|(_, g)| g.mass)
        .sum();
    let odd: f32 = ph
        .pixels
        .iter()
        .zip(&gpu)
        .filter(|(p, _)| p.field() == Field::Odd)
        .map(|(_, g)| g.mass)
        .sum();
    let both: f32 = gpu.iter().map(|g| g.mass).sum();
    assert!(even > 0.0 && odd > 0.0);
    assert!(both > even && both > odd);
    assert!(gpu
        .iter()
        .all(|g| (g.pad - SectionKind::Elliptic.hue()).abs() < 1e-4));
}

#[test]
fn unbound_path_still_exists() {
    let p = QgaPixel::new(0.4, 0.1, 0.0, 0.5, 1.0, Field::Odd, 0.0);
    let g = to_gpu_particle(p);
    assert_eq!(g.pos, p.pos_unbound().to_array());
}

#[test]
fn load_offline_artifact() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shell_trench.bin");
    let trench = Trench::load(path).expect("run make export-shell");
    assert!(trench.samples.len() >= 64);
    assert!(!trench.verts.is_empty());
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    ph.tick(0, TwoClock::windowed());
    ph.tick(1, TwoClock::windowed());
    let gpu: Vec<_> = ph
        .pixels
        .iter()
        .map(|p| to_gpu_particle_on(*p, Some(&trench)))
        .collect();
    for (p, g) in ph.pixels.iter().zip(&gpu) {
        let d = Vec3::from_array(g.pos).distance(p.bind_shell(&trench));
        assert!(d < 0.12, "offline mote left trench d={d}");
    }
    assert!(ph.elliptic_only());
}

#[test]
fn occupancy_even_odd_sites_are_complementary() {
    let n = N_OCCUPANCY;
    let ph = Phosphor::on_trench(n);
    assert_eq!(ph.pixels.len(), n);
    let mut even_idx = Vec::new();
    let mut odd_idx = Vec::new();
    for p in &ph.pixels {
        let i = (p.shell_s * n as f32).round() as i32;
        match p.field() {
            Field::Even => {
                assert_eq!(i % 2, 0, "field 0 must own even sites, got {i}");
                even_idx.push(i);
            }
            Field::Odd => {
                assert_eq!(i % 2, 1, "field 1 must own odd sites, got {i}");
                odd_idx.push(i);
            }
        }
    }
    assert_eq!(even_idx.len(), n / 2);
    assert_eq!(odd_idx.len(), n / 2);
    for i in even_idx {
        assert!(!odd_idx.contains(&i), "site {i} owned by both fields");
    }
}

#[test]
fn rails_are_world_cone_axes() {
    let trench = Trench::circle(64);
    let even = QgaPixel::new(1.57, 1.57, 0.0, 0.4, 1.0, Field::Even, 0.25);
    let odd = QgaPixel::new(0.3, 0.0, 0.0, 0.4, 1.0, Field::Odd, 0.25);
    let g = trench.gamma(0.25);
    let e = even.bind_shell(&trench);
    let o = odd.bind_shell(&trench);
    assert!((e - (g + Vec3::Y * RAIL_EPS)).length() < 1e-5);
    assert!((o - (g + Vec3::Z * RAIL_EPS)).length() < 1e-5);
    let split = (e - o).length();
    assert!((split - RAIL_EPS * 2f32.sqrt()).abs() < 1e-4);
}
