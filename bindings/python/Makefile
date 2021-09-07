build:
	rm -fdr target
	rustup update
	cargo update
	(cd ../../code_analysis; cargo run --release --bin bindgen)
	RUSTFLAGS="-C opt-level=3 -C target-cpu=native -C inline-threshold=1000" maturin build --release --strip

atheris:
# https://clang.llvm.org/docs/SanitizerCoverage.html
	maturin develop --release --rustc-extra-args='-Ctarget-cpu=native -Zinstrument-coverage -Cpasses=sancov -Cllvm-args=-sanitizer-coverage-level=4  -Cllvm-args=-sanitizer-coverage-trace-compares  -Cllvm-args=-sanitizer-coverage-inline-8bit-counters  -Cllvm-args=-sanitizer-coverage-pc-table -Cllvm-args=-sanitizer-coverage-stack-depth --verbose -Zsanitizer=address'

install:
	pip install --upgrade --user --no-index --find-links=target/wheels ensmallen

test:
	pytest -s ./pytests
