.PHONY: all
all:
	@echo "Run my targets individually!"

.PHONY: venv
.ONESHELL:
venv:
	test -d venv || python3 -m venv venv
	. venv/bin/activate
	pip install -r requirements-dev.txt


.PHONY: develop
.ONESHELL:
develop: venv
	. venv/bin/activate
	maturin develop

.PHONY: lint
.ONESHELL:
lint: 
	cargo fmt
	cargo clippy --all-targets --all-features

.PHONY: test
.ONESHELL:
test: develop
	. venv/bin/activate
	python -m pytest -vv -s python_tests.py

.PHONY: benchmark
.ONESHELL:
benchmark: venv
	. venv/bin/activate
	rm -rf target/release/rusty_rlp.so
	cargo build --release
	cp target/release/librusty_rlp.so target/release/rusty_rlp.so
	echo "PyRLP Roundtrip (NOT THIS LIBRARY)"
	python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_pyrlp_roundtrip()'
	echo "Rusty RLP Roundtrip"
	python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_rustyrlp_roundtrip()'

	echo "PyRLP Decoding (NOT THIS LIBRARY)"
	python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_pyrlp_decoding()'
	echo "Rusty RLP Decoding"
	python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_rustyrlp_decoding()'

	echo "PyRLP Encoding (NOT THIS LIBRARY)"
	python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_pyrlp_encoding()'
	echo "Rusty RLP Encoding"
	python -m timeit -n 300 -u msec  -s'import benchmark' 'benchmark.bench_rustyrlp_encoding()'

.PHONY: build
.ONESHELL:
build: venv
	. venv/bin/activate
	maturin build

.PHONY: dist
.ONESHELL:
dist: venv
	. venv/bin/activate
	pip install twine
	rm -rf target/wheels/*
	docker run --rm -v $(shell pwd):/io konstin2/maturin build --release --strip
	twine upload target/wheels/*
