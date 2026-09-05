"""Sign mask on the SLM seed. Loopback stays on the unmasked blob."""

from __future__ import annotations

import math
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from export_slm_pixel import PI, apply_sign_mask, pack_pixel, parse_pixel  # noqa: E402


def _elliptic() -> dict:
    return parse_pixel(
        pack_pixel(
            {
                "theta": 0.3,
                "phi": 0.0,
                "psi": 0.0,
                "offset": 0.4,
                "amplitude": 1.0,
                "shell_s": 0.5,
                "persist": 0.0,
                "field": 1,
                "section": "elliptic",
                "layer": 0,
            }
        )
    )


def test_antipode_psi_pi_same_class():
    fields = _elliptic()
    seed, meta = apply_sign_mask(fields, "antipode")
    assert seed["section"] == "elliptic"
    assert seed["field"] == 1
    assert seed["layer"] == 0
    assert seed["amplitude"] == fields["amplitude"]
    assert abs((seed["psi"] - fields["psi"] - PI) % (2 * PI)) < 1e-6
    assert meta["psi_delta"] == PI
    assert meta["nappe"] == "keep"
    assert pack_pixel(fields) != pack_pixel(seed)


def test_blank_amp_zero():
    fields = _elliptic()
    seed_b, meta_b = apply_sign_mask(fields, "blank")
    assert seed_b["amplitude"] == 0.0
    assert seed_b["section"] == "elliptic"
    assert seed_b["psi"] == fields["psi"]
    assert meta_b["blank"] is True


def test_nappe_omit_elliptic_no_section_flip():
    fields = _elliptic()
    seed_n, meta_n = apply_sign_mask(fields, "nappe")
    assert meta_n["nappe"] == "omit"
    assert seed_n["section"] == "elliptic"
    assert seed_n["field"] == fields["field"]
    assert pack_pixel(fields) == pack_pixel(seed_n)


def test_none_is_identity_seed():
    fields = _elliptic()
    seed, meta = apply_sign_mask(fields, "none")
    assert meta["mask"] == "none"
    assert meta["psi_delta"] == 0.0
    assert meta["blank"] is False
    assert meta["nappe"] == "keep"
    assert pack_pixel(fields) == pack_pixel(seed)
    assert parse_pixel(pack_pixel(seed))["n_bytes"] == 32


def test_roundtrip_size():
    fields = _elliptic()
    raw = pack_pixel(fields)
    assert len(raw) == 32
    q = parse_pixel(raw)
    assert q["section"] == "elliptic"
    assert q["field"] == 1
    assert math.isclose(q["shell_s"], 0.5)
