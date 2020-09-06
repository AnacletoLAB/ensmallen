build:
	rm -fdr target
	cargo update
	maturin build --release

install:
	pip install --upgrade --user --no-index --find-links=target/wheels ensmallen_graph

test:
	pytest -s ./pytests