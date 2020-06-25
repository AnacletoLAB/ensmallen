build:
	rm -fdr target
	maturin build --release

build_for_linux:
	# Fully statically compiled libray, the executable will be a lot bigger
	# but it's totally portable and faster
	RUSTFLAGS="-C target-feature=-crt-static" maturin build --release --no-sdist --target x86_64-unknown-linux-musl

build_with_docker:
	# Setup the docker container
	sudo docker build -t ensmallen-env .
	# Run the build making a volume from the current folder to /build inside the container
	sudo docker run -it -v "${PWD}:/build" ensmallen-env

coverage:
	(cd graph && make coverage)

fuzz:
	(cd graph && make fuzz)

fuzz_coverage:
	(cd graph && make fuzz_coverage)

install:
	pip install --upgrade --user ./target/*.whl