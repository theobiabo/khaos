---
title: Getting started
description: Clone, install, and verify the Khaos workspace locally.
sidebar:
  order: 1
---

Khaos is currently installed from source. The workspace requires Node.js 22.12 or newer, pnpm 9, and a stable Rust toolchain once the crates are added.

## Clone the repository

```bash
git clone https://github.com/theobiabo/khaos.git
cd khaos
pnpm install
```

## Build the current apps

```bash
pnpm build
```

This runs the Starlight documentation build through Turborepo.

## Run local development

```bash
pnpm dev
```

The documentation site uses Astro's default development port and is served from the root path.

:::note[Package installation]
There is no npm package to install yet. This page will gain package-manager commands when the Node binding is published.
:::
