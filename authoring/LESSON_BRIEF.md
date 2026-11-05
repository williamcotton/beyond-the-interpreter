# Current assignment

## CH12-02 — Keep every token in a small syntax tree

**Authorization/status:** User approved text and requested implementation, then accepted the combined endpoint with “ok great, commit and let’s move on to section 12.3”. Commit the accepted pair, then draft 12.3 text only. No push authorized.

**Executed reader route:** Started from ec73032. Applied dependency, macro, existing enum opening/end, conversion test and module declaration in displayed order. Created tree language/aliases then appended token builder; all-target check passed. Created/ran token_tree example; output matched the manuscript. Appended tree tests, ran formatting, all-target Clippy with warnings denied, all workspace tests and existing parser inspector. All four resulting files exactly match their complete listings; no hidden edits or manuscript corrections. Cargo.lock gained only syntax's existing Rowan dependency edge.

**Observed checks:** All 79 tests pass locally on macOS ARM64/Rust 1.95.0 (6 app, 6 process, 60 syntax, 7 text). Kind round trips/unknown rejection, source reconstruction including empty/error/Unicode input, parent links and trivia positions pass. All 35 original arithmetic observations and CLI tokens streams/status match captured 12.1 baseline. A disposable native inspection probe verified both comparison AST values and all eleven tree entries/parents/depths/text/ranges, then was removed. Component provenance comment updated; data and behavior unchanged. Four complete listings match; protected parser/AST/text/app/extension/workflow files unchanged. Diff whitespace check passes.

**Limits/publication:** Prior text checks/site build/references/navigation remain applicable because manuscript and component behavior did not change. No repeated site suite, screenshots/browser visual or hydration review, other-platform/hosted CI or minimum-compiler verification. No new framework or later implementation. Final acceptance was supplied by the user's commit/12.3 instruction.
