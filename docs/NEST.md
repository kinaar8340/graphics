# Nested faceplates

The glass onion is the mood board for a dielectric. Nesting is a new machine, not a thicker coat of paint on \(\gamma(s)\). Noun: **RubikConeConduit / ShellCube / 216-cube RingConeChain**. Do not invent a fifth metaphor. 216-cube is later.

Scan A still walks layer 0 only. Do not chase three heads.

## N1 — ledger pass, picture fail

\(L=3\). Same \(\gamma\), three radii. \(\Delta R=0.08R\).

| Sentence | Label |
|---|---|
| Three confocal trenches with \(\Delta R=0.08R>0.0315R\) (splat at `4.2`, persist\(=1\)) read as a radial graphic, not as one thicker necklace. | **Hypothesis, tested: not visible at sculpture distance.** `n1_all` is a thicker necklace. `n1_crop` is one ribbon. |
| \(\texttt{pos}=\gamma(s)+\ell\,\Delta R\,\hat r\), \(\ell\in\{0,1,2\}\). Same trench table. `shell_s` not folded. | **Model** |
| `packed` holds `field:1 \| section:2 \| layer:8`. No 33rd byte. | **Software fact** |
| Splat \(0.0315R\), `RAIL_EPS=0.02R` already collapsed, \(\Delta R\) must beat splat. | **Software fact** (measured) |
| `sep_01=0.080` `sep_12=0.080` `energy_L0=L1=L2=256` `SU=1` `LF=0` | **Software fact** (measured) |

Freeze again. Do not lower \(\Delta R\). Do not shrink splat. Do not add shells 3…255. \(\Delta R=0.08R\) was the candidate; it lost as a picture. Isolated `n1_L0` / `n1_L1` / `n1_L2` sit at different radii (the model is in the record). Composed lock and crop do not read as three curves with air.

```
make nest-headless   # energy + separation
make nest-stills     # locked eye, same camera as 00–03
```

Geometry: one `assets/shell_trench.bin`. \(\mathbf{p}_{i,\ell}=\gamma(s_i)+\ell\Delta R\,\hat r(s_i)\). Rails stay \(\varepsilon=0.02R\) in \(\hat y/\hat z\). Do not use PIC \(\sqrt{3}-1\).

Export / pick: default `layer=0`. Persist on dump stays `0`.

**Picture:** fail. Crop is one ribbon. Lock is one thicker necklace. Isolated layers prove the radial offset; the stack does not read. Scan A still walks layer 0 only.

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
| Candidate \(\Delta R\) (N1) | \(0.08R\) | Must exceed splat. Ledger tested; picture pending. |

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

If every layer writes at once you are back to a particle bed in radius. Animation A extends cleanly: the head is \((i_t, \ell_\star)\). Other layers stay dark or at persist \(0\).

## Configurations (if unfrozen later)

| Rung | What | Not |
|---|---|---|
| **N0** | This page. \(L=3\) on paper. \(\Delta R\) printed next to splat. No new mesh. | Tonight’s instances |
| **N1** | Three offline shells. Same sidecar as `export-shell`. Three `trench_sha256`s. Faceplate binds layer from packed. `make scan` walks layer \(0\) only. Locked eye + crop. | 20 shells |
| **N2** | Glass hull. One transparent envelope. Interior layers are trenches, not solid onions. `LF=0`. | Cubies |
| **N3** | SLM stack. \(L\) biases or \(L\) maps. HITL still not v1. | Prettier `preview_montage` |

**Not N:** Rubik cubies as instanced cubes, null-gap metamaterial shader, 216-cell first pass, gun on to “fill the volume.”

Unfreeze is one line: **N1**, new claim label first. Until then \(L=1\) and Scan A is the only animation.
