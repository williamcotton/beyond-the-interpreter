# Accepted Chapter 11 native workspace

Source commit: `3ce175f7ffabe4db7b03503f54b33554e79c1c6c` (`feat: pair section 11.5 with additive parsing`).
Accepted when the user requested committing 11.5 and outlining Chapter 12.

`rlox.tar.gz` contains the native workspace's tracked source, tests, examples,
input files, Cargo manifests/lockfile and ignore rules at that commit.
Editor/client sources and generated/dependency artifacts are excluded.
`CONTENTS.sha256` records each archived member. Archive SHA-256:

```text
2e9e44adfe8abf61956d711a75fbe8405b9681db09efd9fb21b0ddd2ca5228a4  rlox.tar.gz
```

Extract into a separate directory, then run from the extracted `rlox/`:

```sh
cargo test --workspace
cargo run -p rlox-syntax --example expression_shape
cargo run -p rlox-syntax --example parse_expression
```

The accepted endpoint passed formatting, all-target workspace Clippy with
warnings denied and 74 tests on local macOS ARM64/Rust 1.95.0. This archive
preserves that endpoint before Rowan's coordinate/API migration; it makes
no new platform or offline-dependency claim. Full manuscript instructions
remain in Chapter 11. No archive execution or future migration performed
when creating this source preservation artifact; members were verified
byte-for-byte against the accepted commit.
