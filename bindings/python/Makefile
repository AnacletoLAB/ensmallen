develop:
	cargo update
	python3 ./build.py --skip-repair
	pip uninstall --yes ensmallen
	pip --disable-pip-version-check install ./wheels/merged/ensmallen*.whl