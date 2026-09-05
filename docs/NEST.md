# Nested faceplates

**Verdict (N1 freeze):** ledger pass, picture fail. Tip `95175f3`.

The glass onion is the mood board for a dielectric. Nesting is a new machine, not a thicker coat of paint on \(\gamma(s)\). Noun: **RubikConeConduit / ShellCube / 216-cube RingConeChain**. Do not invent a fifth metaphor.

Sculpture distance cannot carry nested layers as a picture. Address lives in `packed` and in `sep_*`. Depth does not live at sculpture distance. Isolated layers are a software fact. A radial graphic is not.

Packed may keep `layer:8` — the bits are true — they are not a draw license.

Scan A still has a legal stage: one head on layer 0, other layers dark. That is the only motion that does not depend on the eye resolving two gaps at once. Do not chase three heads. Nothing else unfreezes.

## N1 — tested, not visible in composition

\(L=3\). Same \(\gamma\), three radii. \(\Delta R=0.08R\).

| Sentence | Label |
|---|---|
| Three confocal trenches with \(\Delta R=0.08R>0.0315R\) (splat at `4.2`, persist\(=1\)) read as a radial graphic, not as one thicker necklace. | **Hypothesis, tested: not visible in composition.** `n1_all` is a thicker necklace. `n1_crop` is one ribbon. Isolated `n1_L0`/`n1_L1`/`n1_L2` show the model. |
| \(\texttt{pos}=\gamma(s)+\ell\,\Delta R\,\hat r\), \(\ell\in\{0,1,2\}\). Same trench table. `shell_s` not folded. | **Model** |
| `packed` holds `field:1 \| section:2 \| layer:8`. No 33rd byte. | **Software fact** |
| Splat \(0.0315R\), `RAIL_EPS=0.02R` already collapsed, \(\Delta R\) must beat splat. | **Software fact** (measured) |
| `sep_01=0.080` `sep_12=0.080` `energy_L0=L1=L2=256` `SU=1` `LF=0`. Compositor linear (\(256+256+256\)). | **Software fact** (measured) |

The sheet is internally consistent. Composition is what the hypothesis asked about, and composition is one ribbon.

That is the same collapse as rails, now in radius:

| Attempt | Designed gap | Splat at \(4.2\) | Picture |
|---|---|---|---|
| Rails | \(0.02R\) | \(0.0315R\) | one necklace |
| Clocks / occupancy | site parity | same splat | one necklace |
| N1 layers | \(0.08R\) | \(0.0315R\) | one thicker necklace |

Do not raise \(\Delta R\) until the ribbon “splits.” That is growing \(\varepsilon\) again. Do not shrink splat. Do not instance more shells. Do not spend another commit proving the same collapse.

```
make nest-headless   # energy + separation
make nest-stills     # locked eye, same camera as 00–03
```

Geometry: one `assets/shell_trench.bin`. \(\mathbf{p}_{i,\ell}=\gamma(s_i)+\ell\Delta R\,\hat r(s_i)\). Rails stay \(\varepsilon=0.02R\) in \(\hat y/\hat z\). Do not use PIC \(\sqrt{3}-1\).

Export / pick: default `layer=0`. Persist on dump stays `0`.

**Picture:** fail. Crop is one ribbon. Lock is one thicker necklace. Isolated layers prove the radial offset; the stack does not read. Scan A still walks layer 0 only. Packed `layer:8` stays. Not a draw license.

## N0 paper

Until N1, \(L=1\) was the freeze. That paper stands. First experiment is \(3\), not \(256\).

## Claim (N0, superseded as packing)

## Gap

If the gaps are optically null, the stack is one metamaterial, not \(N\) independent screens. Light sees a stratified index, not \(N\) phosphors.

| Gap | What you get |
|---|---|
| Large | Separate sculptures. Mashup. |
| Finite, designed | A radial lattice. Addressable layers. SLM/bias can cite layer. |
| Almost null | Interference / effective medium. Layers stop being pixels and become a crystal. |

