SHELL := /bin/bash

c = cargo
g = grc

.PHONY: build
build: fmt
	$(c) build --release

.PHONY: run
run: fmt
	$(c) run

.PHONY: test
test: fmt
	$(c) test -j 1 -v

.PHONY: fmt
fmt:
	$(c) fmt

.PHONY: publish
publish: fmt
	$(c) publish -v

.PHONY: commit
commit: fmt test run

.PHONY: cov
cov:
	set -e

	export CARGO_INCREMENTAL=0
	export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
	export RUSTDOCFLAGS="-Cpanic=abort"

	rustup default nightly

	cargo build
	cargo test
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/