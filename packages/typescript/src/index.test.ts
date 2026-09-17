import assert from "node:assert/strict";
import test from "node:test";

import { inspectEntropy, replayBytes } from "./index.js";

const input = Uint8Array.from([0b01101001, 0b10010110, 0b01011010]);

test("deterministic replay returns the same bytes", () => {
  const seed = Buffer.from("lesson-one");
  assert.deepEqual(replayBytes(seed, input), replayBytes(seed, input));
});

test("entropy inspection reports the input size", () => {
  assert.equal(inspectEntropy(input).inputBytes, input.length);
});
