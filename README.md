# Standalone RTK Source Distribution

This directory is a self-contained RTK source project that includes the files required to build the current RTK binary with Claude and OpenClaw integration support.

## Included

- `Cargo.toml`, `Cargo.lock`
- `src/` (Rust source used for compilation)
- `hooks/` (Claude integration assets embedded by `init`)
- `openclaw/` (OpenClaw plugin assets embedded by `init`)
- `LICENSE`
- `install.sh` (build + install helper)

## Recommended binary install path

Default binary install path:

```bash
~/.local/bin
```

This is the recommended default because it avoids `sudo` and matches RTK's existing installer behavior.

If you need a system path instead, override it explicitly:

```bash
INSTALL_DIR=/usr/local/bin ./install.sh --host both
```

## Build and install

```bash
./install.sh
```

Install and initialize Claude integration:

```bash
./install.sh --host claude
```

Install and initialize OpenClaw integration:

```bash
./install.sh --host openclaw
```

Install and initialize both:

```bash
./install.sh --host both
```

## Requirements

Build-time requirements:

- Rust toolchain (`cargo`, `rustc`)

Optional host setup requirements:

- `jq` for Claude installation flow
- OpenClaw installed for OpenClaw plugin setup

End-user machines that receive a prebuilt `rtk` binary do not need Rust.
