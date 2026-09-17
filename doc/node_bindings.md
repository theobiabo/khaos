---
title: Node and TypeScript
description: Use the Rust entropy pipeline from JavaScript.
sidebar:
  order: 3
---

The NAPI-rs crate exports three functions. The TypeScript package gives them names that read naturally in application code.

## Inspect a sample

```ts
import { inspectEntropy } from "@khaos/typescript";

const input = Uint8Array.from([105, 150, 90]);
const report = inspectEntropy(input);

console.log(report.inputBytes);
console.log(report.unbiasedBits);
console.log(report.onesRatio);
```

`onesRatio` is a simple observation, not a security score. A value close to `0.5` does not prove the source is unpredictable.

## Generate bytes

```ts
import { generateBytes } from "@khaos/typescript";

const cameraSample = Uint8Array.from([105, 150, 90]);
const randomBytes = generateBytes(cameraSample, 32);
```

The native module starts its pool with operating-system randomness, mixes the sample, and returns a `Uint8Array`.

## Replay a lesson

```ts
import { replayBytes } from "@khaos/typescript";

const seed = new TextEncoder().encode("lesson-one");
const sample = Uint8Array.from([105, 150, 90]);

const first = replayBytes(seed, sample, 32);
const second = replayBytes(seed, sample, 32);
```

`first` and `second` are equal because replay mode is deterministic.

:::danger[Do not use replay mode for secrets]
A known seed produces predictable output. Replay mode exists for tests, demonstrations, and explanations.
:::
