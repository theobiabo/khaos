---
title: Security model
description: Understand what khaos protects, what it does not prove, and where it is safe to experiment.
sidebar:
  order: 6
---

khaos is an educational and experimental cryptography project. It has not received an independent security audit.

## Intended model

Normal generation starts with randomness supplied by the operating system. A physical sample is extracted and mixed into that private state. Physical noise is an additional contribution, not a replacement for the system CSPRNG.

The design aims to preserve useful output when a physical source is weak. It does not claim to make a compromised operating system safe.

## What the pipeline provides

- Removal of simple independent bit bias with Von Neumann unbiasing
- Fixed-size extraction with BLAKE3
- Keyed mixing of new samples into existing state
- ChaCha20 output generation
- A changing generation counter
- Rekeying after output
- A deterministic mode for reproducible education and tests

## What the pipeline does not prove

- That a sample contains a specific amount of entropy
- That a camera or microphone cannot be controlled by an attacker
- That a source is independent between samples
- That passing a statistical suite makes output cryptographically secure
- That pool state cannot leak through the host process
- That the implementation is free of side channels
- That deterministic replay is unpredictable

## Threats to physical sources

A camera source can be influenced by fixed images, controlled lighting, compression, auto-exposure, frame duplication, or a compromised driver.

An audio source can be influenced by injected sound, automatic gain control, digital silence, repeated buffers, or a compromised driver.

Timing and temperature sources can be influenced by workload control, virtualization, sensor precision, and environmental control.

Every driver should define health checks appropriate to its hardware.

## Metrics are observations

`ones_ratio` reports how many raw bits are one. `unbiased_bits` reports how many bits survived one unbiasing method.

Neither value is an entropy estimate. A predictable sequence can have a perfect `0.5` ones ratio.

## Statistical test suites

Dieharder and NIST SP 800-22 can identify suspicious statistical patterns. They cannot establish that an adversary is unable to predict the stream.

Future integrations should report their results as diagnostics, not security certificates.

## Deterministic mode

`EntropyPool::from_seed`, `generate_deterministic`, and `replayBytes` are intentionally reproducible. Anyone who knows the seed and input can reproduce the output.

Never use deterministic mode for:

- Cryptographic keys
- Password reset tokens
- Session identifiers
- Nonces that require unpredictability
- Authentication secrets
- Production random number generation

## Before production use

A production release would need at least:

1. A written threat model.
2. Independent cryptographic review.
3. Source-specific health tests.
4. Continuous failure handling for hardware inputs.
5. Pool-state memory and process-isolation review.
6. Cross-platform CI for native bindings.
7. Fuzzing and property tests.
8. Versioned file, socket, and network protocols.
9. Clear behavior when physical inputs disappear or become unhealthy.
10. Reproducible release artifacts.

:::danger[Current status]
Do not use khaos as the only entropy source for production secrets.
:::
