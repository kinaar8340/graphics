//! Offline faceplate. Software fact of the byte layout, not a Python runtime.
//!
//! `γ(s)` is the contact trench. Bound `pos = γ(shell_s)`.

use anyhow::{bail, Context, Result};
use glam::Vec3;
use qga_gpu::FaceVert;
use std::path::Path;

pub const MAGIC: &[u8; 4] = b"SHSC";
pub const VERSION: u32 = 1;
/// γ table in `shell_trench.bin`. Not the occupancy grid.
pub const N_MOTES: usize = 4096;
/// Occupancy interlace. Field 0 owns even sites, field 1 odd sites.
/// 4096 even/odd is occupancy in the record and one stroke in the picture.
/// 256 sites (128+128) is dense enough to be a trench and sparse enough to dash.
pub const N_OCCUPANCY: usize = 256;

/// Contact trench + dim shell skin. Faces are the envelope, not a second phosphor.
#[derive(Clone, Debug)]
pub struct Trench {
    pub verts: Vec<Vec3>,
    pub faces: Vec<[u32; 3]>,
    pub samples: Vec<Vec3>,
}

impl Trench {
    pub fn circle(n: usize) -> Self {
        let n = n.max(8);
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let a = i as f32 / n as f32 * std::f32::consts::TAU;
            samples.push(Vec3::new(a.cos(), 0.0, a.sin()));
        }
        Self {
            verts: samples.clone(),
            faces: Vec::new(),
            samples,
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let bytes = std::fs::read(path.as_ref())
            .with_context(|| format!("read {}", path.as_ref().display()))?;
        Self::from_bytes(&bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 16 || &bytes[0..4] != MAGIC {
            bail!("shell_trench.bin: bad magic");
        }
        let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        if version != VERSION {
            bail!("shell_trench.bin: version {version}");
        }
        let n_verts = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
        let n_faces = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
        let n_trench = u32::from_le_bytes(bytes[16..20].try_into().unwrap()) as usize;
        let mut off = 20usize;
        let verts = read_vec3(&bytes, &mut off, n_verts)?;
        let mut faces = Vec::with_capacity(n_faces);
        let need = n_faces.checked_mul(12).context("faces overflow")?;
        if off + need > bytes.len() {
            bail!("faces truncated");
        }
        for _ in 0..n_faces {
            let a = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
            let b = u32::from_le_bytes(bytes[off + 4..off + 8].try_into().unwrap());
            let c = u32::from_le_bytes(bytes[off + 8..off + 12].try_into().unwrap());
            faces.push([a, b, c]);
            off += 12;
        }
        let samples = read_vec3(&bytes, &mut off, n_trench)?;
        if samples.len() < 2 {
            bail!("trench too short");
        }
        Ok(Self {
            verts,
            faces,
            samples,
        })
    }

    /// `s` in [0, 1). Closed loop.
    pub fn gamma(&self, s: f32) -> Vec3 {
        let n = self.samples.len();
        let t = s.rem_euclid(1.0) * n as f32;
        let i = t.floor() as usize % n;
        let j = (i + 1) % n;
        let f = t.fract();
        self.samples[i].lerp(self.samples[j], f)
    }

    pub fn normal(&self, s: f32) -> Vec3 {
        self.gamma(s).normalize_or_zero()
    }

    pub fn dim_faces(&self) -> Vec<FaceVert> {
        let color = [0.18, 0.22, 0.28];
        let alpha = 0.28;
        let mut out = Vec::with_capacity(self.faces.len() * 3);
        for &[ia, ib, ic] in &self.faces {
            let a = self.verts.get(ia as usize).copied().unwrap_or(Vec3::ZERO);
            let b = self.verts.get(ib as usize).copied().unwrap_or(Vec3::ZERO);
            let c = self.verts.get(ic as usize).copied().unwrap_or(Vec3::ZERO);
            let nrm = (b - a).cross(c - a).normalize_or_zero();
            for p in [a, b, c] {
                out.push(FaceVert {
                    pos: p.into(),
                    alpha,
                    color,
                    pad: 0.0,
                    nrm: nrm.into(),
                    pad2: 0.0,
                });
            }
        }
        out
    }

    pub fn trench_edges(&self) -> Vec<[Vec3; 2]> {
        let n = self.samples.len();
        (0..n)
            .map(|i| [self.samples[i], self.samples[(i + 1) % n]])
            .collect()
    }
}

fn read_vec3(bytes: &[u8], off: &mut usize, n: usize) -> Result<Vec<Vec3>> {
    let need = n.checked_mul(12).context("vec3 overflow")?;
    if *off + need > bytes.len() {
        bail!("vec3 truncated");
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let x = f32::from_le_bytes(bytes[*off..*off + 4].try_into().unwrap());
        let y = f32::from_le_bytes(bytes[*off + 4..*off + 8].try_into().unwrap());
        let z = f32::from_le_bytes(bytes[*off + 8..*off + 12].try_into().unwrap());
        out.push(Vec3::new(x, y, z));
        *off += 12;
    }
    Ok(out)
}
