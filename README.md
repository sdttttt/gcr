# GRC

 ![](https://img.shields.io/crates/v/grc.svg)
![Rust](https://github.com/sdttttt/gcr/workflows/Rust/badge.svg)
![Release](https://github.com/sdttttt/gcr/workflows/Release/badge.svg)
[![codecov](https://codecov.io/gh/sdttttt/gcr/branch/master/graph/badge.svg)](https://codecov.io/gh/sdttttt/gcr)

Standardized Git submission tool. “Grc” is a temporary name.

Similar to git-cz, grc will help you to provide a better Git experience.

## Install

You can install **grc** in the following ways:

### From Cargo

Please Run:

```sh
cargo install grc
```

### Release Package

Go to [RELEASE](https://github.com/sdttttt/gcr/releases), download one you like.

### From Source

Please Run:

```sh
git clone --depth=1 https://github.com/sdttttt/gcr.git

cd gcr

cargo build --release
```

grc under `target/release/`.

## Using

after the tools install, run command in your repository:

```sh
grc
```

GRC can also automatically help you add files to the index.

```sh
//Add all files
grc -a .

// Add specified file
grc -a <filename>...
```

### GRC config file

> **TIP:**
> This feature is supported above version 0.9.0

You can append custom commit types in the `grc.toml` configuration file:

```toml
# A semicolon separates the type from the description of the type.
type = [
    "type: this is new commit type."
]
```

## IDEA

If you have any new ideas, you are welcome to talk to me.

`grc` has taken over all git submissions from @sdttttt.
