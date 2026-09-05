//! N1 confocal stack. Same γ, three radii. Not a thicker necklace.

use crate::clock::Phosphor;
use crate::trench::{Trench, NEST_DELTA_R, NEST_LAYERS, N_OCCUPANCY, SPLAT_LOCK};

pub const NEST_N: usize = N_OCCUPANCY;

#[derive(Clone, Copy, Debug)]
pub struct NestReport {
    pub sep_01: f32,
    pub sep_12: f32,
    pub energy_l0: f32,
    pub energy_l1: f32,
    pub energy_l2: f32,
}

impl NestReport {
    pub fn energy_sum(self) -> f32 {
        self.energy_l0 + self.energy_l1 + self.energy_l2
    }
}

/// Min |p_ℓ − p_ℓ+1| / R over matching `shell_s`. R ≈ 1.
pub fn min_sep(ph: &Phosphor, trench: &Trench, a: u8, b: u8) -> f32 {
    let mut best = f32::MAX;
    for p in ph.pixels.iter().filter(|p| p.layer() == a) {
        let Some(q) = ph
            .pixels
            .iter()
            .find(|q| q.layer() == b && (q.shell_s - p.shell_s).abs() < 1e-6)
        else {
            continue;
        };
        let d = p.bind_shell(trench).distance(q.bind_shell(trench));
        if d < best {
            best = d;
        }
    }
    if best.is_finite() {
        best
    } else {
        0.0
    }
}

pub fn measure(ph: &Phosphor, trench: &Trench) -> NestReport {
    NestReport {
        sep_01: min_sep(ph, trench, 0, 1),
        sep_12: min_sep(ph, trench, 1, 2),
        energy_l0: ph.layer_energy(0),
        energy_l1: ph.layer_energy(1),
        energy_l2: ph.layer_energy(2),
    }
}

pub fn energy_json(r: NestReport) -> String {
    format!(
        "{{\n  \"L\": {NEST_LAYERS},\n  \"dR\": {NEST_DELTA_R},\n  \"splat\": {SPLAT_LOCK},\n  \"n\": {NEST_N},\n  \"sep_01\": {:.6},\n  \"sep_12\": {:.6},\n  \"energy_L0\": {:.6},\n  \"energy_L1\": {:.6},\n  \"energy_L2\": {:.6}\n}}\n",
        r.sep_01, r.sep_12, r.energy_l0, r.energy_l1, r.energy_l2
    )
}

pub fn layers_on_radius(ph: &Phosphor, trench: &Trench) -> bool {
    ph.pixels.iter().all(|p| {
        let g = trench.gamma(p.shell_s);
        let rhat = trench.normal(p.shell_s);
        let rail = p.field().cone_axis() * crate::trench::RAIL_EPS;
        let along = (p.bind_shell(trench) - g - rail).dot(rhat);
        (along - p.layer() as f32 * NEST_DELTA_R).abs() < 1e-4
    })
}

pub fn shell_s_not_folded(ph: &Phosphor) -> bool {
    let n = NEST_N as f32;
    ph.pixels.iter().all(|p| {
        let i = (p.shell_s * n).round();
        (p.shell_s - i / n).abs() < 1e-5 && p.shell_s < 1.0 + 1e-5
    })
}
