//! `qga_pixel` → `GpuParticle`. Lossy on purpose: RGB/hue is a witness.
//!
//! Does not add a GPU record. Software fact: `GpuParticle` is 32 bytes.

use crate::pixel::QgaPixel;
use crate::trench::Trench;
use glam::Vec3;
use qga_gpu::GpuParticle;

/// Unbound: stereographic Hopf lift. Bound: `QgaPixel::bind_shell`.
/// `mass` = amplitude·persist. `vel` = plane normal (what is written).
/// `pad` = section hue for the four-bin particle shader.
pub fn to_gpu_particle(p: QgaPixel) -> GpuParticle {
    to_gpu_particle_on(p, None)
}

pub fn to_gpu_particle_on(p: QgaPixel, trench: Option<&Trench>) -> GpuParticle {
    let mut pos = p.pos(trench);
    if let Some(t) = trench {
        let n = t.normal(p.shell_s);
        pos += n * (0.035 * p.amplitude * p.persist);
    }
    let dir = p.plane_normal();
    let vel = if dir.length_squared() > 1e-12 {
        dir.normalize()
    } else {
        Vec3::Z
    };
    let mass = (p.amplitude * p.persist).clamp(0.0, 4.0);
    GpuParticle::new(pos, vel, mass).with_hue(p.section().hue())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::Field;
    use std::mem::{align_of, size_of};

    #[test]
    fn gpu_slot_is_32() {
        assert_eq!(size_of::<GpuParticle>(), 32);
        assert_eq!(size_of::<QgaPixel>(), size_of::<GpuParticle>());
        assert!(align_of::<QgaPixel>() >= 4);
        assert_eq!(size_of::<QgaPixel>() % 8, 0);
    }

    #[test]
    fn rgb_is_not_the_store() {
        let mut p = QgaPixel::new(0.4, 0.1, 0.0, 0.5, 1.0, Field::Odd, 0.0);
        p.persist = 1.0;
        let addr = (p.theta, p.phi, p.psi);
        let rgb0 = p.rgb_preview();
        p.amplitude = 0.25;
        let rgb1 = p.rgb_preview();
        assert_eq!((p.theta, p.phi, p.psi), addr);
        assert!(rgb0[2] > rgb1[2]); // cyan channel dropped with amplitude
        let gpu = to_gpu_particle(p);
        assert!((gpu.pad - p.section().hue()).abs() < 1e-5);
    }
}
