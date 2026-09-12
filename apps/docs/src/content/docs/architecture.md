---
title: Architecture
description: The intended boundaries of the Khaos monorepo.
sidebar:
  order: 2
---

Khaos uses one pnpm workspace and one Cargo workspace. Turborepo coordinates JavaScript-facing tasks; Cargo owns Rust compilation.

```text
khaos/
├── apps/
│   └── docs/                Astro Starlight documentation site
├── packages/
│   └── typescript/          JavaScript-facing types and package surface
└── crates/
    ├── khaos-core/          Entropy logic and policy
    └── khaos-napi/          NAPI-rs adapter for Node.js
```

## Boundary rules

1. `khaos-core` should remain independent of Node.js and frontend concerns.
2. `khaos-napi` should translate values and errors without reimplementing entropy policy.
3. `packages/typescript` should provide the narrow public JavaScript surface.
4. Apps should consume packages; packages should not depend on apps.

These are architectural constraints for the implementation phase. The corresponding crates and package directories are reserved by the workspace configuration but are not implemented yet.
