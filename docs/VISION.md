# vision_tracker

`vision_tracker` is the sculpture’s eyeline lock onto observer \(S^2\). Device noun, like `qga_pixel`. Not a second gun. Not stacked clear windows. Not an all-seeing global pointer.

**Logged.** Tip `6dea5f2` is calibration only. `--webcam` stays off. No pointer mesh. No `LF`. Faceplate does not read `gaze.json`. Scan A does not consume `site`. Nothing else unfreezes.

Faceplate may read `gaze.json` only when you unfreeze **Scan A consumes site**. Until that line exists, calibration is the whole implementation.

## Claim

| Sentence | Label |
|---|---|
| A camera can estimate gaze or head pose relative to a calibrated sculpture frame. | Device fact (cameras in general) |
| That pose samples observer \(S^2\); cones and `qga_pixel` already take \((\theta,\phi)\). | Model |
| A calibrated camera maps a pixel to a ray in the sculpture frame. | Device fact (cameras in general) |
| Viewer position \(E\) samples observer \(S^2\) as \(\hat E\). Default eyeline looks at the origin. | Model |
| `calibrate` / `gaze` write `calib.json` and `{theta,phi,locked,site}`. Bench `rms_px=1.03e-4` on six **projected** hull verts. | **Software fact of the bench solver**, not of a lens |
| `gaze --eye-px 640,360` → `site=240`, even, `layer=0`, `persist=0`, `locked=true` | Software fact of the payload |
| `make track-synth` `uv` is this sculpture’s pose | **Not a device fact** until `uv` comes from a still of that camera |
| Gaze-driven Scan A head on \(\gamma\), layer 0, `LF=0`, reads as a pointer. | Hypothesis — **not this unfreeze** |
| Eyeline lock makes the two-clock or radial stack read without a caption. | **Not claimed.** Clocks and N1 already failed at `4.2`. |

v1 is **eyeline lock from head/eye position**, not pupil gaze. Pupils are a later device fact.

## Machine

Frozen equation:

\[
\text{observer }S^2 \to \text{dual cones} \to \text{loom} \to \text{qga\_pixel on }\gamma \to \text{two clocks} \to \text{eye}
\]

`vision_tracker` is the first arrow made physical:

\[
\text{camera on the hull} \to (\theta,\phi)_{\text{eye}} \in S^2 \to \text{same observer slot inner\_cone already has}
\]

It may write \((\theta,\phi)\), `locked`/`lost`, and optionally the nearest trench site \(i_\star\). It may not write `LF`, fiber count, glow, `layer` composition, `section`, persist as a gaze fade, or a pointer mesh.

Reconfigure the signal, not the sculpture. Legal later actuator: Scan A head follows \(i_\star\) on layer 0. Illegal: loom blooms wherever they look.

## Geometry

Sculpture origin = hull / cone apex. Camera atop the hull looks *out* at the viewer.

\[
\mathbf{x}_{\text{cam}}=R\mathbf{X}+t,\qquad
\text{ray through pixel }(u,v):\ \mathbf{o}=C,\ \mathbf{d}=R^\top K^{-1}(u,v,1)
\]

\(C=-R^\top t\). v1 observer sample: eye center \(E\) (midpoint of two landmarks, or `--eye-xyz` / `--eye-px`).

\[
\hat E=\frac{E}{\|E\|},\qquad
\theta=\arccos \hat E_z,\quad
\phi=\operatorname{atan2}(\hat E_y,\hat E_x)
\]

Hit on the trench: nearest occupied even site on layer 0 (`N_OCCUPANCY=256`):

\[
i_\star=\arg\min_{i\ \mathrm{even}}\ \bigl(1-\hat\gamma(s_i)\cdot\hat E\bigr)
\]

That \(i_\star\) is the only legal later actuator. Do not spawn a cursor mesh.

Do not calibrate against phosphor motes. You will lock to splat.

## Placement

Sidecar, same family as `export_slm_pixel.py`. Not inside `qga_gpu` present. No path-dep on `qga_gpu`. No window.

```
sculpture
  cam (top of hull, looking out at the viewer)
  vision_tracker  →  output/track/gaze.json
pick / faceplate  →  does not read gaze.json yet
Scan A           →  head = site  (paper until unfrozen)
```

```
scripts/vision_tracker.py
docs/VISION.md
output/track/calib.json      # after calibrate
output/track/gaze.json       # after gaze
output/track/points.json     # 3D–2D pairs
```

## Protocol

1. Intrinsics (once per lens): checkerboard → `K`, `dist`. Or load a factory YAML / JSON.
2. Six or more hull marks whose sculpture-frame XYZ you know (vertices in `shell_trench.bin`). Click them in one still from the top camera → `points.json`.
3. `calibrate` runs `solvePnP` + reprojection RMS. Fail if `rms_px > 2`.
4. `gaze --eye-px u,v` or `--eye-xyz x,y,z` writes `gaze.json`. `--webcam` is not v1.

Bench path without a live still: `synth-points` projects six hull verts through a known pose so `solvePnP` has a measured `rms_px`. First run: `rms_px=1.03e-4`, \(N=6\), fail bar is \(2.0\). That number is a **software fact of the bench solver**, not of a lens. `make track-synth` cannot become a device fact until `uv` comes from a still of that camera. Replace the synthetic `uv` before you treat `calib.json` as that sculpture’s pose.

Rate, later: 30 Hz names the clock; `LF` stays 0.

Privacy: log pose, not faces, if this ever leaves the bench.

## Makefile

```
make track-synth      # hull verts → output/track/points.json (bench)
make track-calibrate  # solvePnP → output/track/calib.json
make track-gaze EYE=640,360
```

## Must not

- Open a webcam loop in v1
- Draw a pointer, fibers, or layers 1–2
- Touch `packed` beyond reporting `layer: 0` and `site`
- Claim clocks or N1 layers are now visible
- Store faces — only \(\theta,\phi,E,site\)
- Fold `gaze.json` into `pick` or `demo` until **Scan A consumes site**
