#!/usr/bin/env python3
"""Compose pick --dump with vqc_demo loopback / flux_trajectoid SLM.

No Rust path-dep. No VQC path-dep on qga_gpu. Not a projector. Not a gun.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import struct
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
HOME = Path.home()
BLOB = ROOT / "output" / "pick" / "qga_pixel.bin"
TRENCH = ROOT / "assets" / "shell_trench.bin"
SECTION = ("elliptic", "parabolic", "hyperbolic", "flat-pockets")


def parse_pixel(raw: bytes) -> dict:
    if len(raw) != 32:
        raise SystemExit(f"expected 32-byte QgaPixel, got {len(raw)}")
    theta, phi, psi, offset, amplitude, shell_s, persist, packed = struct.unpack(
        "<7fI", raw
    )
    field = packed & 1
    section = SECTION[(packed >> 1) & 3]
    layer = (packed >> 3) & 0xFF
    return {
        "theta": theta,
        "phi": phi,
        "psi": psi,
        "offset": offset,
        "amplitude": amplitude,
        "shell_s": shell_s,
        "persist": persist,
        "field": field,
        "section": section,
        "layer": layer,
        "packed": packed,
        "n_bytes": 32,
    }


def trench_identity(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def loopback(raw: bytes, fields: dict) -> dict:
    sys.path.insert(0, str(HOME / "Projects" / "vqc_demo" / "src"))
    from vqc_demo.pipeline import loopback as vqc_loopback
    from vqc_demo.projector import TEST_PROFILE

    result = vqc_loopback(raw, profile=TEST_PROFILE)
    recovered = result.payload
    n = max(len(raw), 1)
    n_err = sum(a != b for a, b in zip(raw, recovered)) + abs(len(raw) - len(recovered))
    ber = n_err / n
    rec_fields = parse_pixel(recovered) if len(recovered) == 32 else {}
    ok = (
        result.crc_ok
        and recovered == raw
        and rec_fields.get("section") == fields["section"]
        and rec_fields.get("field") == fields["field"]
        and rec_fields.get("n_bytes") == 32
        and ber == 0.0
    )
    return {
        "crc_ok": bool(result.crc_ok),
        "ber": ber,
        "n_err": n_err,
        "n_bytes": len(recovered),
        "section": rec_fields.get("section"),
        "field": rec_fields.get("field"),
        "match": ok,
    }


def export_slm(raw: bytes, fields: dict, out: Path, preset: str) -> dict:
    sys.path.insert(0, str(HOME / "Projects" / "flux_trajectoid" / "src"))
    from flux_trajectoid import PhotonSeedAsteroid

    out.mkdir(parents=True, exist_ok=True)
    ast = PhotonSeedAsteroid(raw, seed=42).build(
        force_stub_flux=True,
        build_3d=True,
        n_shards=4,
        n_coupling_steps=4,
        lattice_nx=8,
    )
    pkg = ast.export_slm(
        str(out),
        preset=preset,
        include_shell_bias=True,
        stack_shards=True,
        use_gs=False,
    )
    ident = trench_identity(TRENCH) if TRENCH.is_file() else None
    extra = {
        "qga_pixel": {
            "shell_s": fields["shell_s"],
            "section": fields["section"],
            "field": fields["field"],
            "persist": fields["persist"],
            "trench_bin": str(TRENCH.relative_to(ROOT)) if TRENCH.is_file() else None,
            "trench_sha256": ident,
            "lock_to_4": True,
        },
        "claim": "loadability of a generic_512 phase package; not far-field beauty",
        "not": [
            "projector MP4 on an SLM",
            "occupancy 256 / RAIL_EPS as hologram knobs",
            "rgb_preview in the phase map",
            "inverse-Hopf live fibers (the crate gun)",
        ],
    }
    man_path = out / "manifest.json"
    man = {}
    if man_path.is_file():
        man = json.loads(man_path.read_text())
    man.update(extra)
    man_path.write_text(json.dumps(man, indent=2) + "\n")
    (out / "qga_pixel.bin").write_bytes(raw)
    (out / "qga_pixel.json").write_text(json.dumps(fields, indent=2) + "\n")
    files = list(getattr(pkg, "files", []) or [])
    return {
        "out_dir": str(out),
        "preset": preset,
        "include_shell_bias": True,
        "shell_s": fields["shell_s"],
        "trench_sha256": ident,
        "files": files,
        "manifest": str(man_path),
    }


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--blob", type=Path, default=BLOB)
    p.add_argument("--out", type=Path, default=ROOT / "output" / "slm" / "generic_512")
    p.add_argument("--preset", default="generic_512")
    p.add_argument(
        "--loopback-only",
        action="store_true",
        help="rung 1: software loopback, no SLM package",
    )
    p.add_argument(
        "--skip-loopback",
        action="store_true",
        help="rung 2 only (not the default)",
    )
    args = p.parse_args()
    if not args.blob.is_file():
        raise SystemExit(f"missing {args.blob} — run: cargo run --release --bin pick -- --dump")
    raw = args.blob.read_bytes()
    fields = parse_pixel(raw)
    print(json.dumps({"parsed": fields}, indent=2))

    if not args.skip_loopback:
        lb = loopback(raw, fields)
        print(json.dumps({"loopback": lb}, indent=2))
        if not lb["match"]:
            print("LOOPBACK FAIL — no Phase 5 package")
            return 2
        print("LOOPBACK MATCH BER 0")
        if args.loopback_only:
            return 0

    slm = export_slm(raw, fields, args.out, args.preset)
    print(json.dumps({"slm": slm}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
