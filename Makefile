.PHONY: test spec check export-shell headless demo stills testimony

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

stills:
	mkdir -p output/png
	cargo run --release --bin shellscan -- --stills --width 1280 --height 720

# 30s @ 30fps orbit of 03_both. Glow off. No HUD. Gun off.
testimony: stills
	mkdir -p output/png/orbit output/mp4
	rm -f output/png/orbit/frame_*.png
	cargo run --release --bin shellscan -- --orbit --frames 900 --width 1280 --height 720
	ffmpeg -y -hide_banner -loglevel error -framerate 30 \
		-i output/png/orbit/frame_%04d.png \
		-c:v libx264 -pix_fmt yuv420p \
		output/mp4/both.mp4
