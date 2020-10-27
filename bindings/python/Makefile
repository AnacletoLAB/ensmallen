build:
	rm -fdr target
	rustup update
	cargo update
	maturin build --release

install:
	pip install --upgrade --user --no-index --find-links=target/wheels ensmallen_graph

test:
	pytest -s ./pytests