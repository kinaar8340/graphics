# Phase 5 — sibling write, not a gun

Still not a window. Still not inverse-Hopf live fibers. The SLM is a *sibling write* of the same 32-byte record into phase.

`vqc_demo` is two machines: HDMI intensity proxy (Sony VPL-HW20A) and a phase-only SLM package. Do not upload projector MP4 frames to an SLM.

## Claims

| Sentence | Label |
|---|---|
| A phase-only SLM maps \([0,2\pi)\) to gray levels. | Theorem / device fact |
| A 32-byte `QgaPixel` is a VQC payload and a quaternion shard. | Model |
| `pick --dump` writes 32 bytes. `vqc_demo slm` writes a phase stack. Sidecar composes them. | Software fact |
| Software loopback recovers that blob with BER 0. | **Hypothesis, tested:** `crc_ok=true`, `ber=0`, `section=elliptic`, `field=1`, 32 bytes. |

Rung 1 passed. Deliverable for rung 2 is a package directory (`output/slm/generic_512/`), not a new sculpture. HITL is not v1.

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

1. **Software fact (v1).** Loopback on the dump returns the same JSON fields (`section`, `field`, 32 bytes). BER 0. No projector.
2. **Model.** `export_slm` + `include_shell_bias=True` → `generic_512` package. Manifest cites `shell_s` and the trench bin identity. Loadability is the claim.
3. **HITL.** `hitl --channel projector` or a real Pluto. Not v1. The VPL-HW20A cannot emit OAM.

## Not wired

- Faceplate PNG / `tick.mp4` / `03_both.png` onto an SLM
- `make pick` growing an SLM preview
- `make demo` presenting donuts
- Turning `LF` on because “the SLM is the gun”
- Unlocked continuous \(S^2\) color into phase