A 3D `qga_pixel` graphic wants the **middle** row: gaps small enough to read as one body, large enough that layer index is a real coordinate. “Almost null” belongs to Phase 5 optics (`include_shell_bias` already exists), not to the Vulkan faceplate.

PIC ShellCube is inscribed \(r=1\), circumscribed \(R=\sqrt{3}\). That differential \(\sqrt{3}-1 \approx 0.732\) is identity topology in the conduit, **not** a Vulkan \(\Delta R\). Do not drop \(\sqrt{3}\) into `bind_shell`.

## Splat vs \(\Delta R\) (print, no mesh)

Witness splat, `particle.wgsl`, locked eye distance \(4.2\), aperture \(1\), head mass \(1\):

\[
\texttt{size} = \mathrm{mix}(0.018,0.072,0.5)\cdot\mathrm{clamp}(4.2/16,0.70,1.70) = 0.045\cdot 0.70 = 0.0315
\]

| Number | Value | Status |
|---|---|---|
| Splat radius at \(4.2\), persist=1 | \(0.0315R\) | Software fact |
| `RAIL_EPS` | \(0.02R\) | Already \(<\) splat. Rails collapsed. |
| \(\Delta R\) (N1) | \(0.08R\) | Exceeds splat in the ledger. Not visible in composition. Do not raise. |

If splat \(> \Delta R\), layers collapse at sculpture distance. You already know that collapse. Do not shrink splat to flatter the Model.

## Do not grow a 33rd byte

The store is still 32 bytes. Packed still has spare bits (`field:1 | section:2`).

\[
\texttt{packed} \supset \texttt{field}\ (1),\ \texttt{section}\ (2),\ \texttt{layer}\ (8)
\]

\(256\) layers max if you want a byte. First experiment is not \(256\). First experiment is \(3\) — a Rubik axis, not a volume texture. **N1 packed `layer:8`. \(L=3\).**

`shell_s` stays the parameter on that layer’s \(\gamma_\ell(s)\). Do not fold \(\ell\) into \(s\).

\[
\texttt{pos} = \gamma_\ell(\texttt{shell\_s}),\qquad \ell \in \{0,\ldots,L-1\}.
\]

Each layer needs its own offline trench table, or one mesh with a radial offset of \(\Delta R\) per layer. \(\Delta R\) is the gap.

## Rubik, made precise

A cube graphic is a **layer permutation**, not a mesh of cubies.

- One move: increment `shell_s` on all pixels with a given \(\ell\), or increment \(\ell\) on a longitude band.
- 3-axis version: trench phase \(s\), layer \(\ell\), and an azimuthal tile index if you panel each shell (that tile index is the only new noun; it can wait).
- 216-cube / RingConeChain is later. Not N0.

Elliptic constraint still applies per layer: compact, no pierce, cyan \(0.55\) unless section changes. A nested stack does not license a river.

## Four layers

| Layer | Nested version |
|---|---|
| CRT ritual | Scan head runs on one \(\ell\) per frame, or a chosen subset. Not all layers hot. |
| Inverse-Hopf write | Still \((\theta,\phi,\psi)\) in the record. One fiber in pick. Do not loom every layer. |
| Conic algebra | Plane cut still sets section. Shared by all layers unless you explicitly bind a plane per shell (do not, v1). |
| Screen | \(L\) confocal \(\gamma_\ell\), gap \(\Delta R\), glass hull is witness geometry only. |

If every layer writes at once you are back to a particle bed in radius. N1 composed that way. It is one ribbon. Animation A’s legal stage: the head is \((i_t, \ell=0)\). Other layers stay dark.

## Configurations

| Rung | State |
|---|---|
| **N0** | Paper. Closed into N1. |
| **N1** | **Closed.** Radial copies of one trench. Ledger pass. Picture fail. Tip `95175f3`. |
| **N2** | Glass hull. Not unfrozen. |
| **N3** | SLM stack. Not unfrozen. HITL still not v1. |

**Not N:** Rubik cubies as instanced cubes, null-gap metamaterial shader, 216-cell first pass, gun on to “fill the volume,” raising \(\Delta R\) until the ribbon splits.

Nothing else unfreezes. Scan A is the only animation.
