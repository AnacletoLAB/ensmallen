CPU_FEATURES:="+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+sse4a,+avx,+avx2,+bmi1,+bmi2,+lzcnt,+popcnt,+cmov"

build_avx:
	rm -fdr target
	rm -fdr wheels_avx
	mkdir wheels_avx
	rustup update
	cargo update
	(cd ../../code_analysis; cargo run --release --bin bindgen)
	cp pyproject_avx.toml pyproject.toml
	RUSTFLAGS="-C opt-level=3 -Ctarget-feature=${CPU_FEATURES} -C inline-threshold=1000" maturin build --release --strip  --out ./wheels_avx

build_no_avx:
	rm -fdr target
	rm -fdr wheels_no_avx
	mkdir wheels_no_avx
	rustup update
	cargo update
	(cd ../../code_analysis; cargo run --release --bin bindgen)
	cp pyproject_no_avx.toml pyproject.toml
	RUSTFLAGS="-C opt-level=3 -C inline-threshold=1000" maturin build --release --strip --out ./wheels_no_avx

build:
	make build_avx
	make build_no_avx

atheris:
# https://clang.llvm.org/docs/SanitizerCoverage.html
	maturin develop --release --rustc-extra-args='-Ctarget-cpu=native -Zinstrument-coverage -Cpasses=sancov -Cllvm-args=-sanitizer-coverage-level=4  -Cllvm-args=-sanitizer-coverage-trace-compares  -Cllvm-args=-sanitizer-coverage-inline-8bit-counters  -Cllvm-args=-sanitizer-coverage-pc-table -Cllvm-args=-sanitizer-coverage-stack-depth --verbose -Zsanitizer=address'

install:
	pip install --upgrade --user --no-index --find-links=target/wheels ensmallen

test:
	pytest -s ./pytests
