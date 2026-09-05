# Phase 5 — sibling write, not a gun

**Verdict (v1 closed):** the sidecar did what it was for. Tip `2e1fe9c`.

Still not a window. Still not inverse-Hopf live fibers. The SLM is a *sibling write* of the same 32-byte record into phase.

`vqc_demo` is two machines: HDMI intensity proxy (Sony VPL-HW20A) and a phase-only SLM package. Do not upload projector MP4 frames to an SLM.

## Freeze

| Slot | State |
|---|---|
| Faceplate + bind | Frozen |
| Clocks as a picture | Falsified, do not re-prove |
| Picker as writer | Frozen |
| SLM sidecar + loopback + `generic_512` | Frozen |
| HITL / Pluto / VPL-HW20A | Not v1 |
| Gun | Off |

Rung 1 — software fact. 32 bytes in, 32 bytes out, `crc_ok=true`, `ber=0`, `section=elliptic`, `field=1`, `match=true`. Persist stayed 0.

Rung 2 — model. `output/slm/generic_512/` exists. Manifest cites `shell_s=0.5` and `trench_sha256=746a79b8ba96d2a79ad47b160044b70634bf4819e13e1de8dbaf69f363230f33`. Bias on. No RGB in the phase maps. No Pluto. No projector.

That is a loadable directory, not a sculpture. `make slm-loopback` / `make slm-export` are the only new verbs. `LF` still 0.

Do not open `preview_montage.png` as a new look-dev surface. Do not fold the phase stack into `pick` or `demo`. Do not treat `field=1` in the manifest as a visible tick.

If unfrozen later: one line only — load `output/slm/generic_512/` onto a named panel, or run `vqc_demo` loopback against a captured projector file of the same 32 bytes. New claim label first. Until then `2e1fe9c` is the Phase 5 tip and main is a faceplate, a writer, and a package directory.

## Claims

| Sentence | Label |
|---|---|
| A phase-only SLM maps \([0,2\pi)\) to gray levels. | Theorem / device fact |
| A 32-byte `QgaPixel` is a VQC payload and a quaternion shard. | Model |
| `pick --dump` writes 32 bytes. `vqc_demo slm` writes a phase stack. Sidecar composes them. | Software fact |
| Software loopback recovers that blob with BER 0. | **Hypothesis, tested (v1):** `crc_ok=true`, `ber=0`, `section=elliptic`, `field=1`, 32 bytes, `match=true`. |

## Seam

```
graphics/scripts/export_slm_pixel.py
  reads  output/pick/qga_pixel.bin
  loopback via vqc_demo.pipeline.loopback(bytes)
  optional: flux_trajectoid.export_slm(..., include_shell_bias=True)
  writes graphics/output/slm/<preset>/
```

No Rust path-dep on VQC. No VQC path-dep on `qga_gpu`. Default preset: `generic_512`. Do not name `holoeye_pluto_2` until a panel exists.

## Blob → VQC

| `QgaPixel` | VQC / SLM |
|---|---|
| 32 raw bytes | `--payload` / codec frame |
| `hopf_q()` / \((\theta,\phi,\psi)\) | quaternion shard / \(S^3\) carrier |
| `section` (2 bits) | LG family / Braille cell class |
| `field` (1 bit) | two-frame SLM stack metadata, **not** a visible CRT tick |
| `shell_s` | cited in the package manifest + `assets/shell_trench.bin` identity |
| `amplitude` | PWM duty / shard length |
| `offset` | eccentricity → mode mix |
| `persist=0` on export | age stays on the faceplate |

Do not invent a 33rd byte. Do not put RGB in the phase map. Occupancy 256 and \(\varepsilon=0.02R\) stay faceplate-side.

## Rungs

1. **Software fact (v1, closed).** Loopback on the dump returns the same JSON fields (`section`, `field`, 32 bytes). BER 0. Persist 0. No projector.
2. **Model (v1, closed).** `export_slm` + `include_shell_bias=True` → `generic_512` package. Manifest cites `shell_s=0.5` and trench `746a79b8…`. Loadability is the claim.
3. **HITL.** `hitl --channel projector` or a real Pluto. Not v1. The VPL-HW20A cannot emit OAM.

## Not wired

- Faceplate PNG / `tick.mp4` / `03_both.png` onto an SLM
- `preview_montage.png` as a look-dev surface
- `make pick` growing an SLM preview, or folding `phase_stack.npy` into `pick` / `demo`
- Turning `LF` on because “the SLM is the gun”
- Treating `field=1` in the manifest as a visible CRT tick
- Unlocked continuous \(S^2\) color into phase
- Sign mask on the **loopback** blob (mask is seed-only; loopback stays unmasked)
