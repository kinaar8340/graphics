# Testimony — Phase 4

**Verdict (freeze):** faceplate pass, clock fail, gun deferred.

The bind held. `04_trench` is a curve of grains on γ(s). Even/odd were the same contact scribble, which is why the clocks did not read. That was occupancy, not camera. `LF=0` stays the law of `make testimony` until 01 and 02 read as complementary dashes at the locked eye, no caption.

Silent objects off Vulkan. HUD lives in `make demo` only. Captions live here.

Genre: `inner_cone` `make testimony` — looping MP4 plus stills. Copy the genre, not the rivers. This set is elliptic-only, gun off. The types are **clock states**, not flux kinds.

```
make stills       # five PNGs, 1280×720, no HUD, glow off
make testimony    # stills + 30s cinematic loop of 03_both
```

## 8-frame sheet (Software fact)

Same headless path as Phase 3, 4090:

```
static_uploads=1  live_fiber_writes=0  particle_skipped=0
even=1347.844     odd=1872.004         both=3219.883
```

`both ≈ even + odd` (`1347.844 + 1872.004 = 3219.848`). Linear compositor. Do not invent a mix curve.

`odd > even` because frame 7 is an odd write: the visual field is the last one lit. That is not a bug. Do not “fix” it.

`PS=0` is correct: persist decays every frame, so the 32-byte slot is dirty. Do not chase hash-skips.

`LF=0`: gun off. Phase 4 is the shell.

## Five stills — same eye for 00–03

Camera matrix locked for `00_hull` … `03_both`. Three-quarter, `Camera::orbit` yaw `0.55`, pitch `0.42`, distance `4.2`. If the hull moves between fields, the stills are eyewash.

| File | Camera | What must be true |
|---|---|---|
| `output/png/00_hull.png` | locked three-quarter, glow off | observer + two cones + gold torus + dim shell. Zero motes. Envelope only. |
| `output/png/01_even.png` | same eye | field 0 only. Motes on the trench, feeling-cone bias. Cyan `0.55`. |
| `output/png/02_odd.png` | same eye | field 1 only. Same trench, visual-cone bias. Same hue. |
| `output/png/03_both.png` | same eye | both fields after the 8-frame path (`both=3219.883`). Brighter trench, still on \(\gamma(s)\). Odd-hot. |
| `output/png/04_trench.png` | close, grazing | grain on the contact trench. If this is a volume cloud, the hypothesis failed. Reshoot this, do not add fibers. |

Then `output/mp4/both.mp4`: 30s orbit of the `03_both` phosphor, `C` cinematic, `live_every=30`, gun still off. That loop is the sculpture.

## Hypothesis (Model, judged by stills)

Pass:

- The glow is a curve on a body, not a soup.
- Even and odd are the same curve in different persistence states, not two objects.
- The cones and torus explain the clock. They are not decoration behind a particle emitter.

Fail:

- Motes fill the cone volumes.
- Even/odd look like two different meshes.
- Glow sits on the convex hull instead of the trench.
- Anyone reaches for `G` to make it “read.”

`G` exists in `make demo`. It does not exist in `make testimony`.

## What the stills actually show

Dense 4k cut (pre-occupancy): `even=1347.844 odd=1872.004 both=3219.883`. Occupancy 256-site cut: `even=84.241 odd=117.002 both=201.244` \(= 3219.883 \times 256/4096\). Same compositor, same odd-hot, fewer sites.

- `00_hull` is the envelope. Zero motes.
- Occupancy made beads instead of a stroke. `01_even` and `02_odd` are still the same dotted body at the locked eye — complementary in the record, not yet in the picture. `03_both` is denser, not a second mesh.
- `04_trench` aimed at \(\gamma(0.18)\): grains on the body, not a volume cloud.

Clocks still need a caption. Next stills-only move is the cheap \(\hat y\) / \(\hat z\) rails, not the gun. The 30s loop is the body, not proof of clocks.

## Occupancy (the CRT leftover)

Do not write `s_i = i/N` for every mote on both fields. Field 0 owns even sites `s = 0, 2, 4, … / N`. Field 1 owns odd sites `s = 1, 3, 5, … / N`. `03_both` is the two dashed sets on the same γ.

`N_OCCUPANCY = 256` (128+128). 4096 even/odd is occupancy in the record and one stroke in the picture. Layout, palette, camera lock, and `live_fiber_writes` stay put. No gun. No rails until 01/02 still fail as complementary dashes.

Gun-on is allowed only after 01 and 02 read as complementary dashes at yaw `0.55`, pitch `0.42`, distance `4.2`, no caption. Then the gun is a write beam along γ, not a second sculpture.
