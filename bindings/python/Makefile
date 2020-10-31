build:
	rm -fdr target
	rustup update
	cargo update
	RUSTFLAGS="-C opt-level=3 -C target-cpu=native -C inline-threshold=1000000" maturin build --release

install:
	pip install --upgrade --user --no-index --find-links=target/wheels ensmallen_graph

test:
	pytest -s ./pytests
