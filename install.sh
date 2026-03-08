#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
INSTALL_PATH="$INSTALL_DIR/rtk"
HOST_MODE="none"
SKIP_BUILD=0

usage() {
    cat <<'EOF'
Usage: ./install.sh [--host none|claude|openclaw|both] [--skip-build]

Options:
  --host MODE   Optional host setup after install (default: none)
  --skip-build  Reuse existing target/release/rtk
  -h, --help    Show this help

Environment:
  INSTALL_DIR   Binary install dir (default: ~/.local/bin)

Examples:
  ./install.sh
  ./install.sh --host claude
  ./install.sh --host openclaw
  INSTALL_DIR=/usr/local/bin ./install.sh --host both
EOF
}

log() {
    printf '[rtk-standalone] %s\n' "$1"
}

fail() {
    printf '[rtk-standalone] error: %s\n' "$1" >&2
    exit 1
}

require_cmd() {
    command -v "$1" >/dev/null 2>&1 || fail "missing required command: $1"
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --host)
                [[ $# -ge 2 ]] || fail "--host requires a value"
                HOST_MODE="$2"
                shift 2
                ;;
            --skip-build)
                SKIP_BUILD=1
                shift
                ;;
            -h|--help)
                usage
                exit 0
                ;;
            *)
                fail "unknown argument: $1"
                ;;
        esac
    done

    case "$HOST_MODE" in
        none|claude|openclaw|both) ;;
        *) fail "invalid --host value: $HOST_MODE" ;;
    esac
}

build_binary() {
    require_cmd cargo

    if [[ "$SKIP_BUILD" -eq 1 ]]; then
        [[ -x "$SCRIPT_DIR/target/release/rtk" ]] || fail "target/release/rtk not found; remove --skip-build or build first"
        return
    fi

    log "building release binary"
    cargo build --release --manifest-path "$SCRIPT_DIR/Cargo.toml"
}

install_binary() {
    mkdir -p "$INSTALL_DIR"
    install -m 755 "$SCRIPT_DIR/target/release/rtk" "$INSTALL_PATH"
    log "installed binary: $INSTALL_PATH"
}

check_path() {
    case ":$PATH:" in
        *":$INSTALL_DIR:"*) ;;
        *)
            log "warning: $INSTALL_DIR is not on PATH"
            log "add this to your shell profile: export PATH=\"$INSTALL_DIR:\$PATH\""
            ;;
    esac
}

init_claude() {
    require_cmd jq
    log "installing Claude integration"
    "$INSTALL_PATH" init --global
}

init_openclaw() {
    log "installing OpenClaw integration"
    "$INSTALL_PATH" init --openclaw
}

run_host_init() {
    case "$HOST_MODE" in
        none) ;;
        claude) init_claude ;;
        openclaw) init_openclaw ;;
        both)
            init_claude
            init_openclaw
            ;;
    esac
}

verify_install() {
    log "verifying binary"
    "$INSTALL_PATH" --version
    "$INSTALL_PATH" gain >/dev/null
    "$INSTALL_PATH" init --show || true
}

main() {
    parse_args "$@"
    build_binary
    install_binary
    check_path
    run_host_init
    verify_install
    log "done"
}

main "$@"
