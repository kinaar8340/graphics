//! Two-clock interlacing. **Model**, not a CRT emulator.
//!
//! Odd field writes the visual cone. Even field writes the feeling cone.
//! The separator is blanking (not represented here). Persistence is mote
//! age on the silent field — the analog of `FluxMote.far_age`.

use crate::pixel::QgaPixel;
use crate::section::SectionKind;
use glam::Vec3;

/// Field bit. Clock 0 = even / feeling / static-side. Clock 1 = odd / visual / live.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Field {
    Even = 0,
    Odd = 1,
}

impl Field {
    pub fn from_frame(frame: u32) -> Self {
        if frame % 2 == 0 {
            Self::Even
        } else {
            Self::Odd
        }
    }

    pub fn from_bit(bit: u32) -> Self {
        if bit & 1 == 0 {
            Self::Even
        } else {
            Self::Odd
        }
    }

    pub fn bit(self) -> u32 {
        self as u32
    }

    /// Sketch axes, same as inner_cone: visual +Z, feeling +Y.
    pub fn cone_axis(self) -> Vec3 {
        match self {
            Self::Even => Vec3::Y,
            Self::Odd => Vec3::Z,
        }
    }

    pub fn is_visual(self) -> bool {
        matches!(self, Self::Odd)
    }
}

/// Cadence. `live_every` is the hold-scene N (30 in the 4090 clip). Phase 2
/// tests use 2 so even/odd are consecutive frames.
#[derive(Clone, Copy, Debug)]
pub struct TwoClock {
    pub live_every: u32,
}

impl TwoClock {
    /// CPU tests: field pair finishes in milliseconds.
    pub fn interlaced() -> Self {
        Self { live_every: 2 }
    }

    /// First window: upload clock matches the 4090 hold clip. Gun stays off;
    /// this only names the cadence. Interlace still writes every frame.
    pub fn windowed() -> Self {
        Self { live_every: 30 }
    }

    pub fn field(self, frame: u32) -> Field {
        Field::from_frame(frame)
    }

    /// Gun-fiber rewrite cadence (hold-scene N). Interlace still writes a
    /// field every frame; this flag is the live *fiber* clock.
    pub fn live_fiber(self, frame: u32) -> bool {
        let n = self.live_every.max(1);
        frame % n == 0
    }
}

/// Phosphor grain: a `qga_pixel` field. Not a framebuffer rectangle.
#[derive(Clone, Debug)]
pub struct Phosphor {
    pub pixels: Vec<QgaPixel>,
    /// Silent-field decay per tick. Model of phosphor, not NTSC.
    pub decay: f32,
    pub write_amp: f32,
    pub static_uploads: u32,
    pub live_writes: u32,
    pub live_fiber_writes: u32,
}

impl Phosphor {
    pub fn new(pixels: Vec<QgaPixel>) -> Self {
        Self {
            pixels,
            decay: 0.72,
            write_amp: 0.35,
            static_uploads: 0,
            live_writes: 0,
            live_fiber_writes: 0,
        }
    }

    /// Split a chart into even (feeling) and odd (visual) pixels.
    pub fn interlaced_chart(n_even: usize, n_odd: usize) -> Self {
        let mut pixels = Vec::with_capacity(n_even + n_odd);
        // Even / feeling (+Y): |n.y| > √2/2 → elliptic vs that axis.
        for i in 0..n_even {
            pixels.push(QgaPixel::new(
                std::f32::consts::FRAC_PI_2,
                std::f32::consts::FRAC_PI_2,
                i as f32 * 0.15,
                0.4,
                1.0,
                Field::Even,
                i as f32,
            ));
        }
        // Odd / visual (+Z): |n.z| > √2/2 → elliptic vs that axis.
        for i in 0..n_odd {
            pixels.push(QgaPixel::new(
                0.3,
                i as f32 * 0.2,
                0.1,
                0.4,
                1.0,
                Field::Odd,
                i as f32,
            ));
        }
        let mut p = Self::new(pixels);
        p.static_uploads = 1;
        p
    }

