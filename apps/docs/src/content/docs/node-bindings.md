---
title: Node bindings
description: The planned NAPI-rs boundary between Khaos Rust code and Node.js.
sidebar:
  order: 3
---

The Node binding is planned, not published. It will use NAPI-rs to expose Rust functionality without duplicating entropy behavior in TypeScript.

## Contract goals

- Keep the exported API small enough to audit.
- Validate JavaScript inputs at the boundary.
- Convert Rust failures into stable JavaScript error shapes.
- Avoid exposing internal buffers longer than necessary.
- Document platform support only after CI verifies it.

## What is intentionally absent

There is no placeholder API reference here. Function names, output formats, and package names should be documented only after the underlying crates exist and tests establish their behavior.

For now, use the [architecture overview](../architecture/) to understand where the binding belongs.
