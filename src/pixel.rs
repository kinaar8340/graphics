//! 32-byte `qga_pixel`. **Model.** RGB is not the store.
//!
//! Layout (Software fact of this crate, matches `GpuParticle` size):
//! ```text
//! theta f32     Hopf / spherical polar of the cutting-plane normal
//! phi   f32     azimuth of that normal (S² cell)
//! psi   f32     fiber phase — subpixel scan
//! offset f32    plane offset n·x = p  (eccentricity / saturation)
//! amplitude f32 quaternion-shard length analog; not R,G, or B
//! shell_s f32   trench parameter (faceplate binding; unused until Phase 3)
//! persist f32   mote age (FluxMote.far_age analog)
//! packed u32    field:1 | section:2 | unused
//! ```
//!
//! FluxMote map (documentation, not a dep):
//! `pos` ← stereographic lift, `kind` ← section, `q` ← hopf_coordinates,
//! `far_age` ← persist, `vel` ← plane normal.

use crate::clock::Field;
use crate::section::{classify_section, SectionKind};
use crate::trench::Trench;
use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use qga_math::{hopf_coordinates, stereographic, Q};

const _: () = assert!(std::mem::size_of::<QgaPixel>() == 32);
const _: () = assert!(std::mem::align_of::<QgaPixel>() >= 4);

const FIELD_BIT: u32 = 0b1;
const SECTION_SHIFT: u32 = 1;
const SECTION_MASK: u32 = 0b11;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct QgaPixel {
    pub theta: f32,
    pub phi: f32,
    pub psi: f32,
    pub offset: f32,
    pub amplitude: f32,
    pub shell_s: f32,
    pub persist: f32,
    packed: u32,
}

impl QgaPixel {
    pub fn new(
        theta: f32,
        phi: f32,
        psi: f32,
        offset: f32,
        amplitude: f32,
        field: Field,
        shell_s: f32,
    ) -> Self {
        let mut p = Self {
            theta,
            phi,
            psi,
            offset,
            amplitude,
            shell_s,
            persist: 0.0,
            packed: 0,
        };
        p.set_field(field);
        p
    }

    pub fn field(self) -> Field {
        Field::from_bit(self.packed & FIELD_BIT)
    }

    pub fn set_field(&mut self, field: Field) {
        self.packed = (self.packed & !FIELD_BIT) | field.bit();
        self.reclassify();
    }

    pub fn section(self) -> SectionKind {
        SectionKind::from_bits(self.packed >> SECTION_SHIFT)
    }

    fn set_section(&mut self, kind: SectionKind) {
        let bits = (kind as u32) & SECTION_MASK;
        self.packed = (self.packed & !(SECTION_MASK << SECTION_SHIFT)) | (bits << SECTION_SHIFT);
    }

    /// Plane normal from (θ, φ). Tilt is hue class; offset is eccentricity.
    pub fn plane_normal(self) -> Vec3 {
        let (st, ct) = self.theta.sin_cos();
        let (sp, cp) = self.phi.sin_cos();
        Vec3::new(st * cp, st * sp, ct)
    }

    pub fn hopf_q(self) -> Q {
        hopf_coordinates(self.theta, self.phi, self.psi)
    }

    /// Unbound witness position. Hopf still wins the slot until bind.
    pub fn pos_unbound(self) -> Vec3 {
        stereographic(self.hopf_q(), 1.0)
    }

    /// Bound faceplate position. Occupancy site on γ, plus one rail:
    /// field 0 → γ + ε ŷ, field 1 → γ + ε ẑ. Hopf fields unchanged.
    pub fn bind_shell(self, trench: &Trench) -> Vec3 {
        trench.gamma(self.shell_s) + self.field().cone_axis() * crate::trench::RAIL_EPS
    }

    pub fn pos(self, trench: Option<&Trench>) -> Vec3 {
        match trench {
            Some(t) => self.bind_shell(t),
            None => self.pos_unbound(),
        }
    }

    pub fn reclassify(&mut self) {
        let kind = classify_section(self.plane_normal(), self.offset, self.field().cone_axis());
        self.set_section(kind);
    }

    /// Witness RGB. Software fact of the projection, not the native store.
    /// Palette is the four inner_cone hues as RGB, scaled by amplitude·persist.
    pub fn rgb_preview(self) -> [f32; 3] {
        let base = match self.section() {
            SectionKind::Elliptic => [0.2, 0.6, 1.0],
            SectionKind::Hyperbolic => [1.0, 0.4, 0.2],
            SectionKind::Parabolic => [1.0, 0.75, 0.2],
            SectionKind::FlatPockets => [1.0, 0.2, 0.8],
        };
        let a = (self.amplitude * self.persist).clamp(0.0, 1.0);
        [base[0] * a, base[1] * a, base[2] * a]
    }

    /// Lock off: interpolate the four hues from the cut, never free RGB.
    pub fn rgb_preview_mix(self) -> [f32; 3] {
        if self.offset.abs() < 1e-4 {
            return scale_rgb([1.0, 0.2, 0.8], self);
        }
        let s = self
            .plane_normal()
            .normalize_or_zero()
            .dot(self.field().cone_axis().normalize_or_zero())
            .abs();
        let g = std::f32::consts::FRAC_1_SQRT_2;
        let hyp = [1.0, 0.4, 0.2];
        let par = [1.0, 0.75, 0.2];
        let ell = [0.2, 0.6, 1.0];
        let rgb = if s <= g {
            lerp3(hyp, par, (s / g).clamp(0.0, 1.0))
        } else {
            lerp3(par, ell, ((s - g) / (1.0 - g)).clamp(0.0, 1.0))
        };
        scale_rgb(rgb, self)
    }

    pub fn to_bytes(self) -> [u8; 32] {
        bytemuck::bytes_of(&self)
            .try_into()
            .expect("QgaPixel is 32")
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        *bytemuck::from_bytes(&bytes)
    }

    pub fn to_json(self) -> String {
        format!(
            "{{\"theta\":{:.6},\"phi\":{:.6},\"psi\":{:.6},\"offset\":{:.6},\"amplitude\":{:.6},\"shell_s\":{:.6},\"persist\":{:.6},\"field\":{},\"section\":\"{}\"}}\n",
            self.theta,
            self.phi,
            self.psi,
            self.offset,
            self.amplitude,
            self.shell_s,
            self.persist,
            self.field().bit(),
            self.section().name()
        )
    }
}

fn lerp3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn scale_rgb(rgb: [f32; 3], p: QgaPixel) -> [f32; 3] {
    let a = (p.amplitude * p.persist.max(0.15)).clamp(0.0, 1.0);
    [rgb[0] * a, rgb[1] * a, rgb[2] * a]
}
