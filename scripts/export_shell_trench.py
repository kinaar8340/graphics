#!/usr/bin/env python3
"""Offline trajectoid faceplate. Not a Rust dep.

Writes assets/shell_trench.bin:
  magic SHSC, version 1,
  n_verts, n_faces, n_trench,
  verts f32x3, faces u32x3, trench samples f32x3 (γ(s), s = i/N).

One generate_shell(..., build_3d=True). Resample the contact curve to 4096.
"""

from __future__ import annotations

import struct
import sys
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(Path.home() / "Projects" / "flux_trajectoid" / "src"))

from flux_trajectoid import generate_shell  # noqa: E402

MAGIC = b"SHSC"
VERSION = 1
N_TRENCH = 4096
OUT = ROOT / "assets" / "shell_trench.bin"


def resample_closed(path: np.ndarray, n: int) -> np.ndarray:
    p = np.asarray(path, dtype=np.float64)
    if len(p) < 2:
        raise SystemExit("contact curve too short")
    if not np.allclose(p[0], p[-1], atol=1e-5):
        p = np.vstack([p, p[0]])
    seg = np.linalg.norm(np.diff(p, axis=0), axis=1)
    cum = np.concatenate([[0.0], np.cumsum(seg)])
    total = float(cum[-1])
    if total < 1e-9:
        raise SystemExit("zero-length trench")
    s = np.linspace(0.0, total, n, endpoint=False)
    out = np.empty((n, 3), dtype=np.float32)
    for i, si in enumerate(s):
        j = int(np.searchsorted(cum, si, side="right") - 1)
        j = max(0, min(j, len(seg) - 1))
        t = 0.0 if seg[j] < 1e-12 else (si - cum[j]) / seg[j]
        out[i] = ((1.0 - t) * p[j] + t * p[j + 1]).astype(np.float32)
    return out


def main() -> None:
    shell = generate_shell(
        "phosphor loom",
        seed=42,
        build_3d=True,
        n_lat=24,
        n_lon=48,
        n_points=256,
        use_tpt=True,
        trench_depth=0.08,
    )
    verts = np.asarray(shell.mesh_vertices, dtype=np.float32)
    faces = np.asarray(shell.mesh_faces, dtype=np.uint32)
    trench = resample_closed(shell.path_on_body, N_TRENCH)
    OUT.parent.mkdir(parents=True, exist_ok=True)
    with OUT.open("wb") as f:
        f.write(MAGIC)
        f.write(struct.pack("<I", VERSION))
        f.write(struct.pack("<III", len(verts), len(faces), len(trench)))
        f.write(verts.tobytes(order="C"))
        f.write(faces.tobytes(order="C"))
        f.write(trench.tobytes(order="C"))
    print(
        f"wrote {OUT} verts={len(verts)} faces={len(faces)} trench={len(trench)} "
        f"is_3d={shell.is_3d}"
    )


if __name__ == "__main__":
    main()
