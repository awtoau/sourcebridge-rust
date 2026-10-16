# Sourcebridge Rust backend pack

This pack emits Rust artifacts from Sourcebridge source facts. The initial
Stage-1 emitter accepts only the Delphi scalar-addition fixture facts validated
under Free Pascal 3.2.3, x86_64 Linux, Delphi mode:

- Delphi `Integer` maps to Rust `i32`;
- Delphi `Cardinal` maps to Rust `u32`;
- function bodies must be an addition of their two declared parameters;
- the pinned FPC variant's tested function-result overflow behavior maps to
  `wrapping_add`.

Unsupported facts are rejected rather than guessed. This pack does not claim
Embarcadero Delphi ABI or runtime equivalence.

The input facts must include the verified source variant. The initial emitter
accepts only Free Pascal 3.2.3, x86_64 Linux, Delphi mode; it refuses otherwise
identical type names from an unverified compiler or target.

## Verified C scalar path

The pack also has a separate, native C-to-Rust proof for the
`sourcebridge.c.ast-facts.v1` facts emitted by `sourcebridge-c` from the
pinned `c-testsuite` corpus. It accepts only Clang 22.1.8,
`x86_64-redhat-linux-gnu`, C11, O0 facts for one `int main` function whose
literal `+`, `-`, and `*` expression is proven not to overflow. It emits a
Rust process-exit program.

This path intentionally rejects overflow-sensitive C expressions until a
profile records the source compiler's overflow policy. It is native
Sourcebridge lowering, not c2rust output and not a claim of general C support.

```text
cargo run --bin sourcebridge-rust-emit -- path/to/scalars.facts.json
```
