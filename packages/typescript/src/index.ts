import {
  analyzeEntropy,
  generateBytes as generateNativeBytes,
  replayBytes as replayNativeBytes,
  type EntropyReport
} from "../../../crates/khaos_napi/index.js";

export type { EntropyReport };

export function inspectEntropy(input: Uint8Array): EntropyReport {
  return analyzeEntropy(Buffer.from(input));
}

export function generateBytes(input: Uint8Array, length = 32): Uint8Array {
  return generateNativeBytes(Buffer.from(input), length);
}

export function replayBytes(
  seed: Uint8Array,
  input: Uint8Array,
  length = 32
): Uint8Array {
  return replayNativeBytes(Buffer.from(seed), Buffer.from(input), length);
}
