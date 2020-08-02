run:
	cargo run --release

watch:
	cargo watch -x 'run --release'

fmt:
	cargo fmt
	cargo clippy

oxipng:
	oxipng -o 3 -s *.png
