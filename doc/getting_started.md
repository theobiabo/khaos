---
title: Getting started
description: Build and run the khaos workspace locally.
sidebar:
  order: 1
---

You need Node.js 22.12 or newer, pnpm 9, and the stable Rust toolchain.

## Install the workspace

```bash
git clone https://github.com/theobiabo/khaos.git
cd khaos
pnpm install
```

## Run the tests

```bash
pnpm test
```

This runs the Rust tests first. It then builds the native Node module, compiles the TypeScript package, tests the wrapper, and checks the Astro site.

## Build everything

```bash
pnpm build
```

The build order is handled for you:

1. Cargo builds `khaos_core`.
2. NAPI-rs builds the native Node module.
3. TypeScript compiles the public wrapper.
4. Astro builds the documentation site.

## Run the website

```bash
pnpm dev
```

Open the local address printed by Astro.

## Run only the Rust tests

```bash
cargo test -p khaos_core
```

Start here when changing the extraction or pool logic. These tests are fast and do not require Node.js.

## Format Rust code

```bash
pnpm format
```

## Important limit

The current API accepts bytes that another program has already captured. Webcam, microphone, temperature, and radio drivers are not implemented yet.
