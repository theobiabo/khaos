---
title: How it works
description: Follow a physical sample through every stage of the khaos entropy pipeline.
sidebar:
  order: 2
---

khaos keeps entropy processing in Rust. NAPI-rs and TypeScript only adapt that core API for Node.js.

```text
physical observation
        ↓
raw input bytes
        ↓
Von Neumann unbiasing
        ↓
BLAKE3 extraction
        ↓
keyed pool mixing
        ↓
ChaCha20 generation
        ↓
pool rekeying
        ↓
output bytes
```

## Physical observation

A changing real-world process can be sampled as bytes. Examples include image frames, audio buffers, timing measurements, and sensor readings.

The current library does not capture these sources. It accepts a byte slice supplied by your application. Keeping capture separate from extraction lets different hardware drivers use the same cryptographic pipeline.

## Raw input bytes

The input can have any positive length, but it must contain at least one changing bit pair after being split into pairs. Empty input returns `EntropyError::EmptyInput`.

A larger sample does not automatically mean a better sample. The important question is how difficult the physical process is for an attacker to observe, control, or predict.

## Von Neumann unbiasing

A physical source may produce more zeroes than ones. The Von Neumann method processes two bits at a time:

| Input pair | Output    |
| ---------- | --------- |
| `00`       | discarded |
| `11`       | discarded |
| `01`       | `0`       |
| `10`       | `1`       |

Matching pairs are discarded. Different pairs become one output bit. If every pair is discarded, khaos returns `EntropyError::BiasedInput`.

This method addresses a simple independent bias. It does not make a predictable source unpredictable and does not defend against every form of correlation.

## Packing the surviving bits

The remaining bits are packed into bytes. The final byte may contain fewer than eight meaningful bits. khaos also records the original input length and surviving bit count in the extraction input so different sample shapes are separated.

## BLAKE3 extraction

BLAKE3 compresses the packed bits into a fixed 32-byte digest. The digest is convenient to mix into a pool regardless of the original sample size.

Hashing does not create entropy. If an attacker knows the input, the attacker can calculate the same digest.

## Pool initialization

`EntropyPool::from_system()` asks the operating system for 32 random bytes through `getrandom`. Those bytes are hashed with a khaos-specific domain label to form the initial pool key.

The physical input is therefore an additional contribution. It does not replace the operating-system source.

`EntropyPool::from_seed()` uses a caller-provided seed instead. This constructor is deterministic and intended for replay and testing.

## Pool mixing

`EntropyPool::mix()` extracts the physical sample and uses keyed BLAKE3 to combine:

- The current private pool key
- A domain label
- The current generation counter
- The extracted 32-byte digest

The result replaces the previous pool key. The generation counter is then increased.

The method returns an `EntropyReport` describing the original sample.

## ChaCha20 generation

`EntropyPool::bytes()` derives a 12-byte nonce from the private key and generation counter. ChaCha20 then fills a zeroed output buffer with its keystream.

A single request can return at most 1,048,576 bytes. Larger requests return `EntropyError::OutputTooLarge`.

## Rekeying

After generation, keyed BLAKE3 combines the current key, generation counter, and generated output. That result becomes the next key.

Rekeying prevents the same pool state from being reused for the next request. The generation counter is increased again.

## One-shot mode

`generate(input, length)` creates a new pool from operating-system randomness, mixes one sample, and returns bytes. Every call creates a fresh pool.

Use it when you have one sample and one output request.

## Stateful mode

Create an `EntropyPool` directly when samples arrive over time. You can call `mix()` repeatedly and call `bytes()` whenever output is needed.

This mode matches a future camera, microphone, or sensor service more closely because the pool evolves while the process runs.

## Deterministic replay

`generate_deterministic(seed, input, length)` creates the pool from a known seed rather than the operating system. The same seed, input, and length return the same bytes.

Replay mode is useful for:

- Repeatable tests
- Educational demonstrations
- Debugging a recorded sample
- Explaining each transformation

Replay mode is predictable and must not be used to generate secrets.

## Workspace boundaries

```text
khaos/
├── apps/
│   └── doc/                         Astro renderer and documentation content
├── crates/
│   ├── khaos_core/                  Entropy pipeline
│   ├── khaos_cli/                   Rust TUI
│   └── khaos_napi/                  Node.js adapter
└── packages/
    └── typescript/                  TypeScript-facing API
```

The dependency direction is one-way:

```text
TypeScript → NAPI-rs → Rust core
Rust TUI → Rust core
Astro → local documentation content
```

The binding does not reimplement cryptographic behavior. That keeps one source of truth for the entropy pipeline.
