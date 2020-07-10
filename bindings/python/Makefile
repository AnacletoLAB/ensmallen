build:
	rm -fdr target
	maturin build --release

install:
	pip install --upgrade --user ./target/*.whl

test:
	pytest -s ./pytests