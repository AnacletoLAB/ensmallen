
setup_for_build:
	pip install maturin

build:
	rm -fdr target
	maturin build --release

build_with_docker:
	# Setup the docker container
	sudo docker build -t ensmallen-env .
	# Run the build making a volume from the current folder to /build inside the container
	sudo docker run -it -v "${PWD}:/build" ensmallen-env
	
coverage:
	make -C graph coverage

install:
	pip install --upgrade --user ./target/*.whl