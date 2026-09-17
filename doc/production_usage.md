---
title: Application integration
description: Integrate khaos into a TypeScript service with explicit boundaries, validation, and failure handling.
sidebar:
  order: 4
---

This guide shows how to call khaos from an application shaped like a production service. The structure is suitable for development and evaluation, but khaos itself is still experimental and unaudited.

:::danger[Do not treat this as production approval]
Use operating-system randomness as the security foundation. Do not make khaos the only source for keys, credentials, session identifiers, or other production secrets.
:::

## Integration boundary

Keep four responsibilities separate:

```text
physical source → capture adapter → khaos service → application
```

| Layer           | Responsibility                                                     |
| --------------- | ------------------------------------------------------------------ |
| Physical source | Produces a changing real-world signal                              |
| Capture adapter | Converts a frame, sample, or reading into bytes                    |
| khaos service   | Validates, inspects, and submits those bytes to the native binding |
| Application     | Decides when output is needed and how it is consumed               |

This boundary makes a camera replaceable with a microphone or recorded fixture without changing the entropy pipeline.

## Create an application service

Wrap the package once instead of importing it throughout your application.

```typescript title="src/entropy/khaos_service.ts"
import { generateBytes, inspectEntropy, type EntropyReport } from "@khaos/typescript";

export type EntropyResult = {
  bytes: Uint8Array;
  report: EntropyReport;
};

export class KhaosService {
  generate(sample: Uint8Array, length = 32): EntropyResult {
    this.validateSample(sample);
    this.validateLength(length);

    const report = inspectEntropy(sample);
    const bytes = generateBytes(sample, length);

    return { bytes, report };
  }

  private validateSample(sample: Uint8Array): void {
    if (sample.byteLength === 0) {
      throw new Error("The physical sample is empty");
    }
  }

  private validateLength(length: number): void {
    if (!Number.isSafeInteger(length) || length < 1 || length > 1_048_576) {
      throw new RangeError("Output length must be between 1 and 1,048,576 bytes");
    }
  }
}
```

The wrapper centralizes argument validation and gives the rest of the application one stable interface. Cryptographic work still happens in Rust.

## Capture bytes outside khaos

A capture adapter should only return bytes and source metadata.

```typescript title="src/entropy/capture_adapter.ts"
export type CapturedSample = {
  bytes: Uint8Array;
  capturedAt: Date;
  source: string;
};

export interface CaptureAdapter {
  capture(): Promise<CapturedSample>;
}
```

An application-specific adapter can read a camera frame, audio buffer, sensor packet, or recorded fixture.

```typescript title="src/entropy/recorded_adapter.ts"
import type { CaptureAdapter, CapturedSample } from "./capture_adapter.js";

export class RecordedAdapter implements CaptureAdapter {
  constructor(private readonly fixture: Uint8Array) {}

  async capture(): Promise<CapturedSample> {
    return {
      bytes: this.fixture.slice(),
      capturedAt: new Date(),
      source: "recorded-fixture"
    };
  }
}
```

Recorded input is useful for application tests. It is not a live source and should not be represented as one.

## Compose the service

Create one orchestration function that captures a sample and submits it to khaos.

```typescript title="src/entropy/generate_from_source.ts"
import type { CaptureAdapter } from "./capture_adapter.js";
import { KhaosService, type EntropyResult } from "./khaos_service.js";

export async function generateFromSource(
  adapter: CaptureAdapter,
  service: KhaosService,
  length = 32
): Promise<EntropyResult> {
  const sample = await adapter.capture();
  return service.generate(sample.bytes, length);
}
```

Use dependency injection so tests can supply a recorded adapter while a deployed application supplies a hardware-specific adapter.

## Call it from an HTTP route

The current package runs synchronously because NAPI-rs calls the Rust functions directly. Keep request sizes bounded and do not expose arbitrary output lengths without validation.

