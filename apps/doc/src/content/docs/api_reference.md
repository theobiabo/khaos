---
title: API reference
description: Contracts for the Rust, NAPI-rs, and TypeScript APIs.
sidebar:
  order: 5
---

## Rust functions

### `analyze`

```rust
pub fn analyze(input: &[u8]) -> EntropyReport
```

Inspects raw input without generating bytes. Empty input is accepted and returns zero values.

### `extract`

```rust
pub fn extract(input: &[u8]) -> Result<[u8; 32], EntropyError>
```

Applies Von Neumann unbiasing, packs surviving bits, and returns a 32-byte BLAKE3 digest.

### `generate`

```rust
pub fn generate(input: &[u8], length: usize) -> Result<Vec<u8>, EntropyError>
```

Creates a fresh pool from operating-system randomness, mixes one sample, and returns `length` bytes.

### `generate_deterministic`

```rust
pub fn generate_deterministic(
    seed: &[u8],
    input: &[u8],
    length: usize,
) -> Result<Vec<u8>, EntropyError>
```

Creates a pool from a caller-provided seed. Equal arguments produce equal output.

## `EntropyPool`

### `EntropyPool::from_system`

```rust
pub fn from_system() -> Result<Self, EntropyError>
```

Creates a pool from 32 bytes supplied by the operating system.

### `EntropyPool::from_seed`

```rust
pub fn from_seed(seed: &[u8]) -> Self
```

Creates a deterministic pool from caller-provided bytes.

### `EntropyPool::mix`

```rust
pub fn mix(&mut self, input: &[u8]) -> Result<EntropyReport, EntropyError>
```

Extracts and mixes one sample. The pool state changes when the operation succeeds.

### `EntropyPool::bytes`

```rust
pub fn bytes(&mut self, length: usize) -> Result<Vec<u8>, EntropyError>
```

Generates up to 1,048,576 bytes and then rekeys the pool.

## `EntropyReport`

```rust
pub struct EntropyReport {
    pub input_bytes: usize,
    pub unbiased_bits: usize,
    pub ones_ratio: f64,
}
```

| Field           | Range       | Meaning                                |
| --------------- | ----------- | -------------------------------------- |
| `input_bytes`   | `0..`       | Original sample size                   |
| `unbiased_bits` | `0..`       | Bits retained by Von Neumann unbiasing |
| `ones_ratio`    | `0.0..=1.0` | Fraction of raw bits equal to one      |

## `EntropyError`

| Variant                       | Cause                                             |
| ----------------------------- | ------------------------------------------------- |
| `EmptyInput`                  | Extraction or generation received an empty sample |
| `BiasedInput`                 | No bit pair survived Von Neumann unbiasing        |
| `OutputTooLarge`              | More than 1,048,576 bytes were requested          |
| `SystemRandomnessUnavailable` | The operating system did not provide a seed       |

## TypeScript functions

### `inspectEntropy`

```ts
function inspectEntropy(input: Uint8Array): EntropyReport;
```

Calls the Rust `analyze` function.

### `generateBytes`

```ts
function generateBytes(input: Uint8Array, length?: number): Uint8Array;
```

Calls one-shot generation. `length` defaults to `32`.

### `replayBytes`

```ts
function replayBytes(seed: Uint8Array, input: Uint8Array, length?: number): Uint8Array;
```

Calls deterministic generation. `length` defaults to `32`.

## Native NAPI-rs exports

The native adapter exposes:

- `analyzeEntropy`
- `generateBytes`
- `replayBytes`

Most Node.js users should import the TypeScript package rather than call the generated native module directly.
