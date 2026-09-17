---
title: Node and TypeScript
description: Build and use the native khaos binding from TypeScript.
sidebar:
  order: 4
---

The TypeScript package calls the Rust core through NAPI-rs. Entropy extraction and generation still happen in Rust.

The package is not published to npm yet. The following instructions apply to the repository workspace. For a complete service boundary, error strategy, HTTP example, and deployment checklist, continue with [Application integration](/production_usage/).

## Build the native module

From the repository root:

```bash
pnpm --filter @khaos/native build
pnpm --filter @khaos/typescript build
```

NAPI-rs compiles a native module for your current operating system and CPU architecture. TypeScript then compiles the wrapper in `packages/typescript`.

## Inspect a sample

```ts
import { inspectEntropy } from "@khaos/typescript";

const input = Uint8Array.from([105, 150, 90]);
const report = inspectEntropy(input);

console.log(report.inputBytes);
console.log(report.unbiasedBits);
console.log(report.onesRatio);
```

The returned object has this shape:

```ts
type EntropyReport = {
  inputBytes: number;
  unbiasedBits: number;
  onesRatio: number;
};
```

`onesRatio` describes the raw sample. A value near `0.5` does not prove unpredictability.

## Generate bytes

```ts
import { generateBytes } from "@khaos/typescript";

const cameraSample = Uint8Array.from([105, 150, 90]);
const randomBytes = generateBytes(cameraSample, 32);

console.log(randomBytes);
```

The default output length is 32 bytes:

```ts
const randomBytes = generateBytes(cameraSample);
```

Each call creates a new pool from operating-system randomness, mixes the sample, and returns a `Uint8Array`.

The largest accepted output length is 1,048,576 bytes per call.

## Replay a recorded sample

```ts
import { replayBytes } from "@khaos/typescript";

const encoder = new TextEncoder();
const seed = encoder.encode("lesson-one");
const sample = Uint8Array.from([105, 150, 90]);

const first = replayBytes(seed, sample, 32);
const second = replayBytes(seed, sample, 32);

console.log(Buffer.from(first).equals(Buffer.from(second)));
```

The result is `true` because both calls use the same seed, sample, and length.

:::danger[Do not use replay mode for secrets]
A known seed produces predictable output. Replay mode exists for tests, demonstrations, and debugging.
:::

## Use bytes captured elsewhere

Any API that produces a `Uint8Array` can feed khaos. For example, an image-processing layer can pass selected frame bytes into `generateBytes`.

Keep capture logic separate from entropy processing:

```ts
import { generateBytes, inspectEntropy } from "@khaos/typescript";

export function processPhysicalSample(sample: Uint8Array) {
  const report = inspectEntropy(sample);
  const bytes = generateBytes(sample, 32);

  return { report, bytes };
}
```

This example does not decide whether the physical source is trustworthy. Your application must define source health checks and a threat model.

## Error behavior

NAPI-rs converts Rust failures into JavaScript errors. Catch them normally:

```ts
import { generateBytes } from "@khaos/typescript";

try {
  generateBytes(new Uint8Array(), 32);
} catch (error) {
  console.error(error);
}
```

Errors include empty input, input with no surviving bit pairs, output requests above the limit, and unavailable operating-system randomness.

## Current limitation

The TypeScript wrapper exposes one-shot generation and deterministic replay. It does not yet expose the stateful Rust `EntropyPool` class. Applications that require repeated mixing should currently use the Rust API directly.
