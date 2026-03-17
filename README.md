<!--
SPDX-FileCopyrightText: 2026 The mwifiex_nl80211_test_mode authors
SPDX-License-Identifier: GPL-2.0-only
-->

# mwifiex NL80211 Wi-Fi RF Test Mode

A netlink user space implementation of all Wi-Fi RF test mode commands from
[AN14114: RF Test Mode on LinuxOS](https://docs.nxp.com/bundle/AN14114/page/topics/wi-fi_rf_test_mode.html)
and OTP programming commands for all Marvell/NXP Wi-Fi cards supported by the
[mainline Linux mwifiex](https://github.com/torvalds/linux/tree/v6.19/drivers/net/wireless/marvell/mwifiex)
and [downstream NXP mwifiex](https://github.com/nxp-imx/mwifiex) driver.

## Usage

```
mwifiex_test_mode [OPTIONS]
```

| Flag                              | Description                           | Default |
| --------------------------------- | ------------------------------------- | ------- |
| `-i, --interface <INTERFACE>`     | Mwifiex network interface             | `mlan0` |
| `--interactive`                   | Launch the interactive shell          | -       |
| `-l, --list-commands`             | Print YAML syntax reference           | -       |
| `-f, --file <FILE>`               | Path to a YAML sequence file          | -       |
| `-s, --set <KEY=VALUE>`           | Override a YAML variable (repeatable) | -       |
| `-d, --dry-run`                   | Only simulate sending commands        | -       |
| `-c, --card-type <CARD_TYPE>`     | Wi-Fi card type                       | `iw610` |
| `-V, --version`                   | Print version                         | -       |

## Modes of operation

There are two modes of operation which have different goals in mind:

1. Select and run commands inside an interactive shell:
    * Quick exploration and prototyping of command sequences
2. Execute a command sequence defined in YAML files:
    * Repeatable test sequences for usage during development and certification

**Interactive shell** - select and configure commands from a menu:

```sh
mwifiex_nl_test_mode --interactive
```

**YAML sequence file** - run a predefined test sequence:

```sh
mwifiex_nl_test_mode -f examples/88w9098/5_ghz_tx_frame.yaml --card-type 9098
```

**YAML sequence file with variable overrides** - override default values from the command line:

```sh
mwifiex_nl_test_mode -f examples/5_ghz_rx.yaml --set channel=116 --set bandwidth=bw80
```

**Dry run mode** - validate a sequence without actual hardware:

```sh
mwifiex_nl_test_mode --dry-run -f examples/syntax.yaml
```

## [YAML command sequence syntax](examples/syntax.yaml)

A sequence files describe a list of commands to execute in order. A file consists
of an optional `variables` block (overridable via `--set`) and a required
`steps` list. The complete syntax is documented under
[**examples/syntax.yaml**](examples/syntax.yaml). The example folder includes all
examples from [AN14114: RF Test Mode on
LinuxOS](https://docs.nxp.com/bundle/AN14114/page/topics/wi-fi_rf_test_mode.html)
ported to YAML as well.

## Kernel NL80211 test mode support

The tool communicates with the mwifiex driver via the NL80211 test mode
implementation, which needs to be enabled on the target system's kernel:

```Make
CONFIG_NL80211_TESTMODE=y
```

## Cross-compilation with [`cross`](https://github.com/cross-rs/cross)

The project includes a Makefile for easy cross-compilation.

### Prerequisites

Before building, ensure you have the following tools installed:

* [`cross`](https://github.com/cross-rs/cross) - Cross-compilation tool for Rust
* [`rustup`](https://rustup.rs/) - Rust toolchain manager
* [`podman`](https://podman.io/) or [`docker`](https://www.docker.com/) - Container runtime for cross

You can verify all dependencies are installed by running:

```bash
make check-toolchain
```

### Makefile usage

To see all available make targets and options:

```bash
make help
```

Common build commands:

```bash
# Build binaries and CPIO archives for all default targets
make all

# Build for a specific target only
make aarch64-unknown-linux-musl-build

# Use Docker instead of Podman
make CROSS_CONTAINER_ENGINE=docker all
```

Default target architectures:
* `aarch64-unknown-linux-musl`
* `arm-unknown-linux-musleabihf`
* `x86_64-unknown-linux-musl`

Build artifacts are placed in `target/<arch>/<profile>/` directories.

## Known limitations

* No big endian support
* No auto-detection of the card type - must be supplied via CLI argument

## License

GPL-2.0-only
