# Current assignment

## CH12-01 — Use Rowan-compatible text coordinates

**Authorization:** User approved 12.1's text with “ok, looks good, now update rlox”, authorizing execution of its reader route. No commit, push or 12.2 authorized.

**Status:** Text approved; matching implementation and ordered reader route **Ready for combined review** at <http://localhost:4321/chapter-12/12.1-rowan-coordinates/>. Final acceptance remains with the user.

**Starting state/preservation:** Accepted Chapter 11 at 3ce175f, with 74 tests. Its nineteen native source/manifests/lockfile/input files are preserved under [checkpoints/chapter-11](checkpoints/chapter-11/README.md), independently of Git. Archive bytes were not changed by implementation.

**Executed route:** Applied text dependency/production definitions/test patch, scanner import/function/test patch, parser import/failure enum/public entry/private constructor/spelling/EOF/test patch, example patch, app dependency/main patch in displayed order. Finished the atomic cross-crate edit group before compiling. All seven files exactly match the manuscript's complete resulting listings. Cargo generated the documented lockfile additions; existing registry packages remain unchanged. No hidden source edits or manuscript corrections required.

**Result:** Text re-exports Rowan 0.17.0 coordinates and supplies checked_size, checked_range and safe slice. Scanner rejects unrepresentable source lengths before lexing. Public ParseFailure distinguishes SourceTooLarge from located Syntax(ParseError); grammar methods retain their original behavior. App handles fallible scanning and uses its direct text dependency. Every test/example caller is migrated. No grammar, AST, tree-builder, crate boundary, extension or workflow change.

**Observed checks:** Executed the documented cargo check --workspace --all-targets, cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --workspace, tokens hello.lox and parse_expression example. All pass locally on macOS ARM64/Rust 1.95.0. Tests: 75 (6 app, 6 process, 56 syntax, 7 text), including maximum/overflow integer conversion, UTF-8 boundaries, empty/out-of-bounds/reversed ranges and existing arithmetic depth/precedence/error cases. Captured Chapter 11 outputs before editing: token/source/manual example output and statuses, all 35 parsed-source observations, and malformed UTF-8 lexical listing/stderr/status match byte-for-byte afterward. Both displayed parser output examples agree with native observations. Seven complete listings and generated lockfile changes verified; diff whitespace check passes.

**Publication/limits:** Prior text-phase site check/build (40 pages), five official references and local routes/anchors remain applicable because the manuscript/navigation did not change during implementation. Appearance belongs to the user; no screenshot/browser visual review. Oversized conversion is exercised without allocating a multi-gigabyte string; oversized-file end-to-end execution, other platforms, hosted CI and minimum compiler compatibility are not verified. No repeated publication build, new validation framework, commit/push or 12.2 work.

**Next permitted action:** Human combined review of 12.1 and matching rlox. Await acceptance and further instruction; do not advance while corrections remain unresolved.

**Acceptance:** User accepted the combined endpoint by requesting its commit and section 12.2. Commit the accepted pair, then draft only 12.2. No push.
