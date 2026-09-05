.PHONY: test spec check export-shell headless demo stills testimony tick scan nest-headless nest-stills pick slm-loopback slm-export track-synth track-calibrate track-gaze

TRACK_PY := $(shell test -x .venv/bin/python && echo .venv/bin/python || echo python3)
EYE ?= 640,360

test:
	cargo test

spec:
	@cat docs/SPEC.md

check:
	cargo test --offline 2>/dev/null || cargo test

export-shell:
	PYTHONPATH=../flux_trajectoid/src python3 scripts/export_shell_trench.py

headless:
	cargo run --release --bin shellscan -- --headless --frames 8

demo:
	cargo run --release --bin shellscan

# Phase 6 writer. Hemisphere → 32-byte qga_pixel. Not the faceplate.
pick:
	cargo run --release --bin pick

# Phase 5 sidecar. Reads output/pick/qga_pixel.bin. Not a window. Not a gun.
slm-loopback:
	cargo run --release --bin pick -- --dump
	python3 scripts/export_slm_pixel.py --loopback-only

slm-export:
	cargo run --release --bin pick -- --dump
	python3 scripts/export_slm_pixel.py --preset generic_512

# vision_tracker. Calibration sidecar. No window. No gun. Scan A does not consume site yet.
track-synth:
	mkdir -p output/track
	$(TRACK_PY) scripts/vision_tracker.py synth-points --points output/track/points.json

track-calibrate:
	mkdir -p output/track
	$(TRACK_PY) scripts/vision_tracker.py calibrate --points output/track/points.json

track-gaze:
	mkdir -p output/track
	$(TRACK_PY) scripts/vision_tracker.py gaze --eye-px $(EYE)

stills:
	mkdir -p output/png
	cargo run --release --bin shellscan -- --stills --width 1280 --height 720

# N1 confocal stack. Ledger first, then locked-eye stills. L=3, ΔR=0.08R. LF=0.
nest-headless:
	mkdir -p output/nest
	cargo run --release --bin shellscan -- --nest-headless

nest-stills:
	mkdir -p output/png
	cargo run --release --bin shellscan -- --nest-stills --width 1280 --height 720

# Animation A. Persist peak on even sites. Locked eye + 04 crop. Glow off. HUD off. LF=0.
scan:
	mkdir -p output/png/scan_lock output/png/scan_crop output/mp4 output/scan
	rm -f output/png/scan_lock/frame_*.png output/png/scan_crop/frame_*.png
	cargo run --release --bin shellscan -- --scan --width 1280 --height 720
	ffmpeg -y -hide_banner -loglevel error -framerate 30 \
		-i output/png/scan_lock/frame_%04d.png \
		-c:v libx264 -pix_fmt yuv420p \
		output/mp4/scan_lock.mp4
	ffmpeg -y -hide_banner -loglevel error -framerate 30 \
		-i output/png/scan_crop/frame_%04d.png \
		-c:v libx264 -pix_fmt yuv420p \
		output/mp4/scan_crop.mp4

# Locked-eye 6s tick. Hard cuts. Not both.mp4. Glow off. HUD off. LF=0.
tick:
	mkdir -p output/png/tick output/mp4
	rm -f output/png/tick/frame_*.png
	cargo run --release --bin shellscan -- --tick --width 1280 --height 720
	ffmpeg -y -hide_banner -loglevel error -framerate 30 \
		-i output/png/tick/frame_%04d.png \
		-c:v libx264 -pix_fmt yuv420p \
		output/mp4/tick.mp4

# 30s @ 30fps orbit of 03_both. Glow off. No HUD. Gun off. Body, not clocks.
testimony: stills
	mkdir -p output/png/orbit output/mp4
	rm -f output/png/orbit/frame_*.png
	cargo run --release --bin shellscan -- --orbit --frames 900 --width 1280 --height 720
	ffmpeg -y -hide_banner -loglevel error -framerate 30 \
		-i output/png/orbit/frame_%04d.png \
		-c:v libx264 -pix_fmt yuv420p \
		output/mp4/both.mp4
