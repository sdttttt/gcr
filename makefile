c = cargo
g = grc

.PHONY: build
build: fmt
	$(c) build --release

.PHONY: build-dev
build-dev: fmt
	$(c) build -v

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
cov:CARGO_INCREMENTAL=0
cov:RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
cov:RUSTDOCFLAGS="-Cpanic=abort"
cov: fmt
	
	$(c) build
	$(c) test
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/