.PHONY: build clean re test build_debug
.PHONY: lint fmt fmt_check audit

NAME := bloom_core
VERSION := $(shell cat Cargo.toml | grep '^version =' | cut -d '"' -f2)
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")

all: build

dev:
	cargo watch -x 'check'

build:
	cargo build --release

build_debug:
	cargo build

clean:
	rm -rf target/

re: clean build

test:
	cargo test
	cd bloom/auth && cargo test

fmt:
	cargo fmt

fmt_check:
	cargo fmt --all -- --check

lint:
	cargo clippy -- -D warnings -A clippy::needless_return

audit:
	cargo audit
