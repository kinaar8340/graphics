# Phosphor Loom

**The Realism Interface** — *a 3D CRT*

**Repo / crate / folder name:** `shellscan`  
**Public title:** Phosphor Loom  
**Sculpture series:** The Realism Interface  
**Subtitle:** a 3D CRT  
**Type (not a repo):** `qga_pixel`

Remote: [github.com/kinaar8340/graphics](https://github.com/kinaar8340/graphics).  
The checkout **should** be `~/Projects/shellscan`. This tree is still `~/Projects/graphics` until the directory is renamed. Do not create a second repo. `qga_pixel` is the type, not a repo. Crate name: `shellscan`.

A CRT writes a glowing surface by sweeping a beam in two fields. This crate writes a glowing shell by lifting a chart through inverse-Hopf in two clocks. The pixel is a local plane-cut of the observer’s double cone. Color is a conic type, not an RGB triple. The shell is a trajectoid, so the screen has an identity and a trench, not a rectangle.

`qga_gpu` owns the frame. `inner_cone` owns the observer. `flux_trajectoid` owns the shell. The only new noun is `qga_pixel`.

This is a **Model** and a **Software fact**. It is not a vacuum tube and not a theorem of displays. Claims: [docs/SPEC.md](docs/SPEC.md).

## Status

- [x] Phase 0 — spec with four claim labels
- [x] Phase 1 — 32-byte `qga_pixel` (section, Hopf address, field bit, RGB preview)
- [x] Phase 2 — two-clock CPU write + persistence tests
- [x] Phase 3 — offline trench bind; headless 8-frame; then a `demo-tiny`-scale window
- [x] Phase 4 — closed. Faceplate pass, clock fail (tested: not visible at sculpture distance), gun deferred
- [ ] Phase 5 — optional SLM / photonic sibling (later)
- [x] Phase 6 — picker: writes 32-byte `qga_pixel` (`make pick`). Not a second loom.

Hypothesis (two clocks as a picture): **tested, not visible at sculpture distance.** See [docs/TESTIMONY.md](docs/TESTIMONY.md). Splat vs `ε`: [docs/LAYOUT.md](docs/LAYOUT.md).

## `qga_pixel`

32 bytes, same slot as `GpuParticle`:

| Field | Meaning |
|---|---|
| `theta, phi` | S² cell: cutting-plane tilt (hue class) |
| `psi` | fiber phase (subpixel scan) |
| `offset` | plane offset (eccentricity / saturation) |
| `amplitude` | shard length. Not R, G, or B |
| `shell_s` | trench parameter (Phase 3) |
| `persist` | mote age (`FluxMote.far_age` analog) |
| packed | field bit + section (2 bits) |

RGB is `rgb_preview()`, a witness projection onto the inner_cone four-bin palette (cyan / gold / orange / magenta). Do not invent a fifth.

Odd field → visual cone (`+Z`). Even field → feeling cone (`+Y`). Separator = blanking.

## Test / run

```
make test          # CPU: record, clocks, trench bind
make headless      # 8 frames, 4k elliptic, static_uploads == 1
make demo          # demo-tiny-scale window; live_every = 30; gun off; HUD on
make stills        # five 1280×720 PNGs, no HUD, glow off
make tick          # locked-eye 6s: 2s even / 2s odd / 2s both → output/mp4/tick.mp4
make testimony     # stills + 30s orbit of 03_both (body, not clocks)
make pick          # Phase 6 writer: hemisphere → 32-byte qga_pixel (not the faceplate)
                   # click hemisphere; L lock-to-4; [ ] shell_s; F field bit (packed, not a picture); E export
                   # cargo run --release --bin pick -- --dump  # no window, writes output/pick/
```

Sidecar (once, not in the frame loop):

```
make export-shell  # PYTHONPATH sibling flux_trajectoid → assets/shell_trench.bin
```

Sibling checkouts:

```
~/Projects/qga_engine
~/Projects/qga_gpu
~/Projects/flux_trajectoid   # sidecar only
~/Projects/shellscan         # this crate (currently ~/Projects/graphics)
```

## Related

| Repo | Role |
|---|---|
| [graphics](https://github.com/kinaar8340/graphics) | this crate (Phosphor Loom / `shellscan`) |
| [qga](https://github.com/kinaar8340/qga) | manuscript |
| [qga_engine](https://github.com/kinaar8340/qga_engine) | math / sim |
| [qga_gpu](https://github.com/kinaar8340/qga_gpu) | frame |
| [inner_cone](https://github.com/kinaar8340/inner_cone) | observer / cones |
| [flux_trajectoid](https://github.com/kinaar8340/flux_trajectoid) | faceplate |
| [vqc_demo](https://github.com/kinaar8340/vqc_demo) | later photonic write |

## License

MIT — same ecosystem as qga / qga_gpu / inner_cone.
