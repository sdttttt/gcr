# GRC

![GRC](https://img.shields.io/crates/v/grc.svg)
![Rust](https://github.com/sdttttt/gcr/workflows/Rust/badge.svg)
![Release](https://github.com/sdttttt/gcr/workflows/Release/badge.svg)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/6501c2d5bc19413dbbd297c6ee39e060)](https://app.codacy.com/gh/sdttttt/gcr?utm_source=github.com&utm_medium=referral&utm_content=sdttttt/gcr&utm_campaign=Badge_Grade)
[![codecov](https://codecov.io/gh/sdttttt/gcr/branch/master/graph/badge.svg)](https://codecov.io/gh/sdttttt/gcr)


Compact specification git commit tool, written in Rust, it has a variety of practical small functions. 

Similar to `git-cz`, GRC will help you to provide a better git commit experience.

## Install

You can install **grc** in the following ways:

### From crates.io

grc has very few dependencies, and you can build it very quickly and easily!

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
Starting with **0.9.1**, grc using `~/.config/grc/grc.toml` as the default configuration file.

```toml
# A colon separates the type from the description of the type.
type = [
    "type: this is new commit type."
]

# Starting with **1.0.0**, grc can enhance your submission with the `--emoji` command line argument.
emoji = true

# You can also use `overwrite_emoji` to enhance custom submission types or override basic submission types in GRC.
overwrite_emoji = [
    "deps:🚕", # Appends an emoji to a custom submission type
    "test:🚗"  # Test is a GRC built-in submission type that you can override.
]


# Starting with **1.2.0**, Added two new options to the configuration file, `pre` and `after`, which are similar to githook. Here you can enter the actions of the commands before and after COMMIT. (This feature may not work properly on Windows.)
pre = [
"cargo test"
]

after = [
"echo Ok!"
]

```

### Plug

plug are a new feature added in `1.1.0`. Details of the plug-in and usage can be found [here](https://github.com/sdttttt/gcr/tree/master/src/plugins).

## IDEA

If you have any new ideas, you are welcome to talk to me.

GRC repo is used GRC to commit!
