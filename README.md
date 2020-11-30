# GRC

 ![GRC](https://img.shields.io/crates/v/grc.svg)
![Rust](https://github.com/sdttttt/gcr/workflows/Rust/badge.svg)
![Release](https://github.com/sdttttt/gcr/workflows/Release/badge.svg)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/6501c2d5bc19413dbbd297c6ee39e060)](https://app.codacy.com/gh/sdttttt/gcr?utm_source=github.com&utm_medium=referral&utm_content=sdttttt/gcr&utm_campaign=Badge_Grade)
[![codecov](https://codecov.io/gh/sdttttt/gcr/branch/master/graph/badge.svg)](https://codecov.io/gh/sdttttt/gcr)

Semantic git commits tool.

Similar to `git-cz`, grc will help you to provide a better Git experience.

## Install

You can install **grc** in the following ways:

### From crates.io

Please Run:

```sh
cargo install grc
```

### Release Package

Go to [RELEASE](https://github.com/sdttttt/gcr/releases), download one you like.

### From Source

Please Run:

```sh
cargo install --git https://github.com/sdttttt/gcr.git
```

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

You can append custom commit types in the `grc.toml` configuration file at repo root directory:

```toml
# A colon separates the type from the description of the type.
type = [
    "type: this is new commit type."
]
```

Starting with **0.9.1**, grc using `~/.config/grc/grc.toml` as the default configuration file.

## IDEA

If you have any new ideas, you are welcome to talk to me.

GRC repo is used GRC to commit!
