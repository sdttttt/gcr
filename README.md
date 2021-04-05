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

> **🚧Note:** The bad news is that my workflow is broken and I have no time to fix it. In Release, only download the highest `0.9.2` version of GRC. If possible, use Cargo to install it.

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

Starting with **1.0.0**, grc can enhance your submission with the `--emoji` command line argument.

Or in the configuration file:

```toml
emoji = true
```

You can also use `overwrite_emoji` to enhance custom submission types or override basic submission types in GRC.

```toml
overwrite_emoji = [
    "deps:🚕", # Appends an emoji to a custom submission type
    "test:🚗"  # Test is a GRC built-in submission type that you can override.
]
```

### Plug

plug are a new feature added in `1.1.0`. Details of the plug-in and usage can be found [here](https://github.com/sdttttt/gcr/tree/develop/src/plugins).

## IDEA

If you have any new ideas, you are welcome to talk to me.

GRC repo is used GRC to commit!
