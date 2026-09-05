# Nested faceplates — N0 paper only

The glass onion is the mood board for a dielectric. The law is still the right-hand frame: one shell, wire observer, two cones, a yellow separator, a few cyan beads on a curve.

Nesting is a new machine, not a thicker coat of paint on \(\gamma(s)\). Noun already in the ecosystem: **RubikConeConduit / ShellCube / 216-cube RingConeChain** ([pic](https://github.com/kinaar8340/pic)). Do not invent a fifth metaphor. 216-cube / \(6^3\) is a later discrete map. Do not start there.

**Until the hypothesis is tested the same way rails were tested, \(L=1\).** Scan A stays the only animation on the table. Nested shells are paper for where that head is allowed to step next (\(\ell\) as well as \(s\)).

## Claim (not tested)

| Sentence | Label |
|---|---|
| Three confocal trenches with \(\Delta R\) greater than splat radius read as a radial graphic, not as one thicker necklace. | Hypothesis |
| Packed spare bits can hold `layer` without a 33rd byte. | Model (not packed) |
| Splat radius at the locked eye, head persist=1, is \(0.0315R\). `RAIL_EPS=0.02R` is already smaller. | Software fact |

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
| Candidate \(\Delta R\) (N1, not claimed) | \(0.08R\) | Must exceed splat. Not tested at \(4.2\). |

If splat \(> \Delta R\), layers collapse at sculpture distance. You already know that collapse. Do not shrink splat to flatter the Model.

## Do not grow a 33rd byte

The store is still 32 bytes. Packed still has spare bits (`field:1 | section:2`).

\[
\texttt{packed} \supset \texttt{field}\ (1),\ \texttt{section}\ (2),\ \texttt{layer}\ (8)
\]

\(256\) layers max if you want a byte. First experiment is not \(256\). First experiment is \(3\) or \(6\) — a Rubik axis, not a volume texture. **Not packed. `layer` bits stay unused while \(L=1\).**

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
