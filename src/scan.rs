//! Animation A: one persist peak walking even occupancy sites on γ.
//! Not a gun. Not configuration C (two heads).

use crate::clock::{Field, Phosphor};
use crate::pixel::QgaPixel;
use crate::trench::N_OCCUPANCY;

pub const SCAN_K_TAIL: usize = 8;
pub const SCAN_DELTA: f32 = 0.7;
pub const SCAN_I0: usize = 0;
/// Occupancy stride: field-0 even sites only. Odd sites stay dark.
pub const SCAN_STRIDE: usize = 2;
pub const SCAN_EVEN_LAP: usize = N_OCCUPANCY / SCAN_STRIDE;
/// Two even laps. 256 frames at 30 fps ≈ 8.5s.
pub const SCAN_FRAMES: u32 = (SCAN_EVEN_LAP * 2) as u32;

#[derive(Clone, Copy, Debug)]
pub struct ScanEnergy {
    pub t: u32,
    pub head_i: usize,
    pub head_s: f32,
    pub energy_head: f32,
    pub energy_tail: f32,
    pub energy_other: f32,
}

impl ScanEnergy {
    pub fn total(self) -> f32 {
        self.energy_head + self.energy_tail + self.energy_other
    }

    pub fn other_frac(self) -> f32 {
        let t = self.total();
        if t <= 1e-12 {
            0.0
        } else {
            self.energy_other / t
        }
    }

    pub fn peak_is_peak(self, k_tail: usize) -> bool {
        let k = k_tail.max(1) as f32;
        self.energy_head > self.energy_tail / k
    }
}

/// Even occupancy index. `i0` forced even. One even site per frame.
pub fn head_index(t: u32, i0: usize) -> usize {
    let i0 = i0 & !1;
    (i0 + SCAN_STRIDE * t as usize) % N_OCCUPANCY
}

pub fn tail_index(head_i: usize, k: usize) -> usize {
    let n = N_OCCUPANCY;
    (head_i + n - SCAN_STRIDE * k) % n
}

/// Direct persist assignment on layer 0 only. Does not call `Phosphor::tick`.
/// Other layers stay dark. Do not chase three heads.
pub fn apply_scan(ph: &mut Phosphor, head_i: usize) {
    apply_scan_params(ph, head_i, SCAN_K_TAIL, SCAN_DELTA);
}

fn layer0_indices(pixels: &[QgaPixel]) -> Vec<usize> {
    pixels
        .iter()
        .enumerate()
        .filter(|(_, p)| p.layer() == 0)
        .map(|(i, _)| i)
        .collect()
}

pub fn apply_scan_params(ph: &mut Phosphor, head_i: usize, k_tail: usize, delta: f32) {
    for p in &mut ph.pixels {
        p.persist = 0.0;
    }
    let idx = layer0_indices(&ph.pixels);
    let n = idx.len();
    if n == 0 {
        return;
    }
    let head = (head_i % n) & !1;
    if let Some(&i) = idx.get(head) {
        ph.pixels[i].persist = 1.0;
    }
    let mut acc = 1.0;
    for k in 1..=k_tail {
        acc *= delta;
        let t = (head + n - SCAN_STRIDE * k) % n;
        if let Some(&i) = idx.get(t) {
            ph.pixels[i].persist = acc;
        }
    }
}

pub fn measure(ph: &Phosphor, t: u32, head_i: usize, k_tail: usize) -> ScanEnergy {
    let idx = layer0_indices(&ph.pixels);
    let n = idx.len();
    let head_slot = if n == 0 {
        0
    } else {
        (head_i as usize % n) & !1
    };
    let head = idx.get(head_slot).copied().unwrap_or(0);
    let mut energy_head = 0.0;
    let mut energy_tail = 0.0;
    let mut energy_other = 0.0;
    for (i, p) in ph.pixels.iter().enumerate() {
        let e = p.amplitude * p.persist;
        if i == head {
            energy_head += e;
        } else if n > 0
            && (1..=k_tail).any(|k| idx.get((head_slot + n - SCAN_STRIDE * k) % n) == Some(&i))
        {
            energy_tail += e;
        } else {
            energy_other += e;
        }
    }
    let head_s = ph.pixels.get(head).map(|p| p.shell_s).unwrap_or(0.0);
    ScanEnergy {
        t,
        head_i: head,
        head_s,
        energy_head,
        energy_tail,
        energy_other,
    }
}

pub fn energy_json(rows: &[ScanEnergy]) -> String {
    let mut s = String::from("{\n");
    s.push_str(&format!("  \"n\": {N_OCCUPANCY},\n"));
    s.push_str(&format!("  \"k_tail\": {SCAN_K_TAIL},\n"));
    s.push_str(&format!("  \"delta\": {SCAN_DELTA},\n"));
    s.push_str(&format!("  \"i0\": {SCAN_I0},\n"));
    s.push_str(&format!("  \"stride\": {SCAN_STRIDE},\n"));
    s.push_str(&format!("  \"even_lap\": {SCAN_EVEN_LAP},\n"));
    s.push_str(&format!("  \"frames\": [\n"));
    for (i, r) in rows.iter().enumerate() {
        s.push_str(&format!(
            "    {{\"t\":{},\"head_i\":{},\"head_s\":{:.6},\"energy_head\":{:.6},\"energy_tail\":{:.6},\"energy_other\":{:.6}}}",
            r.t, r.head_i, r.head_s, r.energy_head, r.energy_tail, r.energy_other
        ));
        if i + 1 != rows.len() {
            s.push(',');
        }
        s.push('\n');
    }
    s.push_str("  ]\n}\n");
    s
}

pub fn odd_sites_dark(ph: &Phosphor) -> bool {
    ph.pixels
        .iter()
        .filter(|p| p.field() == Field::Odd)
        .all(|p| p.persist <= 1e-8)
}

pub fn expected_tail_energy(k_tail: usize, delta: f32) -> f32 {
    if (delta - 1.0).abs() < 1e-8 {
        return k_tail as f32;
    }
    delta * (1.0 - delta.powi(k_tail as i32)) / (1.0 - delta)
}
