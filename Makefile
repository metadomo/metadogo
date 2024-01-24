ROOT_DIR=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

build:
	@cargo contract build

build-release:
	@cargo contract build --release

subscan-arm:
	$(ROOT_DIR)/script/subscan.sh arm64

subscan-amd:
	$(ROOT_DIR)/script/subscan.sh amd64

clean:
	@cargo clean

TESTS = ""
test:
	@cargo test $(TESTS) --offline -- --color=always --test-threads=1 --nocapture

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	touch campman/src/**
	cargo clippy --all-targets --all-features -- -D warnings

dev:
	@cargo run

.PHONY: build test docs style-check lint