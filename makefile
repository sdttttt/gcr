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