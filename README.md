# GRC

| NAME | STATE
|--|--
|Test| ![Rust](https://github.com/sdttttt/gcr/workflows/Rust/badge.svg)
|Release| ![Release](https://github.com/sdttttt/gcr/workflows/Release/badge.svg)

Standardized Git submission tool. “Grc” is a temporary name.

Similar to git-cz, gcr will help you to provide a better Git experience.

## Install

You can install **grc** in the following ways:

### From Cargo

Please Run `cargo install grc`.

### Release Package

Go to [RELEASE](https://github.com/sdttttt/gcr/releases), download one you like.

### From Source

Please Run:

```bash
git clone --depth=1 https://github.com/sdttttt/gcr.git

cd gcr

cargo build --release
```

grc under `target/release/`.

## Using

after the tools install, run command in your repository:

```
grc
```