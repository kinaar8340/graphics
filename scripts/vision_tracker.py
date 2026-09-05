#!/usr/bin/env python3
"""vision_tracker — calibrate top-of-hull camera into the sculpture frame.

v1 is eyeline lock from a known eye pixel or a 3D eye point.
Does not open the gun. Does not draw. Scan A may consume gaze.json later.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import struct
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parents[1]
TRACK = ROOT / "output" / "track"
ASSETS = ROOT / "assets"
N_OCCUPANCY = 256

# Well / attention. Constants, not a net. See docs/VISION.md.
E_DEF = np.array([0.0, 0.0, 1.0], dtype=np.float64)
THETA_DEF = 0.0
PHI_DEF = 0.0
# Even site nearest +ẑ on shell_trench.bin (sha 746a79b8…). Recomputed if samples load.
I_DEF = 40
DT_NOM = 1.0 / 30.0
ALPHA_MIN = float(np.deg2rad(3.0))
T_OFF = 2.0
TAU = 0.8
BETA = 0.3
EPS_NORM = 1e-12


def load_json(p: Path) -> dict:
    return json.loads(p.read_text())


def save_json(p: Path, obj: dict) -> None:
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(obj, indent=2) + "\n")


def as_3x3(a) -> np.ndarray:
    return np.asarray(a, dtype=np.float64).reshape(3, 3)


def camera_center(R: np.ndarray, t: np.ndarray) -> np.ndarray:
    return -R.T @ t.reshape(3)


def pixel_ray(K: np.ndarray, R: np.ndarray, t: np.ndarray, uv) -> tuple[np.ndarray, np.ndarray]:
    u, v = float(uv[0]), float(uv[1])
    x = np.linalg.inv(K) @ np.array([u, v, 1.0])
    d = R.T @ x
    d = d / np.linalg.norm(d)
    return camera_center(R, t), d


def s2_from_E(E: np.ndarray) -> dict:
    n = np.linalg.norm(E)
    if n < 1e-9:
        raise ValueError("eye coincides with sculpture origin")
    e = E / n
    theta = float(np.arccos(np.clip(e[2], -1.0, 1.0)))
    phi = float(np.arctan2(e[1], e[0]))
    return {"theta": theta, "phi": phi, "E": e.tolist(), "radius": n}


def nearest_site(Ehat: np.ndarray, trench_xyz: np.ndarray) -> int:
    g = trench_xyz / np.linalg.norm(trench_xyz, axis=1, keepdims=True)
    return int(np.argmin(1.0 - g @ Ehat))


def gamma(samples: np.ndarray, s: float) -> np.ndarray:
    n = samples.shape[0]
    t = (s % 1.0) * n
    i = int(np.floor(t)) % n
    j = (i + 1) % n
    f = t - np.floor(t)
    return samples[i] * (1.0 - f) + samples[j] * f


def occupancy_even_xyz(samples: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    """Layer-0 even occupancy sites. Returns (xyz, occupancy_index)."""
    idx = np.arange(0, N_OCCUPANCY, 2, dtype=np.int32)
    xyz = np.stack([gamma(samples, int(i) / N_OCCUPANCY) for i in idx], axis=0)
    return xyz, idx


def unit(v: np.ndarray) -> np.ndarray:
    n = float(np.linalg.norm(v))
    if n < EPS_NORM:
        return E_DEF.copy()
    return np.asarray(v, dtype=np.float64).reshape(3) / n


def angle_between(a: np.ndarray, b: np.ndarray) -> float:
    return float(np.arccos(np.clip(np.dot(unit(a), unit(b)), -1.0, 1.0)))


def default_site(samples: np.ndarray | None = None) -> int:
    if samples is None:
        return I_DEF
    even, idx = occupancy_even_xyz(samples)
    return int(idx[nearest_site(E_DEF, even)])


class AttentionWell:
    """Blob → S². Unmatched samples dropped. Drift to visual +ẑ when attention is false.

    locked = accepted blob this sample.
    attention_state = well gate (presence holds; T_off then leak).
    """

    def __init__(self, samples: np.ndarray | None = None):
        self.E_hat = E_DEF.copy()
        self.E_hat_last = E_DEF.copy()
        self.attention_state = False
        self.locked = False
        self.time_unmatched = 0.0
        self.samples = samples
        self.even_xyz = None
        self.even_idx = None
        self.i_def = I_DEF
        if samples is not None:
            self.even_xyz, self.even_idx = occupancy_even_xyz(samples)
            self.i_def = int(self.even_idx[nearest_site(E_DEF, self.even_xyz)])

    def _site(self, Ehat: np.ndarray) -> int:
        if self.even_xyz is None:
            return self.i_def
        local = nearest_site(Ehat, self.even_xyz)
        return int(self.even_idx[local])

    def step(self, meas, dt: float | None = None) -> dict:
        dt = DT_NOM if dt is None or dt <= 0 else float(dt)
        if meas is None:
            self.locked = False
            self.time_unmatched += dt
            if self.time_unmatched >= T_OFF:
                self.attention_state = False
        else:
            E_meas = unit(np.asarray(meas, dtype=np.float64).reshape(3))
            self.locked = True
            self.time_unmatched = 0.0
            self.attention_state = True
            self.E_hat_last = E_meas
            self.E_hat = unit((1.0 - BETA) * self.E_hat + BETA * E_meas)

        drift = False
        if not self.attention_state:
            lam = min(1.0, dt / TAU)
            self.E_hat = unit(self.E_hat + lam * (E_DEF - self.E_hat))
            drift = True

        alpha = angle_between(self.E_hat, E_DEF)
        if (not self.attention_state) and alpha <= ALPHA_MIN:
            self.E_hat = E_DEF.copy()
            alpha = 0.0
            site = self.i_def
        else:
            site = self._site(self.E_hat)

        s2 = s2_from_E(self.E_hat)
        return {
            "theta": s2["theta"],
            "phi": s2["phi"],
            "locked": bool(self.locked),
            "attention_state": bool(self.attention_state),
            "site": int(site),
            "layer": 0,
            "persist": 0.0,
            "sensor": "lwir",
            "alpha_from_default": float(alpha),
            "drift": bool(drift),
            "E": self.E_hat.tolist(),
            "note": "well filter; Scan A does not consume site",
        }


def read_shsc(path: Path) -> tuple[np.ndarray, np.ndarray]:
    raw = path.read_bytes()
    if raw[:4] != b"SHSC":
        raise ValueError(f"{path}: bad magic")
    version = struct.unpack_from("<I", raw, 4)[0]
    if version != 1:
        raise ValueError(f"{path}: version {version}")
    n_verts, n_faces, n_trench = struct.unpack_from("<III", raw, 8)
    off = 20
    verts = np.frombuffer(raw, dtype="<f4", count=n_verts * 3, offset=off).reshape(
        n_verts, 3
    ).astype(np.float64)
    off += n_verts * 12 + n_faces * 12
    samples = np.frombuffer(raw, dtype="<f4", count=n_trench * 3, offset=off).reshape(
        n_trench, 3
    ).astype(np.float64)
    if samples.shape[0] < 2:
        raise ValueError("trench too short")
    return verts, samples


def load_trench_xyz(path: Path) -> np.ndarray:
    xyz_sidecar = path.with_suffix(".xyz.npy")
    if xyz_sidecar.exists():
        return np.load(xyz_sidecar)
    if path.exists() and path.read_bytes()[:4] == b"SHSC":
        _verts, samples = read_shsc(path)
        return samples
    raise FileNotFoundError(f"need {path} (SHSC) or {xyz_sidecar}")


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    h.update(path.read_bytes())
    return h.hexdigest()


def look_out_Rt(C: np.ndarray, target: np.ndarray, up: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    """OpenCV convention: camera +Z forward, Y down."""
    z = target - C
    z = z / np.linalg.norm(z)
    y_down = -up / np.linalg.norm(up)
    x = np.cross(y_down, z)
    xn = np.linalg.norm(x)
    if xn < 1e-9:
        x = np.array([1.0, 0.0, 0.0])
    else:
        x = x / xn
    y = np.cross(z, x)
    R = np.column_stack([x, y, z]).T
    t = -R @ C
    return R, t


def synth_points(out_path: Path, trench_path: Path) -> dict:
    """Project six hull verts through a known pose. Bench RMS, not a live still."""
    verts, _samples = read_shsc(trench_path)
    # spread: extrema along each axis
    picks = []
    for ax in range(3):
        picks.append(int(np.argmax(verts[:, ax])))
        picks.append(int(np.argmin(verts[:, ax])))
    # unique, keep 6
    seen = []
    for i in picks:
        if i not in seen:
            seen.append(i)
        if len(seen) >= 6:
            break
    X = verts[seen[:6]]
    K = np.array([[800.0, 0.0, 640.0], [0.0, 800.0, 360.0], [0.0, 0.0, 1.0]])
    dist = np.zeros(5)
    C = np.array([0.0, 1.05, 0.15])
    target = np.array([0.0, 0.20, 2.40])
    R, t = look_out_Rt(C, target, np.array([0.0, 1.0, 0.0]))
    import cv2

    rvec, _ = cv2.Rodrigues(R.astype(np.float64))
    proj, _ = cv2.projectPoints(
        X.astype(np.float32),
        rvec,
        t.reshape(3, 1).astype(np.float32),
        K.astype(np.float32),
        dist.astype(np.float32),
    )
    uv = proj.reshape(-1, 2)
    payload = {
        "K": K.tolist(),
        "dist": dist.tolist(),
        "xyz": X.tolist(),
        "uv": uv.tolist(),
        "note": "bench solver only; replace uv with a still of that camera before treating calib as sculpture pose",
        "vert_index": seen[:6],
    }
    save_json(out_path, payload)
    print(json.dumps({"synth_points": {"n": 6, "out": str(out_path)}}, indent=2))
    return payload


def calibrate(points_path: Path, out_path: Path, K_path: Path | None) -> dict:
    pts = load_json(points_path)
    X = np.asarray(pts["xyz"], dtype=np.float64)
    uv = np.asarray(pts["uv"], dtype=np.float64)
    if X.shape[0] < 4 or X.shape != (uv.shape[0], 3):
        raise ValueError("need N>=4 matching xyz (N,3) and uv (N,2)")

    if K_path:
        kin = load_json(K_path)
        K = as_3x3(kin["K"])
        dist = np.asarray(kin.get("dist", [0, 0, 0, 0, 0]), dtype=np.float64)
    else:
        K = as_3x3(pts.get("K", [[800, 0, 640], [0, 800, 360], [0, 0, 1]]))
        dist = np.asarray(pts.get("dist", [0, 0, 0, 0, 0]), dtype=np.float64)

    try:
        import cv2
    except ImportError as e:
        raise SystemExit("calibrate needs opencv-python (cv2.solvePnP)") from e

    ok, rvec, tvec = cv2.solvePnP(
        X.astype(np.float32),
        uv.astype(np.float32),
        K.astype(np.float32),
        dist.astype(np.float32),
        flags=cv2.SOLVEPNP_ITERATIVE,
    )
    if not ok:
        raise RuntimeError("solvePnP failed")
    R, _ = cv2.Rodrigues(rvec)
    proj, _ = cv2.projectPoints(
        X.astype(np.float32),
        rvec,
        tvec,
        K.astype(np.float32),
        dist.astype(np.float32),
    )
    rms = float(np.sqrt(np.mean(np.sum((proj.reshape(-1, 2) - uv) ** 2, axis=1))))
    if rms > 2.0:
        raise SystemExit(f"rms_px={rms:.3f} > 2.0 — recapture hull marks")

    trench = ASSETS / "shell_trench.bin"
    payload = {
        "K": K.tolist(),
        "dist": dist.reshape(-1).tolist(),
        "R": R.tolist(),
        "t": tvec.reshape(3).tolist(),
        "C": camera_center(R, tvec.reshape(3)).tolist(),
        "rms_px": rms,
        "n_points": int(X.shape[0]),
        "frame": "sculpture",
        "trench": str(trench.relative_to(ROOT)) if trench.exists() else None,
        "trench_sha256": sha256_file(trench) if trench.exists() else None,
        "note": "v1 eyeline = look-at-origin from detected E",
    }
    save_json(out_path, payload)
    print(json.dumps({"calibrate": payload}, indent=2))
    return payload


def gaze(calib_path: Path, out_path: Path, eye_px=None, eye_xyz=None) -> dict:
    cal = load_json(calib_path)
    K, R, t = as_3x3(cal["K"]), as_3x3(cal["R"]), np.asarray(cal["t"], dtype=np.float64)
    if eye_xyz is not None:
        E = np.asarray(eye_xyz, dtype=np.float64).reshape(3)
        locked = True
    elif eye_px is not None:
        o, d = pixel_ray(K, R, t, eye_px)
        C = camera_center(R, t)
        target = -1.2 * C / max(np.linalg.norm(C), 1e-9)
        w = target - o
        E = o + d * float(np.dot(w, d))
        locked = True
    else:
        raise ValueError("pass --eye-px u,v or --eye-xyz x,y,z")

    s2 = s2_from_E(E)
    site = None
    trench_path = ROOT / (cal.get("trench") or "assets/shell_trench.bin")
    try:
        xyz = load_trench_xyz(trench_path)
        even, even_idx = occupancy_even_xyz(xyz)
        local = nearest_site(np.asarray(s2["E"]), even)
        site = int(even_idx[local])
    except FileNotFoundError:
        pass

    payload = {
        "theta": s2["theta"],
        "phi": s2["phi"],
        "locked": locked,
        "site": site,
        "layer": 0,
        "E": E.tolist(),
        "radius": s2["radius"],
        "persist": 0.0,
        "note": "v1 look-at-origin; Scan A may snap head to site",
    }
    save_json(out_path, payload)
    print(json.dumps({"gaze": payload}, indent=2))
    return payload


def main() -> None:
    p = argparse.ArgumentParser(description="vision_tracker calibration / gaze")
    sub = p.add_subparsers(dest="cmd", required=True)

    s = sub.add_parser("synth-points", help="bench: project hull verts (not a live still)")
    s.add_argument("-o", "--points", type=Path, default=TRACK / "points.json")
    s.add_argument("--trench", type=Path, default=ASSETS / "shell_trench.bin")

    c = sub.add_parser("calibrate")
    c.add_argument("--points", type=Path, default=TRACK / "points.json")
    c.add_argument("--K", type=Path, default=None)
    c.add_argument("-o", type=Path, default=TRACK / "calib.json")

    g = sub.add_parser("gaze")
    g.add_argument("--calib", type=Path, default=TRACK / "calib.json")
    g.add_argument("--eye-px", type=str, default=None, help="u,v in camera pixels")
    g.add_argument("--eye-xyz", type=str, default=None, help="x,y,z in sculpture frame")
    g.add_argument("-o", type=Path, default=TRACK / "gaze.json")

    f = sub.add_parser("step", help="attention well: step(meas|None, dt); no webcam")
    f.add_argument("--none", action="store_true", help="unmatched sample")
    f.add_argument("--meas-xyz", type=str, default=None, help="blob E in sculpture frame")
    f.add_argument("--dt", type=float, default=DT_NOM)
    f.add_argument("--n", type=int, default=1)
    f.add_argument("--trench", type=Path, default=ASSETS / "shell_trench.bin")
    f.add_argument("-o", type=Path, default=TRACK / "gaze.json")

    args = p.parse_args()
    if args.cmd == "synth-points":
        synth_points(args.points, args.trench)
    elif args.cmd == "calibrate":
        calibrate(args.points, args.o, args.K)
    elif args.cmd == "gaze":
        eye_px = [float(x) for x in args.eye_px.split(",")] if args.eye_px else None
        eye_xyz = [float(x) for x in args.eye_xyz.split(",")] if args.eye_xyz else None
        gaze(args.calib, args.o, eye_px=eye_px, eye_xyz=eye_xyz)
    elif args.cmd == "step":
        samples = load_trench_xyz(args.trench) if args.trench.exists() else None
        well = AttentionWell(samples)
        meas = None
        if args.meas_xyz:
            meas = [float(x) for x in args.meas_xyz.split(",")]
        elif not args.none:
            raise SystemExit("step needs --none or --meas-xyz")
        payload = None
        for _ in range(max(1, args.n)):
            payload = well.step(meas, args.dt)
        save_json(args.o, payload)
        print(json.dumps({"step": payload}, indent=2))


if __name__ == "__main__":
    main()
