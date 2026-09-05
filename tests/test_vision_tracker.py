"""vision_tracker geometry. No webcam. No draw. Software fact of S² sample."""

from __future__ import annotations

import math
import sys
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from vision_tracker import (  # noqa: E402
    N_OCCUPANCY,
    camera_center,
    gamma,
    load_trench_xyz,
    nearest_site,
    occupancy_even_xyz,
    read_shsc,
    s2_from_E,
)


def test_s2_from_unit_z():
    s = s2_from_E(np.array([0.0, 0.0, 2.0]))
    assert abs(s["theta"]) < 1e-9
    assert abs(s["radius"] - 2.0) < 1e-9


def test_s2_from_y():
    s = s2_from_E(np.array([0.0, 1.0, 0.0]))
    assert abs(s["theta"] - math.pi / 2) < 1e-6
    assert abs(s["phi"] - math.pi / 2) < 1e-6


def test_camera_center_roundtrip():
    R = np.eye(3)
    t = np.array([0.05, 0.0, -1.8])
    C = camera_center(R, t)
    assert np.allclose(C, -t)


def test_occupancy_even_sites_on_trench():
    path = ROOT / "assets" / "shell_trench.bin"
    verts, samples = read_shsc(path)
    assert verts.shape[1] == 3
    assert samples.shape[0] >= 64
    even, idx = occupancy_even_xyz(samples)
    assert even.shape[0] == N_OCCUPANCY // 2
    assert list(idx) == list(range(0, N_OCCUPANCY, 2))
    xyz = load_trench_xyz(path)
    assert xyz.shape == samples.shape
    g0 = gamma(samples, 0.0)
    assert np.linalg.norm(g0 - samples[0]) < 1e-6


def test_nearest_site_is_even_occupancy():
    path = ROOT / "assets" / "shell_trench.bin"
    _verts, samples = read_shsc(path)
    even, idx = occupancy_even_xyz(samples)
    E = even[3]
    local = nearest_site(E / np.linalg.norm(E), even)
    assert idx[local] % 2 == 0
    assert local == 3
