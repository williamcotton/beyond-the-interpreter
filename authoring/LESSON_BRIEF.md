# Current assignment

## CH12-06 — Keep incomplete and unexpected input inspectable

**Authorization/status:** User approved text with “ok, looks good, now update rlox”. Text approved; matching implementation and ordered reader route **Ready for combined review** at <http://localhost:4321/chapter-12/12.6-incomplete-syntax/>. Final acceptance remains with the user. No commit/push or 12.7 authorized.

**Starting state:** Accepted 12.5 at d93a6c0, with 92 passing tests. Confirmed native source matched HEAD and captured owned arithmetic, CLI token and token_tree baselines before editing.

**Executed route:** Applied displayed MissingExpr kind, private error storage/initialization, missing leaf, recovering unary, primary invariant, group closer helper/arm and public root entry in order. All-target workspace check passed before expanding the example source list. Ran example and matched missing-left/right trees and multi-error observations. Applied required old-test migration, appended new tests, then ran formatting, all-target Clippy with warnings denied, workspace tests and owned arithmetic inspector. All three resulting files exactly match complete listings. No hidden edits, dependency changes or manuscript corrections.

**Behavior:** Zero-width MissingExpr leaves preserve operand roles without fabricated text. Ordinary operand/group/root errors accumulate with original source retained; operators and closers stay available to their callers. Resource failures stop grammar recovery and retain the suffix without additional syntax diagnostics. Binary checkpoint/height loops and valid-input tree/trivia contracts remain unchanged.

**Observed checks:** All 96 workspace tests pass locally on macOS ARM64/Rust 1.95.0 (6 app, 6 process, 77 syntax, 7 text). New/migrated tests verify left/right slots, zero-width leaves, group/root delimiter ownership, multiple lexical errors, malformed-input progress, source conservation and resource exits; prior 64/65 nesting/height tests still pass. Both displayed missing-operand trees and multi-error output agree with native example. Disposable native inspector verified all four recovery fixtures (39 CST entries and five diagnostics) plus all four prior arithmetic fixtures (66 entries), including exact text/kinds/parents/depths/ranges; removed afterward. Fixture provenance comment updated; data/controls unchanged. All 35 owned arithmetic observations, CLI token streams/status and token_tree output match baselines. Three complete views, unchanged binary loops and protected source/dependency/workflow comparisons pass; diff whitespace check passes.

**Publication/limits:** Prior site check/build (45 pages), references, 206 local links and three-mode/116-selection hook-state/SSR checks remain applicable because manuscript/navigation/controls did not change during implementation. Appearance remains with user; no repeated publication suite, screenshots or browser visual/hydration review. No new hosted/minimum-compiler/other-platform/oversized-file claim, framework, commit/push or later section.

**Next permitted action:** Human combined review of 12.6 and matching code, particularly recovered operand slots, delimiter ownership and resource-stop behavior. Await acceptance and further instruction.

**Acceptance/next assignment:** User accepted the combined endpoint by requesting its commit and 12.7. Commit the accepted pair, then draft only 12.7; no push.
