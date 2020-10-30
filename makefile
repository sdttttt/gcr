c = cargo

.PHONY: build
build:
	$(c) build --release

.PHONY: build-dev
build-dev:
	$(c) build -v

.PHONY: test
test:
	$(c) test -j 1 -v