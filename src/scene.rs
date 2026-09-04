//! demo-tiny hull + dim shell skin. Not inner_cone's flux rivers.
//!
//! Sketch visual +Z / feeling +Y (SPEC π/2 about X is the same pair).
//! Renderer is Y-up: visual cone along +Y, feeling cone rotated_x(π/2).

use crate::trench::Trench;
use glam::Vec3;
use qga_gpu::{FaceVert, GpuFiber, Mesh};

pub fn hull_meshes() -> Vec<Mesh> {
    vec![
        Mesh::sphere(0.35).colored([0.75, 0.82, 0.95]),
        // Visual / odd / sketch +Z → renderer +Y (upright cyan).
        Mesh::cone(0.55, 0.95).colored([0.20, 0.60, 1.00]),
        // Feeling / even / sketch +Y → renderer +Z.
        Mesh::cone(0.42, 0.75)
            .rotated_x(std::f32::consts::FRAC_PI_2)
            .colored([1.00, 0.40, 0.20]),
        Mesh::torus(1.05, 0.03).colored([0.95, 0.85, 0.20]),
    ]
}

pub struct StaticHull {
    pub faces: Vec<FaceVert>,
    pub edges: Vec<[Vec3; 2]>,
    pub fibers: Vec<GpuFiber>,
}

/// Tessellate hull once, append dim shell faces + trench polyline.
pub fn static_hull(trench: &Trench, lod: u32) -> StaticHull {
    let mut faces = Vec::new();
    let mut edges = Vec::new();
    let mut fibers = Vec::new();
    for m in hull_meshes() {
        let t = m.tessellate(lod);
        faces.extend(t.faces);
        edges.extend(t.edges);
        fibers.extend(t.fibers);
    }
    faces.extend(trench.dim_faces());
    edges.extend(trench.trench_edges());
    StaticHull {
        faces,
        edges,
        fibers,
    }
}
