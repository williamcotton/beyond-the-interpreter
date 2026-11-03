# Current assignment

## CH06-02 — Keep the spaces (implementation phase)

**Status:** Accepted when the user requested committing and moving on to 6.3.

**Authorization:** the user accepted 6.1 and requested committing it before moving on to 6.2. Paired lesson/scanner committed as `63f42ef` (`feat: pair section 6.1 with identifier scanning`). The previously requested user-owned visual confirmation rule is separately committed as `f9268bb`. The user subsequently approved the text and requested updating rlox, authorizing the displayed Rust edits and checks. The latest request accepts 6.2 and authorizes its commit followed by drafting 6.3. No push requested.

**Accepted starting point:** actual 6.1 scanner/coordinate API inspected; identifiers, parentheses, !/!=, =/==, and complete-character Error fallback. Nine syntax/twenty workspace tests previously pass. Source coverage already exists; spaces currently become Error tokens. App/text, scanner signature, manifests/lockfile and dependencies stay unchanged.

**Reader question/example:** how can we retain the spaces between meaningful tokens without calling them errors? Use the source described by the Rust string "a \\t\\r\\n!= b": Identifier [0,1), Whitespace [1,5), BangEqual [5,7), Whitespace [7,8), Identifier [8,9).

**Behavior taught:** add Whitespace; recognize runs of exactly space/tab/CR/LF with an alternative pattern and repeated next_if. Reuse the existing end update; keep every trivia byte in the token stream. Explain that grammar consumers may later ignore trivia without the scanner dropping it.

**New Rust reasoning:** pattern alternatives and matches!; test-only String accumulation with push_str; running-byte-position assertions through existing TextRange constructor/equality/slice. No helper abstraction, coordinate getters, new crate/dependency or scanner API change.

**Reader route:** add enum variant with focused diff; insert complete whitespace arm before identifier arm; append explicit classification and range/reconstruction tests inside existing test module; run syntax tests then format/Clippy/workspace tests. Keep all earlier tests; include complete resulting library separately.

**Evidence proposed:** explicit mixed-input kinds/ranges; out-of-set control/Unicode characters remain Error; a small corpus checks valid nonempty slices, exact adjacency from byte zero, full final byte coverage and byte-exact reconstruction. Classification and partition assertions prove different properties. Empty input yields no tokens. No new property-testing dependency or permanent author validation framework.

**Visual approach:** static annotated token/range listing with visible escapes answers which source bytes whitespace occupies and how ranges join. Existing code-listing presentation suffices; no new React component or interaction. The user supplies visual confirmation; do not capture/analyze screenshots or perform visual browser review. Use source/build/link checks.

**References:** current official matches! and pattern alternatives, string accumulation, and Crafting Interpreters whitespace handling. Check exact URLs/fragments and compatibility; no dependency versions change.

**Excluded:** line comments/slash (6.3), numbers/strings/keywords/Logos (Chapter 7), Unicode whitespace extensions, parser/grammar code, source database, EOF token, CLI changes, new APIs/dependencies, subsequent section drafts and unrelated accepted Rust changes.

**Text-phase checks (before implementation):** site check zero errors/warnings/hints; build succeeds (17 generated pages including 404) with existing optional i18n/404/sitemap and npm user-config notices. Local section HTTP 200. All four manuscript reference URLs/fragments and affected rendered internal links/anchors resolve. Displayed edits applied to accepted source in memory agree with complete resulting file; diff context and rustfmt stdin formatting checked. Static example byte intervals/spellings reconstruct the described source and exactly match the displayed expected tokens. Accepted 6.1 manuscript body is preserved, with next link updated; rlox, workflow, components and styles exactly match HEAD. No Rust files or temporary implementation created/compiled/tested; no new dependencies, interactive component, browser review, screenshots, push, or 6.3 draft. git diff --check passes. User owns visual confirmation. Native behavior and actual ordered command route remain unverified until implementation approval.

**Implementation evidence:** started from actual accepted 6.1; applied enum edit, inserted whitespace arm before identifier arm, then appended the three displayed classification/coverage tests in order. Actual lib.rs exactly matches the complete listing. Ran displayed syntax test, formatting, workspace Clippy with warnings denied, and workspace tests in order: twelve syntax tests and all twenty-three workspace tests pass. Mixed whitespace groups, out-of-set Error handling, nonempty valid UTF-8 slices, adjacency, final coverage and reconstruction all match expectations. Static example byte intervals/spellings/kinds match the now-tested native expected tokens and reproduce the exact source bytes. Manifests/lockfile, app/text, sample/ignore rules and workflow remain unchanged. CLI checks confirm byte-exact source output, help success, usage status 2/stderr and read failure status 1/stderr. No manuscript correction needed. Prior site/source checks remain applicable; no repeated site build, browser review, screenshots, new validation infrastructure, commit, push or hosted run.

**Stopping point:** actual Section 6.2 and matching Rust are Ready for combined review. No unfinished implementation work or known discrepancy remains. The user accepted this endpoint and authorized committing it before drafting 6.3. No push requested.
