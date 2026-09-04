.PHONY: test spec check export-shell headless demo

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