```typescript title="src/http/random_route.ts"
import type { CaptureAdapter } from "../entropy/capture_adapter.js";
import { generateFromSource } from "../entropy/generate_from_source.js";
import { KhaosService } from "../entropy/khaos_service.js";

const service = new KhaosService();

export function createRandomHandler(adapter: CaptureAdapter) {
  return async function handleRequest(request: Request): Promise<Response> {
    try {
      const url = new URL(request.url);
      const length = Number(url.searchParams.get("n") ?? "32");
      const result = await generateFromSource(adapter, service, length);

      return Response.json({
        bytes: Buffer.from(result.bytes).toString("base64url"),
        sample: {
          inputBytes: result.report.inputBytes,
          unbiasedBits: result.report.unbiasedBits,
          onesRatio: result.report.onesRatio
        }
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : "Entropy generation failed";
      return Response.json({ error: message }, { status: 422 });
    }
  };
}
```

This is an application boundary, not the planned khaos daemon. Add authentication, rate limiting, request limits, transport security, and deployment-specific observability before exposing any endpoint.

## Handle failures deliberately

A physical source can fail independently of the entropy pipeline. Keep the failure classes distinct.

| Failure                          | Owner                        | Suggested response                                      |
| -------------------------------- | ---------------------------- | ------------------------------------------------------- |
| Camera or microphone unavailable | Capture adapter              | Mark the source unhealthy and stop accepting samples    |
| Empty or unusable sample         | khaos service                | Reject the sample and record a non-sensitive diagnostic |
| Native module unavailable        | Deployment                   | Fail startup and verify the platform artifact           |
| Output request too large         | HTTP or application boundary | Reject the request before calling khaos                 |
| OS randomness unavailable        | Host environment             | Fail closed and investigate the host                    |

Do not silently reuse an old sample when live capture fails. Repetition can hide source failure and give operators a false picture of source health.

## Protect sensitive material

Treat generated bytes and internal state as sensitive even while experimenting.

- Do not log generated bytes.
- Do not log deterministic replay seeds.
- Do not include raw camera or microphone samples in normal application logs.
- Set explicit retention rules for recorded samples.
- Avoid converting bytes to strings unless a protocol requires an encoding.
- Keep output in `Uint8Array` or `Buffer` form for as long as possible.
- Clear application-owned buffers when their lifetime ends if the host environment permits it.

JavaScript runtimes do not guarantee that every copied buffer is immediately erased from memory. If strict memory control is required, keep the operation inside the Rust boundary and review the implementation for zeroization.

## Keep replay mode isolated

Deterministic replay belongs in tests, examples, and education. Put it behind a separate module so application code cannot select it accidentally.

```typescript title="test/support/replay_fixture.ts"
import { replayBytes } from "@khaos/typescript";

export function createReplayFixture(seed: Uint8Array, sample: Uint8Array, length = 32): Uint8Array {
  if (process.env.NODE_ENV === "production") {
    throw new Error("Deterministic replay is disabled in production mode");
  }

  return replayBytes(seed, sample, length);
}
```

An environment check is a guardrail, not a cryptographic control. Keep replay exports out of production-facing service interfaces.

## Deployment checklist

Before deploying an application that experiments with khaos:

1. Build the NAPI-rs artifact for every supported operating system and CPU architecture.
2. Verify the native module loads during application startup.
3. Bound sample and output sizes before crossing the native boundary.
4. Define source health checks and failure thresholds.
5. Keep physical input additive to operating-system randomness.
6. Disable deterministic replay in deployed application paths.
7. Avoid logging raw samples, generated bytes, or replay seeds.
8. Add rate limits and authentication to any network endpoint.
9. Monitor capture failures separately from extraction failures.
10. Review the [security model](/security/) and document your threat model.

## Current lifecycle limitation

Every TypeScript `generateBytes` call creates a fresh Rust pool, seeds it from the operating system, mixes one sample, returns bytes, and drops the pool.

The stateful `EntropyPool` is not yet exposed through NAPI-rs. A future binding can expose a long-lived pool for repeated sampling, but it must define ownership, concurrency, disposal, and process-fork behavior before that API is considered stable.
