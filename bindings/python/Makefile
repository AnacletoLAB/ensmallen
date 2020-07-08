build:
	rm -fdr target
	maturin build --release

build_native:
	# Fully statically compiled libray, the executable will be a lot bigger
	# but it's totally portable and faster
	RUSTFLAGS=" -C target-cpu=native" maturin build --release --no-sdist

build_with_docker:
	# Setup the docker container
	sudo docker build -t ensmallen-env -f ../../setup/Dockerfile .
	# Run the build making a volume from the current folder to /build inside the container
	sudo docker run -it -v "${PWD}:/build" ensmallen-env

install:
	pip install --upgrade --user ./target/*.whl