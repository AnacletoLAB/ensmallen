build:
	rm -fdr target
	rustup update
	cargo update
	RUSTFLAGS="-C opt-level=3 -C target-cpu=native -C inline-threshold=1000" maturin build --release

build_linux:
	rm -fdr target
	rustup update
	cargo update
	RUSTFLAGS="-C opt-level=3 -C target-cpu=native -C inline-threshold=1000" maturin build --release --target=x86_64-unknown-linux-gnu

install:
	pip install --upgrade --user --no-index --find-links=target/wheels ensmallen_graph

test:
	pytest -s ./pytests
