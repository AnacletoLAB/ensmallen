
setup:
	pip install -r requirements.txt

build:
	rm -fdr target
	maturin build --release

coverage:
	make -C graph coverage

install:
	pip install --upgrade --user ./target/*.whl