    /// Occupancy interlace on γ. **Not** “every mote, both fields.”
    ///
    /// `n` sites on `[0,1)`, `n` even. Field 0 owns even sites
    /// `s = 0, 2, 4, … / n`. Field 1 owns odd sites `s = 1, 3, 5, … / n`.
    /// `03_both` is the two dashed sets on the same curve.
    /// Hopf tilt stays elliptic vs the field's cone axis. Layout untouched.
    pub fn on_trench(n: usize) -> Self {
        let n = n.max(2) & !1;
        let mut pixels = Vec::with_capacity(n);
        for i in 0..n {
            let s = i as f32 / n as f32;
            let field = if i % 2 == 0 { Field::Even } else { Field::Odd };
            let (theta, phi) = match field {
                Field::Even => (std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2),
                Field::Odd => (0.3, 0.0),
            };
            pixels.push(QgaPixel::new(
                theta,
                phi,
                s * std::f32::consts::TAU,
                0.4,
                1.0,
                field,
                s,
            ));
        }
        let mut p = Self::new(pixels);
        p.static_uploads = 1;
        p
    }

    /// One confocal layer. `n` even sites, field 0, elliptic. `shell_s` unchanged.
    pub fn even_layer(n: usize, layer: u8) -> Self {
        let n = n.max(2);
        let mut pixels = Vec::with_capacity(n);
        for i in 0..n {
            let s = i as f32 / n as f32;
            let mut p = QgaPixel::new(
                std::f32::consts::FRAC_PI_2,
                std::f32::consts::FRAC_PI_2,
                s * std::f32::consts::TAU,
                0.4,
                1.0,
                Field::Even,
                s,
            );
            p.set_layer(layer);
            pixels.push(p);
        }
        let mut p = Self::new(pixels);
        p.static_uploads = 1;
        p
    }

    /// `L` confocal even-site layers. Same trench table, packed layer bits.
    pub fn even_layers(n: usize, n_layers: u8) -> Self {
        let n_layers = n_layers.max(1);
        let mut pixels = Vec::new();
        let mut su = 0;
        for ell in 0..n_layers {
            let mut layer = Self::even_layer(n, ell);
            su = layer.static_uploads;
            pixels.append(&mut layer.pixels);
        }
        let mut p = Self::new(pixels);
        p.static_uploads = su;
        p
    }

    pub fn light_persist(&mut self, persist: f32) {
        for p in &mut self.pixels {
            p.persist = persist;
        }
    }

    pub fn layer_energy(&self, layer: u8) -> f32 {
        self.pixels
            .iter()
            .filter(|p| p.layer() == layer)
            .map(|p| p.amplitude * p.persist)
            .sum()
    }

    pub fn tick(&mut self, frame: u32, clock: TwoClock) {
        let field = clock.field(frame);
        self.live_writes += 1;
        if clock.live_fiber(frame) {
            self.live_fiber_writes += 1;
        }
        for p in &mut self.pixels {
            if p.field() == field {
                p.persist = (p.persist + self.write_amp).min(1.0);
            } else {
                p.persist *= self.decay;
            }
        }
    }

    pub fn field_energy(&self, field: Field) -> f32 {
        self.pixels
            .iter()
            .filter(|p| p.field() == field)
            .map(|p| p.amplitude * p.persist)
            .sum()
    }

    pub fn composed_energy(&self) -> f32 {
        self.pixels.iter().map(|p| p.amplitude * p.persist).sum()
    }

    pub fn section_counts(&self) -> [u32; 4] {
        let mut c = [0u32; 4];
        for p in &self.pixels {
            c[p.section() as usize] += 1;
        }
        c
    }

    pub fn elliptic_only(&self) -> bool {
        self.pixels
            .iter()
            .all(|p| p.section() == SectionKind::Elliptic)
    }
}
