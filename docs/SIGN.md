# Sign — antipode vs nappe

The red cells are not a broken phosphor. They are sign. On this machine sign is either a \(\pi\) shift on the same Hopf fiber or a write into the orthogonal cone. Those two must not be mixed. The mask is different for each.

**Wired (software fact).** `--mask {none,antipode,blank,nappe}` on the SLM seed only. Loopback stays on the unmasked 32 bytes. Faceplate does not draw this. Scan A does not consume `site`. `LF=0`.

## Claim

| Sentence | Label |
|---|---|
| \(i^2=j^2=k^2=ijk=-1\). Odd permutations \(ik=-j\), \(ji=-k\), \(kj=-i\). | Theorem (cited) |
| \(h(-q)=h(q)\). Same \((\theta,\phi)\), \(\psi\mapsto\psi+\pi\). Intensity witness cannot see it. Phase can. | Theorem / Model |
| Imaginary-axis flip (\(i\mapsto -i\)) is the other nappe: feeling \(+\hat y\), even field, hue \(0.30\). | Model (already frozen) |
| Witness RGB treating \(-q\) as the same cyan bead while the SLM still carries \(\pi\) looks like a dirty cone. | Software fact of the double cover, not a need for more shells |
| `export_slm_pixel.py --mask {none,antipode,blank,nappe}` writes a sign flag on the SLM stack. `none` is the elliptic-plain package. Antipode is \(\psi+\pi\) on the seed only. Blank is `amplitude=0`. Nappe is omit-from-elliptic (no section flip). `rgb_preview` of the dumped blob does not change. | **Software fact** |

## What the red cells are

The table is bilinear products in \(\mathbb{H}\cong\mathbb{R}^4\). Red means the product landed with a minus: \(-1,-i,-j,-k\).

| Algebra | Geometry | Display |
|---|---|---|
| \(q\mapsto -q\) | Antipode on \(S^3\). Hopf: \(h(-q)=h(q)\). Same \((\theta,\phi)\), \(\psi\mapsto\psi+\pi\) | Same site on \(\gamma\). Intensity witness cannot see it. Phase can. |
| Imaginary axis flipped, e.g. \(i\mapsto -i\) | Opposite nappe / orthogonal cone (feeling \(+\hat y\) vs visual \(+\hat z\)) | Even field, orange class, separator blanking |

The crystal-ball “issue” is almost certainly the first one leaking into the second. That is a double-cover collapse, not a license for more shells. N1 already failed as a picture.

## Phase mask for the antipode (\(q\sim -q\))

Same fiber. Same elliptic site. Do not flip section or layer. persist stays 0 on export. `pick` already stores \(\psi\); the mask is a function of the product, not a 33rd byte.

\[
\psi_{\text{mask}}=\psi+\pi \bmod 2\pi
\quad\text{when the product is in a red cell.}
\]

On the SLM (already a phase device):

\[
\phi_{\text{SLM}}(x)\ \leftarrow\ \phi_{\text{SLM}}(x)+\pi
\quad\text{on that slice.}
\]

\(+\pi\) is a minus in the complex field: \(e^{i\pi}=-1\). That is the red cell, written as light.

Hard mask (refuse the antipode): separator blanking — amplitude \(0\), same as the gold torus. Use that only if the sign is **error** (unmatched `AttentionWell` sample), not if it is a legal odd permutation you still want as OAM.

**Rule:** legal red product, same \(h(q)\): \(\pi\) phase. Unmatched / “issue” you do not want in the ball: blank (amp \(0\)), do not steer it into the feeling cone.

## Phase mask for the orthogonal cone (the other red)

\(-i,-j,-k\) as axes are not \(\psi+\pi\). They are a different point of \(S^2\). Dual-cone model already named that object: feeling cone, even field, hue \(0.30\).

Odd field writes visual \(+\hat z\). Even field writes feeling \(+\hat y\). Separator torus is blanking. Elliptic forbids pierce.

Do not invent a second orthogonal cone in Vulkan. The orthogonal nappe is a clock bit + section, not a nested shell.

If the signal (SLM stack) is mixing nappes, mask in **mode index**, not in RGB. v1 was elliptic-only — keep that. A phase-only panel can null a cone by giving that mode a flat \(0\) amplitude in the complex field before `angle()`. Soft aperture in OAM space, not opacity-\(0\) windows.

## Well and mask are different

Lost attention leaks to \(\hat E_{\mathrm{def}}=(0,0,1)\), site \(40\). It does not walk into the feeling cone. Unmatched stays **drop**, not a sign flip. Do not dump `AttentionWell` trunk into \(\psi+\pi\) without a label.

## What not to do

- Paint red cells magenta and call it a fifth palette
- Raise \(\Delta R\) so “the negative cone appears”
- Turn `LF` on to draw \(-i,-j,-k\) as extra fibers
- Calibrate `vision_tracker` on the red beads
- Let unmatched blobs become \(\psi+\pi\)
- Mix the two red readings on one mask

## Wired

```
make slm-export MASK=none       # elliptic-plain; sign.mask=none
make slm-mask                   # antipode seed; loopback still unmasked
make slm-loopback               # original 32 bytes, BER 0
```

`qga_pixel.bin` is the unmasked dump. `qga_pixel_seed.bin` is the masked seed. Manifest carries `"sign": {...}`.

```
red product on same Hopf base → ψ += π  (or SLM slice += π)
red product as other nappe     → field bit / omit from elliptic package
unmatched / issue              → amp = 0  (blanking)
```

The ball stays a pipeline: record carries \(\psi\), package carries phase, glass is optics. The reddish cells are a \(\pi\) on the fiber or a blank, not a third cone.
