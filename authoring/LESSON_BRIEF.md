# Current assignment

## CH12-04 — Keep unary operators in the tree

**Authorization/status:** User approved the text with “ok, looks good, now update rlox”. Text approved; matching implementation and ordered reader route **Ready for combined review** at <http://localhost:4321/chapter-12/12.4-lossless-unary/>. Final acceptance remains with the user. No commit/push or 12.5 authorized.

**Starting state:** Accepted 12.3 at 1b7d108, with 83 passing tests. Confirmed native source matched HEAD and captured CST/arithmetic/CLI token outputs before editing.

**Executed route:** Added UnaryExpr kind, renamed expression dispatch to primary/removing its trivia call, then inserted expression delegation and recursive unary. All-target workspace check passed before changing the example. Expanded only its source list, ran it and observed unchanged earlier cases plus the seven described unary cases. Appended the four displayed tests, then ran formatting, all-target Clippy with warnings denied, workspace tests and owned arithmetic inspector. All three resulting files exactly match complete manuscript views; no hidden edits, dependency changes or manuscript corrections.

**Behavior:** Minus token/intervening trivia are retained in UnaryExpr, recursive operands form nested unary/group nodes, and failures close the unary before propagating to the existing first-error suffix drain. Groups and unary share the existing 64-wrapper budget. Public result/error/cursor/group logic, scanner and owned arithmetic parser remain unchanged. No infix CST, checkpoints, further recovery or typed views implemented.

**Observed checks:** All 87 tests pass locally on macOS ARM64/Rust 1.95.0 (6 app, 6 process, 68 syntax, 7 text). New tests verify exact operator/trivia/text/ranges, chain/group operand structure, missing/invalid operands with retained suffixes and mixed group/unary 64/65-depth cases. Both displayed example blocks and both complete/partial unary diagrams exactly match native output; disposable diagram inspector removed afterward. Earlier CST observations are preserved byte-for-byte as the example output prefix; all 35 arithmetic observations and token CLI streams/status match captured baselines. All three complete views match; existing parser/Expr/tree, app/text, manifests/lockfile, extension/workflow unchanged. Diff whitespace check passes.

**Publication/limits:** Prior text-phase site check/build (43 pages), two references and 125 rendered links/anchors remain applicable because manuscript/navigation/components did not change during implementation. Appearance remains with the user; no repeated publication suite, screenshots or browser visual review. No new hosted/other-platform/minimum-compiler/oversized-file verification, new infrastructure, commit/push or later section.

**Next permitted action:** Human combined review of 12.4 and matching code, especially unary trivia ownership and partial-node behavior. Await acceptance and further instruction.

**Acceptance/next assignment:** User accepted the combined endpoint by requesting its commit and 12.5. Commit the accepted pair, then draft only 12.5; no push.
