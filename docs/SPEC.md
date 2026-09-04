# Phosphor Loom — one-page spec

**Sculpture:** The Realism Interface  
**Subtitle:** a 3D CRT  
**Crate:** `shellscan`  
**New noun:** `qga_pixel`

This is a thin scene crate in the inner_cone pattern. It does not own the frame, the observer, or the faceplate mesh.

## Claims

| Sentence | Label |
|---|---|
| The Hopf fibration \(S^3\to S^2\) exists. A plane cutting a double cone traces ellipse, parabola, hyperbola, or a degenerate pair of lines. | **Theorem** (cited) |
| Dual-cone observer (visual cyan \(+Z\), feeling orange, \(\pi/2\) about \(X\)). Four flux kinds are color classes. Trajectoid shell is the faceplate. Odd field writes the visual cone; even field writes the feeling cone. Separator torus is blanking. Persistence is mote age, not an RGB fade. | **Model** |
| `qga_gpu` owns the frame. Loom writes. Particle records are 32 bytes. `static_uploads` / `live_fiber_writes` / `particle_skipped` are upload counters. RGB in the particle shader is a witness projection. | **Software fact** |
| Two-clock interlacing on those cones will read as a display rather than a particle demo. | **Hypothesis** |

If a sentence cannot wear one of those four tags, it does not belong here.

## Ownership (do not collapse)

| Layer | Owner |
|---|---|
| Algebra / Hopf / IslandType | [qga](https://github.com/kinaar8340/qga) / [qga_engine](https://github.com/kinaar8340/qga_engine) `qga-math` |
| Frame, loom, particles, capture | [qga_gpu](https://github.com/kinaar8340/qga_gpu) |
| Observer, cones, separator, `FluxMote.far_age` | [inner_cone](https://github.com/kinaar8340/inner_cone) |
| Trajectoid mesh, trench, fingerprint | [flux_trajectoid](https://github.com/kinaar8340/flux_trajectoid) |
| `qga_pixel` record + two-clock write | **this crate** |

## Machine

\[
\text{observer } S^2 \to \text{dual cones} \to \text{inverse-Hopf loom} \to \text{qga\_pixel field on a trajectoid shell} \to \text{two-clock persistence} \to \text{eye}
\]

Keep: gun (inverse-Hopf), two-field clock, persistence, faceplate ≠ framebuffer.  
Drop: rectangular raster as ontology, three independent guns, progressive “more pixels,” vacuum-tube claims, Hopf-as-display-theorem.

## Palette (do not invent a fifth)

| Section | inner_cone kind | hue (`GpuParticle.pad`) |
|---|---|---|
| elliptic | `FluxKind::Elliptic` | 0.55 cyan |
| parabolic | `FluxKind::Parabolic` | 0.10 gold |
| hyperbolic | `FluxKind::Hyperbolic` | 0.30 orange |
| degenerate / flat-pocket | `FluxKind::FlatPockets` | 0.80 magenta |

## Phases

0. This page.  
1. `qga_pixel` 32-byte record. Encode section, Hopf address, field bit. Project RGB for the witness camera. **No scene.**  
2. Two-clock write. Clock 0 static / even / feeling. Clock 1 live / odd / visual. Persistence difference must be measurable.  
3. **Shipped.** Bind + window. Offline trench, 4k elliptic, `pos = γ(shell_s)`. Headless 8-frame: `SU=1`, `LF=0`, `PS=0`, `even=1347.844`, `odd=1872.004`, `both=3219.883`. `demo-tiny`-scale window. Gun off.  
4. Testimony stills. Faceplate pass, clock fail, gun deferred. Occupancy interlace next: even sites field 0, odd sites field 1. No HUD, no `G`, `LF=0`. See [TESTIMONY.md](TESTIMONY.md).  
5. Optional SLM / `vqc_demo` photonic sibling.

## First scene (Phase 3, shipped)

One window. No mosaic. One observer. Two cones. One separator. One shell. 4k elliptic motes on `γ(s)`. Two clocks (interlace every frame; `live_every = 30`; gun off). HUD: field bit, `ELLIPTIC`, `SU` / `LF` / `PS`. Orbit camera only. `static_uploads == 1`.
