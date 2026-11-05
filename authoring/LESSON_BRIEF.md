# Current assignment

## CH12-03 — Parse numbers and groups without dropping source

**Authorization/status:** User approved the text with “ok, looks good, now update rlox”. Text approved; matching implementation and ordered reader route **Ready for combined review** at <http://localhost:4321/chapter-12/12.3-lossless-groups/>. Final acceptance remains with the user. No commit/push or 12.4 authorized.

**Starting state:** Accepted 12.2 at 2cda639, with 79 passing tests, generated kind declarations and tree Language/aliases/token_tree. Confirmed native source matched HEAD and captured arithmetic, token_tree and CLI token baselines before editing.

**Executed route:** Applied module declaration and GroupingExpr kind. Created cst imports/result, appended cursor/builder state, inserted grammar methods and public entry in displayed order. All-target workspace check passed before creating the parse_cst example. Created/ran example and observed complete-group, missing-close, invalid-token suffix and unsupported-infix cases exactly as described. Appended tests, ran formatting, all-target Clippy with warnings denied, all workspace tests and original arithmetic inspector. All three resulting files exactly match complete manuscript views; no hidden edits or manuscript corrections. No manifest/lockfile change.

**Behavior:** cst::parse returns a root plus zero/one located errors for representable source; SourceTooLarge remains separate. Full-token cursor emits trivia into root/group according to the displayed ownership policy. Group nodes close before propagating errors; all remaining tokens are drained into root. Original source reconstructed for valid, empty, malformed and depth-limited cases. Shared 64-wrapper group policy retained. Owned arithmetic parser and token_tree stay unchanged; no unary/infix CST or later recovery implemented.

**Observed checks:** All 83 workspace tests pass locally on macOS ARM64/Rust 1.95.0 (6 app, 6 process, 64 syntax, 7 text), including new group/trivia/parent/range, reconstruction, located-error/suffix and 64/65-depth cases. Complete group diagram checked exactly against native kind/text/range/nesting output using a disposable inspection probe, removed afterward. Both displayed example output blocks and partial suffix observations agree with the native example. Original 35 arithmetic observations, token_tree output and CLI token streams/status match captured baselines. All three complete views match; parser/Expr/tree/app/text/extension/workflow/manifests/lockfile unchanged. Diff whitespace check passes.

**Publication/limits:** Prior text-phase site check/build (42 pages), four references and 125 rendered links/anchors remain applicable because manuscript/navigation/components did not change during implementation. Appearance belongs to the user; no repeated publication build, screenshot or browser visual review. No new platform, hosted CI, minimum compiler or oversized-file end-to-end claim; no new framework, commit/push or later section.

**Next permitted action:** Human combined review of 12.3 and matching code, particularly partial-tree/trivia ownership and retained suffix behavior. Await acceptance and further instruction.

**Acceptance/next assignment:** User accepted the combined endpoint by requesting its commit and 12.4. Commit the accepted pair, then draft only 12.4; no push.
