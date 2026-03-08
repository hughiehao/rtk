# RTK GitHub Release Installation Guide

This guide explains how to:

1. find the correct RTK release asset on GitHub
2. download it to the local machine
3. install the `rtk` binary into the user-level bin directory
4. run the correct integration command for Claude Code or OpenClaw

This document is written for both:

- human users
- AI agents that need deterministic, copy-pasteable commands

## Repository and release URL pattern

Current GitHub repository:

```text
https://github.com/hughiehao/rtk
```

GitHub Release asset URL pattern:

```text
https://github.com/<owner>/<repo>/releases/download/<tag>/<asset-name>
```

For this repository, that becomes:

```text
https://github.com/hughiehao/rtk/releases/download/<tag>/<asset-name>
```

Example:

```text
https://github.com/hughiehao/rtk/releases/download/v0.27.4/rtk-aarch64-apple-darwin.tar.gz
```

## Release assets produced by CI

The release workflow publishes these binary archives:

- `rtk-aarch64-apple-darwin.tar.gz`
- `rtk-x86_64-apple-darwin.tar.gz`
- `rtk-x86_64-unknown-linux-musl.tar.gz`
- `rtk-aarch64-unknown-linux-gnu.tar.gz`
- `checksums.txt`

## Choose the correct asset

Use this mapping:

| System | CPU | Asset |
|---|---|---|
| macOS | Apple Silicon (`arm64`) | `rtk-aarch64-apple-darwin.tar.gz` |
| macOS | Intel (`x86_64`) | `rtk-x86_64-apple-darwin.tar.gz` |
| Linux | Intel/AMD 64-bit (`x86_64`) | `rtk-x86_64-unknown-linux-musl.tar.gz` |
| Linux | ARM64 (`aarch64`) | `rtk-aarch64-unknown-linux-gnu.tar.gz` |

## Detect the local system

Run:

```bash
uname -s
uname -m
```

Expected values:

- macOS Apple Silicon: `Darwin` + `arm64`
- macOS Intel: `Darwin` + `x86_64`
- Linux x86_64: `Linux` + `x86_64`
- Linux ARM64: `Linux` + `aarch64`

## Install location

Recommended user-level install directory:

```bash
~/.local/bin
```

Installed binary path:

```bash
~/.local/bin/rtk
```

## Manual installation steps

### 1. Set release variables

Replace `TAG` and `ASSET` with the correct values.

```bash
REPO_OWNER="hughiehao"
REPO_NAME="rtk"
TAG="v0.27.4"
ASSET="rtk-aarch64-apple-darwin.tar.gz"
INSTALL_DIR="$HOME/.local/bin"
```

### 2. Download the selected release asset

```bash
mkdir -p "$INSTALL_DIR"
curl -fL "https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${TAG}/${ASSET}" -o "/tmp/${ASSET}"
```

### 3. Extract the archive

```bash
tar -xzf "/tmp/${ASSET}" -C /tmp
```

### 4. Install the binary into the user bin directory

```bash
install -m 755 /tmp/rtk "$INSTALL_DIR/rtk"
```

### 5. Verify the binary

```bash
"$INSTALL_DIR/rtk" --version
"$INSTALL_DIR/rtk" gain
```

`rtk gain` should work. If it does not, the binary is wrong or broken.

## One-shot install commands by platform

### macOS Apple Silicon

```bash
mkdir -p "$HOME/.local/bin" \
&& curl -fL "https://github.com/hughiehao/rtk/releases/download/v0.27.4/rtk-aarch64-apple-darwin.tar.gz" -o /tmp/rtk.tar.gz \
&& tar -xzf /tmp/rtk.tar.gz -C /tmp \
&& install -m 755 /tmp/rtk "$HOME/.local/bin/rtk" \
&& "$HOME/.local/bin/rtk" --version \
&& "$HOME/.local/bin/rtk" gain
```

### macOS Intel

```bash
mkdir -p "$HOME/.local/bin" \
&& curl -fL "https://github.com/hughiehao/rtk/releases/download/v0.27.4/rtk-x86_64-apple-darwin.tar.gz" -o /tmp/rtk.tar.gz \
&& tar -xzf /tmp/rtk.tar.gz -C /tmp \
&& install -m 755 /tmp/rtk "$HOME/.local/bin/rtk" \
&& "$HOME/.local/bin/rtk" --version \
&& "$HOME/.local/bin/rtk" gain
```

### Linux x86_64

```bash
mkdir -p "$HOME/.local/bin" \
&& curl -fL "https://github.com/hughiehao/rtk/releases/download/v0.27.4/rtk-x86_64-unknown-linux-musl.tar.gz" -o /tmp/rtk.tar.gz \
&& tar -xzf /tmp/rtk.tar.gz -C /tmp \
&& install -m 755 /tmp/rtk "$HOME/.local/bin/rtk" \
&& "$HOME/.local/bin/rtk" --version \
&& "$HOME/.local/bin/rtk" gain
```

### Linux ARM64

```bash
mkdir -p "$HOME/.local/bin" \
&& curl -fL "https://github.com/hughiehao/rtk/releases/download/v0.27.4/rtk-aarch64-unknown-linux-gnu.tar.gz" -o /tmp/rtk.tar.gz \
&& tar -xzf /tmp/rtk.tar.gz -C /tmp \
&& install -m 755 /tmp/rtk "$HOME/.local/bin/rtk" \
&& "$HOME/.local/bin/rtk" --version \
&& "$HOME/.local/bin/rtk" gain
```

## PATH setup

If `~/.local/bin` is not already on `PATH`, add it.

### zsh

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### bash

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Host integration commands

After the binary is installed, choose one integration mode.

### Claude Code: current project only

Run this inside the target project directory:

```bash
rtk init
```

This installs project-local Claude integration using:

- `./.claude/hooks/rtk-rewrite.sh`
- `./.claude/settings.json`
- `./RTK.md`
- `./CLAUDE.md` with `@RTK.md`

### Claude Code: global installation

```bash
rtk init -g
```

This installs global Claude integration under `~/.claude/`.

### OpenClaw

```bash
rtk init --openclaw
```

This installs the OpenClaw plugin under:

```text
~/.openclaw/extensions/rtk-rewrite/
```

### Claude + OpenClaw

If both are needed, run both commands:

```bash
rtk init
rtk init --openclaw
```

or, if Claude should be global:

```bash
rtk init -g
rtk init --openclaw
```

## AI agent quick path

If you are an AI agent, follow this exact flow:

1. detect `uname -s` and `uname -m`
2. map the platform to the correct asset filename
3. download the asset from GitHub Releases
4. install `rtk` to `~/.local/bin/rtk`
5. verify with:

```bash
~/.local/bin/rtk --version
~/.local/bin/rtk gain
```

6. if the user wants Claude project-local integration:

```bash
~/.local/bin/rtk init
```

7. if the user wants Claude global integration:

```bash
~/.local/bin/rtk init -g
```

8. if the user wants OpenClaw integration:

```bash
~/.local/bin/rtk init --openclaw
```

## Verification checklist

- [ ] correct asset selected for the current OS and CPU
- [ ] archive downloaded successfully from GitHub Releases
- [ ] binary installed to `~/.local/bin/rtk`
- [ ] `rtk --version` works
- [ ] `rtk gain` works
- [ ] the requested host integration command has been run

## Notes

- Release archives contain the `rtk` binary directly.
- GitHub Releases also publish `checksums.txt`.
- OpenClaw requires `rtk` to be on `PATH`, or the plugin must be configured with a custom `rtkPath`.
- For Claude project-local integration, run `rtk init` from the project root.
