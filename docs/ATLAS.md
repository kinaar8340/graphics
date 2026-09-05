# Atlas — four tunings of one machine

The inverse-Hopf loom clip is already panel 1 of QGA Fig 6.1, with the knobs pinned. Cloning that machine means **four isolated presets**, not a second sculpture that “is” the figure, and not four types composited on \(\gamma\).

Office-reply warning: two of the panels *look* like the same picture until you read the separators. That is clocks, rails, and N1 again — composition hides type.

## What the two posts are

| Post | Object |
|---|---|
| [209575474…](https://x.com/kinaar8340/status/2095754742886695171) | Special case: inverse-Hopf loom, **elliptic**, \(\lambda=0.15\), two clocks, `SU=1`, `LF` live on the 4090 (`static_uploads=1 · live_fiber_writes=10 · skipped=291 · fallbacks=0`) |
| [209351556…](https://x.com/kinaar8340/status/2093515561049493959) | QGA Fig 6.1: **four flux topographs** — elliptic compact, hyperbolic periodic, parabolic low-structure, 0-hyperbolic pockets |

The meme under Fig 6.1 is the claim already proved on the faceplate: without the separator story, parabolic and flat-ish collapse to “the same picture.”

## Special case → clone

The loom is one point in a four-point family. The family is already in `qga_pixel.section` (2 bits) and in `inner_cone::FluxKind`. Cloning means **changing those bits and the hold parameter**, not instancing four crates.

| Panel (Fig 6.1) | `section` | Mechanism | Loom / phosphor knobs | Prose |
|---|---|---|---|---|
| Elliptic — finite compact separators | `0` | bound, no river, \(z>0\) | \(\lambda=0.15\), `LF` only in `qga_gpu`, faceplate `LF=0` | “compact cloud” |
| Hyperbolic — periodic separators | `2` | river, Zener, pierce | do **not** drop this on \(\gamma\) as elliptic | “river / even nappe” |
| Parabolic — transitional | `1` | \(\Delta=0\) locus, \(\beta=\alpha\) | lock-to-4 must flip class at the cut | “on the generatrix” |
| 0-hyperbolic — flat pockets | `3` | near-constant pockets | magenta witness only | “not a fifth palette” |

Same record, same 32 bytes, same two clocks as *ledger*. Different **type**. That is the fractal: structure (topograph), mechanism (`FluxKind` + \(\lambda\) + Zener), prose (claim label) are the same sentence at four scales of the form.

It is **not** a fractal renderer. It is self-similarity of the *cut*:

\[
\text{plane }\cap\text{ double cone}
\quad\text{at four discriminants}
\quad\text{= four topographs}
\quad\text{= four }\texttt{section}\text{ values.}
\]

The whole is not the sum of four GPU windows. The whole is the classification. Summing the panels in one framebuffer is the meme.

## Where to clone (and where not)

**Clone in `qga_gpu` / `inner_cone`**, the machine that filmed the loom. Four presets. One binary, four `--section` values. That *replicates* Fig 6.1 as four runs, **isolated**, like the four boxes on the page.

**Do not clone into `shellscan` testimony.** The faceplate is elliptic by law. Hyperbolic on \(\gamma\) is a pierce. Four types composited on one trench is N1 with extra mythology. `pick` already writes `section`; it does not need four hemispheres.

**Prose clone** is one labeled sentence per preset (Model: this `FluxKind`; Software fact: these counters; Hypothesis: only if you claim the *eye* will see the separator). Fig 6.1’s captions *are* that prose. Copy them. Do not invent a fifth.

## Holism

Holism here is the **Hopf address + conic type + two clocks**, reused. It is not “put ocean + loom + Fig 6.1 + pine + ball in one present.”

```
manuscript Fig 6.1     prose / structure
inner_cone FluxKind    mechanism
qga_pixel.section      store
qga_gpu loom clip      one tuned cell (elliptic, λ=0.15)
shellscan faceplate    elliptic phosphor only, LF=0
SLM --mask             sign on the same store
```

Each layer is the same four-word sentence. That *is* the fractal. Adding geometry so the whole “looks like the sum” is how parabolic and flat become Pam’s two printouts.

## If unfrozen later

One line, in **`qga_gpu` not `shellscan`**:

> **Software fact.** Four loom presets (`--section elliptic|parabolic|hyperbolic|flat`) reproduce Fig 6.1 as four isolated runs. No composition window.

Fail bar: hyperbolic run must *not* satisfy `elliptic_stays_in_visual_hemisphere`. If it does, you did not clone the machine — you retuned \(\lambda\) and kept the type name.

Until that line exists, the posted loom stays the elliptic special case, Fig 6.1 stays the atlas, and Phosphor Loom stays one cell of that atlas with the gun off.
