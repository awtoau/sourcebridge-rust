# Sourcebridge Rust backend pack

This pack emits Rust artifacts from Sourcebridge source facts. The initial
Stage-1 emitter accepts only the Delphi scalar-addition fixture facts validated
under Free Pascal 3.2.3, x86_64 Linux, Delphi mode:

- Delphi `Integer` maps to Rust `i32`;
- Delphi `Cardinal` maps to Rust `u32`;
- function bodies must be an addition of their two declared parameters;
- only non-overflow examples are currently validated.

Unsupported facts are rejected rather than guessed. This pack does not claim
Delphi overflow, ABI, or runtime equivalence.

The input facts must include the verified source variant. The initial emitter
accepts only Free Pascal 3.2.3, x86_64 Linux, Delphi mode; it refuses otherwise
identical type names from an unverified compiler or target.

```text
cargo run --bin sourcebridge-rust-emit -- path/to/scalars.facts.json
```
