"""vision_tracker geometry. No webcam. No draw. Software fact of S² sample."""

from __future__ import annotations

import math
import sys
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from vision_tracker import (  # noqa: E402
    ALPHA_MIN,
    E_DEF,
    I_DEF,
    N_OCCUPANCY,
    T_OFF,
    TAU,
    AttentionWell,
    camera_center,
    default_site,
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


def test_i_def_is_even_nearest_plus_z():
    path = ROOT / "assets" / "shell_trench.bin"
    _verts, samples = read_shsc(path)
    i = default_site(samples)
    assert i == I_DEF
    assert i % 2 == 0
    assert i == 40


def test_unmatched_does_not_slam():
    well = AttentionWell()
    off = np.array([0.2, 0.0, 1.0])
    g = well.step(off, 1.0 / 30)
    assert g["locked"] is True
    assert g["attention_state"] is True
    held = np.array(well.E_hat)
    g2 = well.step(None, 0.5)
    assert g2["locked"] is False
    assert g2["attention_state"] is True
    assert np.allclose(well.E_hat, held)
    assert g2["persist"] == 0.0
    assert g2["layer"] == 0


def test_presence_in_dead_zone_keeps_attention():
    well = AttentionWell()
    well.step(E_DEF, 1.0 / 30)
    wobble = np.array([np.sin(ALPHA_MIN * 0.3), 0.0, np.cos(ALPHA_MIN * 0.3)])
    g = well.step(wobble, 1.0 / 30)
    assert g["locked"] is True
    assert g["attention_state"] is True
    assert g["drift"] is False


def test_timeout_then_leak_to_well():
    well = AttentionWell()
    meas = np.array([0.6, 0.0, 0.8])
    well.step(meas, 1.0 / 30)
    dt = 1.0 / 30
    steps = int(T_OFF / dt) + 5
    last = None
    for _ in range(steps):
        last = well.step(None, dt)
    assert last["locked"] is False
    assert last["attention_state"] is False
    leak_n = int(3 * TAU / dt)
    for _ in range(leak_n):
        last = well.step(None, dt)
    assert last["drift"] is True
    assert last["site"] == I_DEF
    assert last["alpha_from_default"] <= ALPHA_MIN
    assert np.allclose(well.E_hat, E_DEF, atol=1e-6)
    assert last["site"] % 2 == 0
