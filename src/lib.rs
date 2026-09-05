//! Phosphor Loom — The Realism Interface (a 3D CRT).
//!
//! This crate owns `qga_pixel` and the two-clock write. It does not own the
//! frame ([qga_gpu]), the observer ([inner_cone]), or the faceplate mesh
//! ([flux_trajectoid]).

pub mod capture;
pub mod clock;
pub mod convert;
pub mod nest;
pub mod pixel;
pub mod scan;
pub mod scene;
pub mod section;
pub mod trench;

pub use clock::{Field, Phosphor, TwoClock};
pub use convert::{to_gpu_particle, to_gpu_particle_on};
pub use pixel::QgaPixel;
pub use section::{classify_section, SectionKind};
pub use trench::{Trench, NEST_DELTA_R, NEST_LAYERS, N_MOTES, N_OCCUPANCY, RAIL_EPS, SPLAT_LOCK};
