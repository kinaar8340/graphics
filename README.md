# Phosphor Loom

**The Realism Interface** — *a 3D CRT*

| | |
|---|---|
| Public title | Phosphor Loom |
| Sculpture | The Realism Interface |
| Subtitle | a 3D CRT |
| Crate | `shellscan` |
| Type (not a repo) | `qga_pixel` |
| Checkout | still `~/Projects/graphics` until renamed to `~/Projects/shellscan` |
| Remote | [github.com/kinaar8340/graphics](https://github.com/kinaar8340/graphics) |

A CRT writes a glowing surface by sweeping a beam in two fields. This crate writes a glowing shell by lifting a chart through inverse-Hopf in two clocks. The pixel is a local plane-cut of the observer’s double cone. Color is a conic type, not an RGB triple. The shell is a trajectoid, so the screen has an identity and a trench, not a rectangle.

Four layers of one machine, not a mashup render:

| Layer | Role |
|---|---|
| CRT | scan ritual (two-field ledger, persist as age, separator as blanking) |
| Inverse-Hopf loom | write path / address \((\theta,\phi,\psi)\) |
| Plane ∩ double cone | pixel algebra (section) |
| Trajectoid shell | faceplate \(\gamma(s)\) |

`qga_gpu` owns the frame. `inner_cone` owns the observer. `flux_trajectoid` owns the shell (sidecar only). The only new noun is `qga_pixel`.

This is a **Model** and a **Software fact**. It is not a vacuum tube and not a theorem of displays. Claims: [docs/SPEC.md](docs/SPEC.md).

## Freeze

Tip of the sign-mask fact: `c5c5095`. Working rule: unfreeze is one labeled sentence, not a new sculpture.

| Slot | State |
|---|---|
| Faceplate + bind | Frozen |
| Two clocks as a picture | Tested, not visible at sculpture distance |
| Picker as writer | Frozen |
| SLM sidecar + `generic_512` + `--mask` | Frozen (loopback on unmasked blob) |
| N1 nested layers in composition | Tested, not visible |
| `vision_tracker` calib + AttentionWell | Paper + bench solver; not a lens |
| Scan A consumes `gaze.site` | Frozen |
| Gun / `LF` | Off |
| HITL / named panel | Not v1 |

Splat at \(4.2\), persist \(=1\): \(0.0315R\). Rails \(0.02R\) and \(\Delta R=0.08R\) both lost as pictures.

Do not: Scan A consume `site`, HITL Pluto, nested draw, `LF`, stacked clear windows, wetware pine in `demo`.

## Status

- [x] Phase 0 — spec with four claim labels
- [x] Phase 1 — 32-byte `qga_pixel`
- [x] Phase 2 — two-clock CPU write + persistence tests
- [x] Phase 3 — offline trench bind; headless 8-frame; `demo-tiny`-scale window
- [x] Phase 4 — faceplate pass, clock fail, gun deferred. [docs/TESTIMONY.md](docs/TESTIMONY.md)
- [x] Phase 5 v1 — `vqc_demo` loopback BER 0; `generic_512` (`shell_s=0.5`, trench `746a79b8…`). HITL not v1. [docs/PHASE5.md](docs/PHASE5.md)
- [x] Phase 6 — `make pick` writes the blob. Not a second loom
- [ ] Animation A — scan head on \(\gamma(s)\), layer 0. [docs/SCAN.md](docs/SCAN.md)
- [x] N1 — three confocal trenches, ledger pass, composition fail. [docs/NEST.md](docs/NEST.md)
- [ ] `vision_tracker` — real lens \(K\) + still. Well is specified. Scan A does not consume `site`. [docs/VISION.md](docs/VISION.md)
- [x] Sign mask — `--mask {none,antipode,blank,nappe}` on the SLM seed. [docs/SIGN.md](docs/SIGN.md)

End goal compatible with the freeze: a pipeline that emits a drive signal (record → dump → phase package → later a named panel / glass). Not a single Vulkan window that is a 4D OAM crystal ball.

The only box still worth ticking inside this crate is `make scan`. Everything else is a new claim or another repo.

## `qga_pixel`

32 bytes, same slot as `GpuParticle`:

| Field | Meaning |
|---|---|
| `theta`, `phi` | \(S^2\) cell: cutting-plane tilt (hue class) |
| `psi` | fiber phase (subpixel scan). Antipode mask is \(\psi+\pi\) on the SLM seed only |
| `offset` | plane offset (eccentricity / saturation) |
| `amplitude` | shard length. Blank mask sets seed amp \(0\). Not R, G, or B |
| `shell_s` | trench parameter. Do not fold layer into this |
| `persist` | mote age. Export dumps \(0\) |
| packed | `field:1 \| section:2 \| layer:8` |

RGB is `rgb_preview()`, a witness projection onto the inner_cone four-bin palette (elliptic cyan \(0.55\) / parabolic gold \(0.10\) / hyperbolic orange \(0.30\) / flat magenta \(0.80\)). No fifth hue. Antipode does not change that preview.

Odd field → visual cone (`+Z`). Even field → feeling cone (`+Y`). Separator = blanking. Elliptic = compact, no pierce, this side of the separator.

After bind: `pos = γ(shell_s)`. Occupancy \(256\). `RAIL_EPS = 0.02R` (collapsed at sculpture distance).

## Test / run

```
make test             # CPU: record, clocks, trench bind
make headless         # 8 frames, elliptic, static_uploads == 1
make demo             # tiny window; live_every = 30; gun off
make stills           # locked-eye PNGs, no HUD, glow off
make tick             # 6s locked strip (clocks-as-picture: failed)
make scan             # Animation A: persist peak, lock + crop
make nest-headless    # N1 ledger
make nest-stills      # N1 composition stills (picture: failed)
make testimony        # body stills + orbit of 03_both
make pick             # writer: hemisphere → output/pick/qga_pixel.bin
make slm-loopback     # recover dump; BER 0 on unmasked blob
make slm-export       # generic_512; MASK=none
make slm-mask         # antipode seed (ψ+π)
make track-synth      # bench points.json (not a live still)
make track-calibrate  # solvePnP → calib.json
make track-gaze       # EYE=u,v → gaze.json (not consumed)
```

Sidecar (once, not in the frame loop):

```
make export-shell     # sibling flux_trajectoid → assets/shell_trench.bin
```

Picker: click hemisphere; `L` lock-to-4; `shell_s`; `F` field bit (packed, not a picture); `E` export.

```
cargo run --release --bin pick -- --dump
```

SLM mask:

```
make slm-export MASK=none       # plain package
make slm-mask                   # antipode seed
# qga_pixel.bin        = dump
# qga_pixel_seed.bin   = masked seed
```

## Attention well (tracker paper)

Default \(\hat E_{\mathrm{def}}=(0,0,1)\), \((\theta,\phi)_{\mathrm{def}}=(0,0)\), \(i_{\mathrm{def}}=40\) (even site nearest \(+\hat z\) on trench `746a79b8…`).

\(\Delta t=1/30\,\mathrm{s}\), \(\alpha_{\min}=3^\circ\), \(T_{\mathrm{off}}=2\,\mathrm{s}\), \(\tau=0.8\,\mathrm{s}\), \(\beta=0.3\). Presence inside the dead zone keeps attention. Loss of blob starts the timer, then leak. Unmatched samples dropped. `locked` ≠ `attention_state`. Faceplate does not read `gaze.json`.

## Related

| Repo | Role |
|---|---|
| [graphics](https://github.com/kinaar8340/graphics) | this crate |
| [qga](https://github.com/kinaar8340/qga) | manuscript |
| [qga_engine](https://github.com/kinaar8340/qga_engine) | math / sim |
| [qga_gpu](https://github.com/kinaar8340/qga_gpu) | frame |
| [inner_cone](https://github.com/kinaar8340/inner_cone) | observer / cones |
| [flux_trajectoid](https://github.com/kinaar8340/flux_trajectoid) | faceplate sidecar |
| [vqc_demo](https://github.com/kinaar8340/vqc_demo) | loopback + SLM package |
| [vqc_proto](https://github.com/kinaar8340/vqc_proto) | LG / typehead (sibling, not a path-dep) |
| [wetware_printer](https://github.com/kinaar8340/wetware_printer) | Hopf stylus grower — stay in that crate |
| [pic](https://github.com/kinaar8340/pic) | RubikConeConduit noun; \(\sqrt{3}-1\) is identity topology, not \(\Delta R\) |

## License

MIT — same ecosystem as qga / qga_gpu / inner_cone.
