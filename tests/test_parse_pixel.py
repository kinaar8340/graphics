"""Parse the frozen 32-byte dump without VQC. Software fact of layout."""

from __future__ import annotations

import struct
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from export_slm_pixel import parse_pixel  # noqa: E402


def _ensure_blob() -> Path:
    blob = ROOT / "output" / "pick" / "qga_pixel.bin"
    if blob.is_file() and blob.stat().st_size == 32:
        return blob
    subprocess.check_call(
        ["cargo", "run", "--release", "--bin", "pick", "--", "--dump"],
        cwd=ROOT,
    )
    return blob


def test_parse_matches_pick_json():
    blob = _ensure_blob()
    raw = blob.read_bytes()
    fields = parse_pixel(raw)
    assert fields["n_bytes"] == 32
    assert fields["persist"] == 0.0
    assert fields["section"] in (
        "elliptic",
        "parabolic",
        "hyperbolic",
        "flat-pockets",
    )
    assert fields["field"] in (0, 1)
    theta, phi, psi, offset, amp, shell_s, persist, packed = struct.unpack("<7fI", raw)
    assert packed == fields["packed"]
    assert abs(theta - fields["theta"]) < 1e-6
    assert abs(shell_s - fields["shell_s"]) < 1e-6
    assert persist == 0.0
    assert amp == 1.0 or amp > 0.0
