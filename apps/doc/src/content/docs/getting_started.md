---
title: Getting started
description: Install, build, test, and run the khaos workspace.
sidebar:
  order: 1
---

This guide gets the repository running locally. The packages are not published yet, so khaos is currently used from its source workspace.

## Requirements

Install the following tools:

- Node.js 22.12 or newer
- pnpm 9 or newer
- A stable Rust toolchain
- A C/C++ build toolchain supported by Rust and NAPI-rs

Check the installed versions:

```bash
node --version
pnpm --version
rustc --version
cargo --version
```

## Clone the repository

```bash
git clone https://github.com/theobiabo/khaos.git
cd khaos
pnpm install
```

`pnpm install` installs Turborepo, Astro, NAPI-rs tooling, and TypeScript dependencies. Cargo downloads Rust dependencies when a Rust command first runs.

## Build the workspace

```bash
pnpm build
```

The root build performs these steps:

1. Build `khaos_core` with Cargo.
2. Build the native Node.js module with NAPI-rs.
3. Compile `packages/typescript`.
4. Build the Rust TUI binary.
5. Check and build the Astro documentation site.

## Run all tests

```bash
pnpm test
```

This validates the Rust pipeline, native binding, TypeScript wrapper, and documentation application.

## Run linting and type checks

```bash
pnpm lint
pnpm typecheck
```

The lint command runs Rust formatting checks, Clippy with warnings denied, TypeScript checks, and Astro checks.

## Format Rust code

```bash
pnpm format
```

## Run the documentation site

```bash
pnpm dev
```

Open the local URL printed by Astro. The content and renderer both live in `apps/doc`.

## Run the Rust TUI

Pass a recorded sample file to the CLI:

```bash
cargo run -p khaos_cli -- path/to/sample.bin
```

Press `g` to generate 32 bytes and `q` or `Esc` to quit. See [Rust TUI](/cli/) for its current behavior and planned source adapters.

## Work on only the Rust core

```bash
cargo test -p khaos_core
cargo clippy -p khaos_core --all-targets -- -D warnings
```

Use this faster loop when changing extraction or pool behavior.

## Work on only the native binding

```bash
pnpm --filter @khaos/native build
```

This generates the local native module and its TypeScript declarations in `crates/khaos_napi`.

## Work on only the TypeScript package

Build the native dependency first, then compile and test the wrapper:

```bash
pnpm --filter @khaos/native build
pnpm --filter @khaos/typescript build
pnpm --filter @khaos/typescript test
```

## Next steps

Continue with [TypeScript usage](/node_bindings/) for the primary integration path, or [Rust usage](/rust_usage/) when you need the core crate directly.
