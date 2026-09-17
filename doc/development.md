---
title: Development guide
description: Find the code, run focused checks, and add new functionality without crossing boundaries.
sidebar:
  order: 7
---

## Repository map

| Path                  | Responsibility                                            |
| --------------------- | --------------------------------------------------------- |
| `crates/khaos_core`   | Entropy extraction, reporting, pool state, and generation |
| `crates/khaos_napi`   | Conversion between Node.js values and Rust values         |
| `packages/typescript` | TypeScript names, defaults, and public package surface    |
| `apps/web`            | Astro and Starlight renderer                              |
| `doc`                 | Documentation content                                     |

## Rust core modules

| Path                        | Responsibility                                                  |
| --------------------------- | --------------------------------------------------------------- |
| `src/entropy/error.rs`      | Public error variants and messages                              |
| `src/entropy/extractor.rs`  | Analysis, Von Neumann unbiasing, packing, and BLAKE3 extraction |
| `src/entropy/pool.rs`       | System seeding, mixing, ChaCha20 generation, and rekeying       |
| `src/entropy/report.rs`     | Sample report type                                              |
| `tests/entropy_pipeline.rs` | Public behavior tests                                           |

## Run focused checks

```bash
cargo fmt --all --check
cargo clippy -p khaos_core --all-targets -- -D warnings
cargo test -p khaos_core
```

## Add a physical input driver

A driver should capture data and return bytes. It should not duplicate extraction or pool behavior.

A future driver should define:

- How samples are captured
- The sample format
- The sampling interval
- Health checks
- Failure behavior
- Privacy implications
- Platform support

Pass accepted samples into `EntropyPool::mix`.

## Add a new binding

Keep bindings thin:

1. Validate language-specific arguments.
2. Convert them into Rust values.
3. Call `khaos_core`.
4. Convert the result or error back.

Do not implement a second entropy pipeline in the binding. Follow [Add a language binding](/binding_guide/) for API naming, byte semantics, errors, lifecycle decisions, tests, and release requirements.

## Change the entropy pipeline

When changing extraction or generation:

1. Add or update a public behavior test.
2. Run Rust formatting and Clippy.
3. Run the full workspace tests.
4. Update `doc/architecture.md`.
5. Update `doc/api_reference.md` if the contract changed.
6. Describe security implications in the pull request.

## Documentation

All documentation lives in `doc/`. Astro loads this directory through `apps/web/src/content.config.ts`.

Run the local site with:

```bash
pnpm dev
```

Run a production documentation check with:

```bash
pnpm --filter @khaos/web build
```
