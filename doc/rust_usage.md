---
title: Rust usage
description: Analyze samples, generate bytes, and manage an entropy pool in Rust.
sidebar:
  order: 3
---

The Rust API lives in `crates/khaos_core`. The crate is not published, so applications currently use it as a path or Git dependency.

## Add a local path dependency

For another crate inside a nearby project, add this to its `Cargo.toml`:

```toml
[dependencies]
khaos_core = { path = "../khaos/crates/khaos_core" }
```

Adjust the path for your directory layout.

## Generate bytes from one sample

```rust
use khaos_core::generate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = [0b0110_1001, 0b1001_0110, 0b0101_1010];
    let bytes = generate(&sample, 32)?;

    println!("{bytes:02x?}");
    Ok(())
}
```

`generate` performs four actions:

1. Create a pool from operating-system randomness.
2. Extract the supplied sample.
3. Mix the extracted digest into the pool.
4. Return the requested number of ChaCha20 bytes.

## Analyze a sample

```rust
use khaos_core::analyze;

fn main() {
    let sample = [0b0110_1001, 0b1001_0110, 0b0101_1010];
    let report = analyze(&sample);

    println!("input bytes: {}", report.input_bytes);
    println!("unbiased bits: {}", report.unbiased_bits);
    println!("ones ratio: {}", report.ones_ratio);
}
```

The report contains:

| Field           | Meaning                                            |
| --------------- | -------------------------------------------------- |
| `input_bytes`   | Number of bytes supplied by the caller             |
| `unbiased_bits` | Number of bits that survived Von Neumann unbiasing |
| `ones_ratio`    | Fraction of raw input bits that are `1`            |

These values help inspect a source. They do not measure min-entropy and are not proof of cryptographic safety.

## Extract a digest

```rust
use khaos_core::extract;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = [0b0110_1001, 0b1001_0110, 0b0101_1010];
    let digest = extract(&sample)?;

    println!("{digest:02x?}");
    Ok(())
}
```

`extract` applies Von Neumann unbiasing and BLAKE3 extraction. It returns exactly 32 bytes.

Do not treat the digest as proof that the input contained 256 bits of entropy. Output size and entropy amount are different concepts.

## Keep a stateful pool

Use `EntropyPool` when multiple samples arrive over time:

```rust
use khaos_core::EntropyPool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let first_sample = [0b0110_1001, 0b1001_0110];
    let second_sample = [0b0101_1010, 0b1010_0101];

    let mut pool = EntropyPool::from_system()?;

    let first_report = pool.mix(&first_sample)?;
    let first_output = pool.bytes(32)?;

    let second_report = pool.mix(&second_sample)?;
    let second_output = pool.bytes(32)?;

    println!("{}", first_report.unbiased_bits);
    println!("{}", second_report.unbiased_bits);
    println!("{first_output:02x?}");
    println!("{second_output:02x?}");

    Ok(())
}
```

The pool changes after every mix and generation operation. Keep it private and do not serialize or log its internal state.

## Deterministic replay

```rust
use khaos_core::generate_deterministic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed = b"lesson-one";
    let sample = [0b0110_1001, 0b1001_0110, 0b0101_1010];

    let first = generate_deterministic(seed, &sample, 32)?;
    let second = generate_deterministic(seed, &sample, 32)?;

    assert_eq!(first, second);
    Ok(())
}
```

Use deterministic replay only for tests, examples, and recorded lessons.

## Handle errors directly

```rust
use khaos_core::{generate, EntropyError};

fn main() {
    match generate(&[], 32) {
        Ok(bytes) => println!("{bytes:02x?}"),
        Err(EntropyError::EmptyInput) => println!("capture a sample before generating bytes"),
        Err(error) => println!("khaos failed: {error}"),
    }
}
```

See the [API reference](/api_reference/) for every error variant and function contract.
