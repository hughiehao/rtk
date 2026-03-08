# RTK for OpenClaw

This plugin rewrites OpenClaw `exec` tool calls to RTK equivalents before execution.

## Install

```bash
rtk init --openclaw
```

## What it does

- rewrites supported shell commands through `rtk rewrite --json`
- injects OpenClaw session metadata into RTK tracking via environment variables
- keeps command mapping in Rust as the single source of truth

## Notes

- unsupported commands pass through unchanged
- the plugin only intercepts the `exec` tool
- OpenClaw must be able to find `rtk` on `PATH`, or set `rtkPath` in plugin config
