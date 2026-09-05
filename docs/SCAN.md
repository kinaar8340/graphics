# Animation A — scan head

**Unfreeze:** animation A. One persist peak walking \(\gamma(s)\). Not a gun. Not a second fiber. Not a class change. Caption lives here, not in the framebuffer.

| Sentence | Label |
|---|---|
| A single persist peak advancing on \(\gamma(s)\), elliptic lock, LF=0, reads as a scanline on the faceplate. | Hypothesis |
| Head site is argmax persist; tail is decay on the last \(K\) occupied sites. | Software fact (measured) |
| Section stays elliptic. Occupancy 256. Rails \(\varepsilon=0.02R\). Camera lock unchanged. | Software fact (already frozen) |

If the locked-eye movie still needs a caption to see the head, freeze again. Do not grow \(\varepsilon\), shrink splat, or turn LF on.

## Target

`make scan`

Does, in order:

1. Headless energy sheet (`output/scan/energy.json`)
2. `output/mp4/scan_lock.mp4` — locked three-quarter
3. `output/mp4/scan_crop.mp4` — grazing at \(\gamma(0.18)\)

No HUD. No glow. No `both.mp4` reuse.

## State the scanner owns

One write head. The rest of the record is the frozen elliptic field.

```
N_OCCUPANCY = 256
RAIL_EPS    = 0.02 * R
section     = elliptic
LF          = 0
SU          = 1          # hull + shell once
live_every  = 30         # unused while LF=0; do not advertise it

head_index  even occupancy index in {0, 2, …, 254}
Δ           = 1 even site / frame   # occupancy +2; do not interpolate
K_TAIL      = 8
persist_head = 1.0
persist_k    = δ^k                 # k = even sites behind head, 0 at head
persist_else = 0
δ            = 0.7
```

Odd sites stay dark. That is one beam. Two beams is C, which is not this target.

\[
i_t = (i_0 + 2t) \bmod 256,\qquad
\texttt{shell\_s}(i)\ \text{unchanged},\qquad
\texttt{persist}(i_t)=1,\quad
\texttt{persist}(i_t-2k)=\delta^k\ \text{for }k=1..K
\]

\(i_0 = 0\). One even lap is 128 frames. Movies are 256 frames = two laps at 30 fps ≈ 8.5s.

Do not raise \(K\) if the lock view is a uniform necklace.

## Headless acceptance

```
scan: SU=1 LF=0 PS=0 N=256 K=8 delta=0.7
head_i=0  head_s=0.000000  energy_head=1.000  energy_tail=2.199  energy_other≈0
```

Measured on the 4090: that block, `energy_other=0` every frame, even lap 128 repeats (`t=0` ≡ `t=128`). Head site is argmax persist. Odd sites dark. Section elliptic. `shell_s` unchanged. This line is now a software fact.

Checks:

- `energy_other / energy_total ≈ 0`
- `energy_head > energy_tail / K` (peak is a peak)
- head `shell_s` equals the occupancy sample at `head_index` (still on the trench)
- section bits remain elliptic for every site
- after one even lap (128 frames), `head_index` is back at \(i_0\) and the energy sheet repeats

Do not require odd/even additivity. This is not the two-field compositor test.

## Two movies

Same renderer as `make testimony`. Glow off. HUD off. Camera is a tripod.

| File | Eye | Length | Must show |
|---|---|---|---|
| `output/mp4/scan_lock.mp4` | yaw 0.55, pitch 0.42, distance 4.2 — frozen | 256 frames, 30 fps | Head travels the contact scribble. Hull does not fill. |
| `output/mp4/scan_crop.mp4` | same crop as `04_trench` at \(\gamma(0.18)\) | same frame count | Bead + tail on the curve. Air around the trench. |

Hard cuts only at loop wrap. No crossfade. No orbit.

One lap of the even necklace, field 0 sites only, \(K=8\), \(\delta=0.7\).

Lock stills: a cyan cluster travels the contact scribble; the hull does not fill; it is not a 256-bead necklace. Crop at \(\gamma(0.18)\): bead + tail on the curve when the head crosses; air around the trench. Hypothesis still sits on the two movies, not on a caption in the framebuffer.

## Must not touch

`QgaPixel` layout. `RAIL_EPS`. Occupancy table. `shell_trench.bin`. `pick`. SLM sidecar. LF. Section / lock-to-4. `tick.mp4` / `both.mp4`.

## Fail / stop

Stop and re-freeze if any of these:

- Head is only findable from `energy_head=…`
- Tail leaves \(\gamma\) or fills a cone
- A second head on odd sites to “make it read”
- `G`, fibers, or a smaller splat

Pass is narrow: lock view shows a traveling bright on the scribble; crop shows a comet on the trench. Then animation A is a software fact plus a weak hypothesis pass. Next would be named separately (B is picker-only \(\psi\); C is two heads). Not this target.

Nested shells ([NEST.md](NEST.md)) N1: packed `layer` exists; the stack did not read as three curves. Scan A still walks layer 0 only. Do not chase three heads.
