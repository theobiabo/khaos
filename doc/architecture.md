---
title: Architecture
description: Follow physical noise through the khaos entropy pipeline.
sidebar:
  order: 2
---

khaos keeps cryptographic decisions in Rust and treats JavaScript as a thin adapter.

```text
physical signal
      ↓
raw bytes
      ↓
Von Neumann unbiasing
      ↓
BLAKE3 extraction
      ↓
keyed entropy pool
      ↓
ChaCha20 output
```

## The workspace

```text
khaos/
├── apps/
│   └── web/                  Astro landing page and documentation
├── packages/
│   └── typescript/           Friendly TypeScript API
└── crates/
    ├── khaos_core/            Entropy pipeline
    └── khaos_napi/            Node.js bridge
```

## Step 1: accept raw bytes

The core does not need to know whether bytes came from a camera, microphone, temperature sensor, radio, or saved recording. Drivers can be added outside the core and pass their captured bytes into the same pipeline.

## Step 2: remove simple bias

Von Neumann unbiasing reads two bits at a time:

| Pair | Result |
| --- | --- |
| `00` | discard |
| `11` | discard |
| `01` | output `0` |
| `10` | output `1` |

This helps with a source that favors zero or one, but it cannot repair a predictable or malicious source.

## Step 3: extract a digest

BLAKE3 turns the variable amount of surviving data into a 32-byte digest. A digest is not proof that the input had enough entropy. It gives the pool a fixed-size value to mix.

## Step 4: mix the pool

The pool begins with randomness from the operating system. Each physical sample is extracted and keyed into the current pool state. This design means a weak physical sample does not replace the operating system source.

## Step 5: generate bytes

ChaCha20 expands the private pool state into the requested number of bytes. The pool changes its key after each request so later state does not simply reuse the previous stream.

## Deterministic replay

`generate_deterministic` replaces operating-system randomness with a chosen seed. The same seed and same input produce the same result. This is useful for tests and education, but it must not be used when unpredictable output is required.

## Security boundary

Randomness metrics and statistical suites measure output patterns. They do not prove that an attacker cannot predict the output. Security review, source-health tests, memory-hardening decisions, and threat-model documentation are still required before production use.
