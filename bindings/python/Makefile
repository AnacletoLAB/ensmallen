CPU_FEATURES:="+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+sse4a,+avx,+avx2,+bmi1,+bmi2,+lzcnt,+popcnt,+cmov"

build:
	rm -fdr target
	rustup update
	cargo update
	(cd ../../code_analysis; cargo run --release --bin bindgen)
	RUSTFLAGS="-C opt-level=3 -Ctarget-feature=${CPU_FEATURES} -C inline-threshold=1000" maturin build --release --strip -i python3.6

atheris:
# https://clang.llvm.org/docs/SanitizerCoverage.html
	maturin develop --release --rustc-extra-args='-Ctarget-cpu=native -Zinstrument-coverage -Cpasses=sancov -Cllvm-args=-sanitizer-coverage-level=4  -Cllvm-args=-sanitizer-coverage-trace-compares  -Cllvm-args=-sanitizer-coverage-inline-8bit-counters  -Cllvm-args=-sanitizer-coverage-pc-table -Cllvm-args=-sanitizer-coverage-stack-depth --verbose -Zsanitizer=address'

install:
	pip install --upgrade --user --no-index --find-links=target/wheels ensmallen

test:
	pytest -s ./pytests
