# `qga_pixel` vs `FluxMote` vs `GpuParticle`

Software fact of sizes. Meaning is Model.

## Sizes

| Record | Bytes | Owner |
|---|---|---|
| `QgaPixel` | 32 | shellscan (this crate) |
| `GpuParticle` | 32 | qga_gpu |
| `FluxMote` | fat CPU | inner_cone |

Do not add a fourth GPU vertex record.

## Native store (`QgaPixel`)

```
theta     f32   S² polar of the cutting plane (tilt → hue class)
phi       f32   S² azimuth
psi       f32   fiber phase (subpixel scan)
offset    f32   n·x = p (eccentricity)
amplitude f32   shard length — not R, G, or B
shell_s   f32   trajectoid trench parameter (Phase 3)
persist   f32   mote age
packed    u32   field:1 | section:2
```

RGB is `rgb_preview()`, a projection onto the four inner_cone hues.

## Map onto `FluxMote` (inner_cone, not a dep)

| FluxMote | QgaPixel |
|---|---|
| `pos` | see bind cases below |
| `vel` | `plane_normal()` |
| `kind` | `section()` → Elliptic/Parabolic/Hyperbolic/FlatPockets |
| `q` | `hopf_q()` |
| `far_age` | `persist` |
| `pierced` | hyperbolic + persist on the far cone (not packed yet) |
| `twist` / `omega` | not in the 32-byte slot |

## `pos` bind (Phase 3)

`QgaPixel` does not store `pos`. Conversion picks:

\[
\text{pos} = \begin{cases}
\sigma(\mathrm{hopf}(\theta,\phi,\psi)) & \text{unbound}\\
\gamma(s)+\varepsilon\,\hat y & \text{bound, field 0}\\
\gamma(s)+\varepsilon\,\hat z & \text{bound, field 1}
\end{cases}
\]

`γ` is the trench-parameterized curve on the offline mesh (`assets/shell_trench.bin`).
`θ,φ,ψ` still decide *what* is written. `shell_s` decides *where it glows*.
`QgaPixel::bind_shell` returns the rail and does not touch Hopf fields.
Optional grain: offset along the local radial normal by a fraction of `amplitude * persist`.
Motes that leave the trench are a bug.

**Splat vs ε (Software fact).** `RAIL_EPS = 0.02` (units of shell radius R ≈ 1). On `04_trench` the witness particle splat at persist=1 is larger than `0.02R`, so the rail slot is not air in the picture. Do not shrink splat radius to flatter the Model. Do not grow ε past this until it reads as two meshes.

Splat radius at the locked eye (distance \(4.2\), aperture \(1\), head mass \(1\)): \(\mathrm{mix}(0.018,0.072,0.5)\cdot 0.70 = 0.0315R\). Nested \(\Delta R=0.08R\) must exceed that number or layers collapse the same way. Packed `layer:8`. \(L=3\) N1. See [NEST.md](NEST.md).

Occupancy and rails stay **faceplate-side** (`N_OCCUPANCY=256`, `ε=0.02R`). A Phase 6 picker may export the field bit; it does not own the trench.

Do not path-dep `flux_trajectoid`. The sidecar writes bytes; Rust reads bytes.

Phase 5 v1 is a package directory (`output/slm/generic_512/`), not a sculpture. Do not fold `phase_stack.npy` into `pick` or `demo`. `field` in the SLM manifest is packed metadata, not a visible tick. Do not open `preview_montage.png` as look-dev.

Animation A lights even occupancy sites only. Occupancy table, rails, and splat stay put. Persist assignment does not call the two-clock field write.

`exited` / `pierced` stay out of the 32-byte slot.

## Map onto `GpuParticle` (lossy, `to_gpu_particle`)

| GpuParticle | QgaPixel |
|---|---|
| `pos` | unbound: stereographic lift; bound: `bind_shell` |
| `mass` | `amplitude * persist` |
| `vel` | unit plane normal |
| `pad` | `section().hue()` (0.55 / 0.10 / 0.30 / 0.80) |

The particle shader's four-bin wheel is the witness camera. It is not the store.

## Two clocks

| Clock | Field | Cone | Cadence |
|---|---|---|---|
| 0 | Even | feeling `+Y` | field write every even frame |
| 1 | Odd | visual `+Z` | field write every odd frame |
| static | — | hull / lattice | `static_uploads = 1` |
| live fiber | — | gun | `live_every` (2 in CPU tests; **30 in the window**; gun **off** in first window) |

Do not collapse interlace clocks and upload clocks into one integer. HUD names them apart.

Renderer (Y-up) vs sketch: visual cone along renderer `+Y` is sketch `+Z`; feeling cone `rotated_x(π/2)` is sketch `+Y`. Classification uses sketch axes (`Field::cone_axis`).
