# Authoring status

Last updated: 2026-11-03. This file records actual state and decisions, not future assignments. Read the [current brief](LESSON_BRIEF.md) for authorized work and the [roadmap](PROJECT_OUTLINE.md) for chapter coverage.

## Accepted work

- **SITE-01 — Local site and introduction.** Accepted when the user said “ok, looks good” before requesting the initial commits. The introduction is [introduction.mdx](../site/src/content/docs/introduction.mdx), served at <http://localhost:4321/>. It is the accepted orientation/style reference.
- **CH01-01 — Chapter 1, Section 1: A program we can run.** The user approved the text with “this is great, let’s follow the instructions now.” After the implementation report, the user said “I’m happy with what we’ve done so far” and requested this reconciliation. The [section](../site/src/content/docs/chapter-1/1.1-first-program.mdx) and matching [Rust source](../rlox/crates/app/src/main.rs) are accepted as the first executable endpoint. Local page: <http://localhost:4321/chapter-1/1.1-first-program/>.

**Chapter 1 complete:** the user explicitly chose to keep only Section 1.1, omit dedicated Git/README sections, and introduce tests in Chapter 2. The accepted Rust endpoint still has no test cases.

Section 1.1 demonstrates a code introduction, a meaningful one-line edit, complete resulting file views, and an ordered build/run route. Section 5.1 now supplies an accepted scanner/byte-range explanation and first interactive cursor example.

## Actual repository and capabilities

- `site/`: Astro/Starlight publication with MDX, React integration, and strict TypeScript. The introduction and Chapter 1 are accepted. Chapter 1 consists of Section 1.1 only; the proposed Section 1.2 was removed after scope review. Section 2.1 and its matching Rust endpoint are accepted; Section 2.2 and its matching Rust/sample are accepted; Section 2.3 and its matching Rust are accepted. Section 2.4 and its matching Rust/tests are accepted. The Chapter 3 section frame is accepted; Section 3.1 and matching Rust are accepted; Section 3.2 and its matching Clap implementation/tests are accepted. Chapter 4’s working frame is accepted; 4.1 and its matching library are accepted; 4.2 and its matching library/tests are accepted; 4.3 and its matching workspace are accepted; 4.4 and its root workflow are accepted; local and hosted checks pass. Section 5.1 and its matching Rust/cursor visual are accepted. Section 5.2 and its matching scanner/lookahead visual are accepted. Navigation includes only authored pages. Starlight supplies listings and a focused diff; no custom diff viewer or trace framework exists.
- `rlox/`: virtual workspace with `rlox-app` in crates/app, `rlox-text` in crates/text, and `rlox-syntax` in crates/syntax, all edition 2024. The app produces the `rlox` executable and alone depends on Clap 4.6.7/derive. The text library has no external dependencies; the app has no unused text dependency. One root Cargo.lock/target directory, with hello.lox and .gitignore retained at the root. CLI source display/help/usage/read-error behavior is unchanged. The library exposes TextSize, TextRange, FileId, and SyntaxOrigin with safe UTF-8 slicing and caller-owned source identity. Six text tests and five app tests pass; CLI streams/statuses are checked separately. The syntax crate depends on text and exposes a small scanner for parentheses, !/!=, =/==, and one-character Error fallback, with six tests. Lookahead conditionally consumes a following equals sign and preserves unmatched characters. Seventeen workspace tests pass. No identifiers/trivia, Lox execution, or source database exists. Root CI workflow is implemented and passing on GitHub.
- `authoring/`: current roadmap, architecture, review guidance, status, and brief. No fixtures/replay framework or future crate scaffold exists.
- Hosted CI: Ubuntu 24.04 formatting, Clippy, and workspace tests pass (4.4 evidence below). Deployment, release artifacts, distribution, and broader cross-platform coverage are not established.

Installed publication dependencies are locked in [site/package-lock.json](../site/package-lock.json): Astro 7.4.5, Starlight 0.42.8 (including MDX), React integration 7.1.2, React 19.3.1, and TypeScript 6.0.3. Local Node 26.0.0/npm 11.12.1 met their requirements. Rust verification used existing Homebrew rustc/Cargo 1.95.0 on macOS; no exact compiler pin or minimum-version verification is established.

## Authoring decisions

1. **Text first.** Draft and render one section; review its writing and scope; after user approval/authorization, follow those exact instructions to create or update Rust. Fix manuscript bugs exposed by the implementation and recheck the affected route. Deliver the matching section/code for acceptance before advancing. Text approval alone is not verification.
2. **One roadmap.** `PROJECT_OUTLINE.md` preserves the supplied very detailed outline’s 38 chapter titles, order, principal coverage, examples, and checkpoints. The user asked to keep the chapter synopsis basically intact. Chapter order is the working plan, not a standing assignment or fixed section schedule. Earlier suggestions about earlier expression evaluation remain proposals; no reorder is approved.
3. **Human scope review.** Roadmap topics do not require dedicated sections. Chapter 1 closes at Section 1.1; testing moves to Chapter 2 alongside useful behavior. No fixed section counts or line/word quotas. Split, expand, or consolidate based on actual reading experience. The Crafting Interpreters approach is the teaching reference; Rust explanations stay at the point of need.
4. **Gradual architecture.** The initial package remains `rlox`; future internal names become `rlox-*` when workspace extraction is needed and taught. Keep CLI/protocol adapters thin, core diagnostics independent of terminal rendering, and execution independent of Rowan handles. The architecture retains both engines, editor services, LSP, real debugging/DAP, collection, testing, and distribution.
5. **Publishing grows with lessons.** The site is author infrastructure. Existing listing/diff features suffice for Section 1.1. Add custom teaching instruments only for a current explanatory need; no competing JavaScript Lox implementation. Proptest and other conditional tooling require a demonstrated need.
6. **Reconstruct accepted endpoints.** Section 1.1 has complete authored file listings and ordered creation commands; its actual lockfile is retained. Before future edits make that state inaccessible, agree on proportionate source/checkpoint preservation beyond Git history alone.

## Verification already observed

**Publication and manuscript:** clean `npm ci`, `npm run check` (zero diagnostics), and `npm run build` succeeded from `site/`. Both local routes, navigation/anchors, exact displayed source text, and external reading links/fragments were checked. Safari visual inspection covered desktop opening/code/end, light and dark site themes, and a 390 × 844 responsive viewport. This was sampling, not a full accessibility/browser/device audit.

**Rust and reader route:** followed the actual section from an absent `rlox/`: checked tool versions; ran the displayed Cargo creation command; inspected generated manifest/main; applied the greeting edit; ran `cargo run` then `cargo run --quiet`; inspected generated files; created the documented ignore file. Both commands printed the expected greeting. All three authored files matched the manuscript exactly. Direct executable execution matched expected stdout including its newline, empty stderr, and successful exit. Formatting and Clippy passed; `cargo test` passed with zero cases. Target ignore behavior was checked. No manuscript discrepancy was found.

**Limits/notices:** fresh Rust installation, linker-failure paths, Windows/Linux behavior, and the minimum compiler were not executed. The site build reported optional i18n/default-404 content notices and skipped sitemap generation without a deployment URL; fallback 404 built. npm reports an existing user-config `always-auth` warning. None prevented local serving or checks.

These are records of accepted-endpoint checks. The current section’s manuscript checks are recorded separately below.

## Development history

- DOCS-01/DOCS-02 consolidated restart guidance and carried forward pacing, meaningful-diff, first-use-visual, and chronological-reader-route lessons from the old attempt. Earlier implementation/completion claims never transferred. Their document organization is superseded by DOCS-03; they are not active pending assignments.
- Authorized initial commits: `066063b` (root Markdown) and `8b52cb7` (site and ignore rules). Section 1.1, navigation, and its matching Rust endpoint were committed together as `db100b7`. The reconciled guidance was committed separately as `ac27e2e`. Chapter 1 closeout and the accepted Chapter 2 section frame were committed as `b1d38da` before beginning Chapter 2 prose. Nothing pushed or published.

## DOCS-03 — Reconcile guidance and move it into authoring

**Authorization:** the user requested consolidation to reflect the accepted workflow and progress, asked to retain the detailed chapter synopsis, then asked to resolve the multiple outlines and move guidance into `authoring/`.

**Status:** Accepted for commit when the user said “ok, let’s make a few logical commits that pair together related changes” after the reconciliation delivery.

**Changes:** root README/AGENTS are entry points; five authoring documents each own one concern. One roadmap remains. Its operating instructions and stale immediate assignments were replaced by links to their owners. Chapter synopses retain their sequence with six targeted corrections: the actual Chapter 1 creation command; honest Chapter 3 command capabilities; flexible Chapter 12/37 lesson boundaries; conditional handwritten diagnostic rendering; and conditional Proptest. General chapter-planning guidance moved into the review guide.

**Verification:** compared the consolidated synopsis against the supplied original: all 38 chapter titles/order match, 32 chapter blocks are unchanged, the six recorded corrections account for every other synopsis edit, and the optional appendix is preserved. Checked all 33 local Markdown links, final newlines, trailing whitespace, fenced-block balance, a single remaining outline, five authoring documents, and only README/AGENTS as root Markdown entry points. Compared protected Rust, MDX, site configuration/styles, package/lock, and ignore files by SHA-256; all unchanged from the start of DOCS-03. `git diff --check` passed. No runtime or site builds were rerun for this Markdown-only change.

**Scope limits:** Markdown only. No manuscript, website configuration, Rust source, dependencies, or build artifacts changed during the reconciliation. The user subsequently authorized the paired lesson/code commit and a separate documentation commit. No push or publication. The local accepted endpoint is preserved.

## CH01-02 — Removed after text review

The user rejected the proposed greeting-process-test section as offering too little value for the supporting concepts it introduced. The draft and navigation were removed; no Rust test was implemented or executed. The subsequent explicit decision closes Chapter 1 at Section 1.1, omits dedicated Git/README sections, and introduces tests in Chapter 2 with source reading. This assignment is closed, not pending review or implementation.

## Chapter 1 closeout and Chapter 2 transition

**Authorization:** the user asked to retain Section 1.1 as all of Chapter 1, move testing to Chapter 2, remove unnecessary Git/README sections, and update relevant authoring guidance.

**Changes:** removed the unaccepted Section 1.2 draft, restored Section 1.1’s forward navigation to its accepted state, removed the sidebar entry, and updated roadmap/review guidance, brief, and orientation. Chapter 1’s checkpoint now describes the actual greeting endpoint without requiring tests. Chapter 2’s checkpoint includes meaningful tests. Other chapter titles/order and architecture are unchanged.

**Status:** Accepted when the user said “ok, let’s frame out the sections for chapter 2” after the closeout report. Rust remains the accepted Section 1.1 endpoint.

**Checks:** `npm run check` reported zero diagnostics and `npm run build` succeeded with the existing optional-content/sitemap notices. Both remaining pages return HTTP 200, the rejected draft route returns 404, and their rendered navigation has no Section 1.2 links. All 32 local Markdown links resolve. Safari confirms Chapter 1 lists only Section 1.1, with no next-section link. Rust files, the accepted Section 1.1 page, and site configuration match HEAD exactly; no tests directory exists. `git diff --check` passed. No Rust checks rerun because Rust is unchanged. No commits or publication.

**Next planning scope:** the user subsequently requested a provisional section frame for Chapter 2, with small code changes per section. That explicit request is the scope of CH02-PLAN below.

## CH02-PLAN — Chapter 2 section frame

**Authorization:** the user explicitly requested section planning for Chapter 2 and asked not to change too much code per section. This is planning, not authorization to draft or implement the chapter.

**Status:** Accepted as the working frame when the user asked to commit it and create Chapter 2. The roadmap retains four adjustable boundaries: filename selection; source reading; helpful failures and argument policy; unit tests for that policy. Sections may be revised through actual text review; no permanent section count or file/line quota is established.

**Scope rationale:** give source reading an early observable payoff; replace temporary `expect` handling before the chapter endpoint; test actual input rules with unit tests rather than introducing greeting-only process infrastructure. Keep arguments, file I/O, failure behavior, and testing as distinct changes. A source example is the only proposed new non-Rust file; production code stays in the small binary without a crate split or dependency.

**Evidence and limits:** inspected current main/manifest and Chapter 2 roadmap. Reviewed prerequisites, explicit temporary failure behavior/recovery, test scope, and chapter boundaries. No code, MDX, dependencies, runtime behavior, or checks changed. Reader commands, helper signature, and test details remain to be authored and verified through the text-first loop.

## CH02-01 — Give the program a filename

**Authorization:** the user asked to commit the preceding changes, then create Chapter 2. Completed that commit as `b1d38da` first, then began Section 2.1 under the existing one-section-at-a-time workflow. The accepted working section frame is now retained in the roadmap while the brief scopes the current section.

**Status:** Accepted when the user said “looks great, but before we commit” and requested the content-path reorganization. The text, matching Rust, and ordered reader route are complete. The subsequent site reorganization is recorded below.

**Scope:** one MDX page at `site/src/content/docs/chapter-2/2.1-filename.mdx`, Chapter 2 sidebar group, forward/back navigation, and author records. Includes a small main-only change to select/display the filename, explicit temporary missing-input panic, quoted-space and extra-argument cases, and recovery. No file I/O, test, dependency, custom component, or algorithm visual.

**Checks observed:** current official Rust Book command-line, Cargo run, args, nth, and Option::expect documentation checked; both reader reference links resolve. `npm run check` reported zero diagnostics and `npm run build` succeeded with the existing optional-content/sitemap notices. Three local routes return HTTP 200 and navigation/anchors resolve. The complete displayed source matches the MDX text; the focused diff’s before view matches accepted main.rs and its after view matches the complete listing. This was text consistency validation, not Rust execution. Safari inspection sampled the desktop opening/diff/explanation and a 390 × 844 responsive viewport. At text handoff all four Rust files matched HEAD and the proposed commands had not been executed. The subsequent implementation evidence follows below.

**Implementation checks:** followed the manuscript in order: ran the greeting with a filename before editing, applied precisely the displayed main-only edit, ran ordinary, quoted-space, and extra-argument cases, observed the intentional missing-argument panic (exit 101 with `expected a file`), then recovered by supplying the filename. All results matched; no manuscript correction was needed. The unquoted-space prediction also matched (`my`). Independent direct-executable checks confirmed exact successful stdout/newlines, empty stderr, and exit 0; the missing-input case had empty stdout, the expected stderr message, and exit 101. Main matches the complete MDX listing byte for byte. Cargo.toml, Cargo.lock, and .gitignore still match HEAD; hello.lox does not exist and was not needed. Formatting, Clippy with warnings denied, and Cargo test passed (zero cases). Checks used the existing macOS toolchain; no other-platform claim. The manuscript was unchanged, so its prior rendered checks remain applicable.

**Review focus:** the rendered explanation and matching filename-selection behavior, including the deliberately temporary missing-input handling. Chapter 1 remains reconstructible through its complete listings. The section, Rust edit, and author records remain uncommitted; no later section has been started.

## SITE-02 — Numbered chapters and descriptive section paths

**Authorization:** after accepting Section 2.1, the user requested chapter/section-based content paths and renaming index.mdx to introduction.mdx before committing. The user subsequently approved a blend: numbered chapter directories, with section numbers and short descriptions in filenames. The user later accepted the completed reorganization (“great”) and explicitly requested committing all current work. No later lesson is authorized.

**Changes:** content now lives at `introduction.mdx`, `chapter-1/1.1-first-program.mdx`, and `chapter-2/2.1-filename.mdx`. Section URLs match those paths; future filenames follow `chapter-N/N.M-short-description.mdx`. Navigation, manuscript links, README files, and author records point to the new paths. Descriptive page titles remain. The introduction uses the documented Starlight slug override (`slug: index`) to retain the home URL `/`. No Rust or instructional content changed during this reorganization.

**Status:** Accepted when the user said “great, let’s commit all of this now.” Commit closeout is authorized.

**Checks observed:** `npm run check` reported zero diagnostics; `npm run build` generated `/`, `/chapter-1/1.1-first-program/`, and `/chapter-2/2.1-filename/`. All three served routes return HTTP 200, with local links and anchors resolving. Explicit section slugs preserve dots that automatic slug generation would otherwise remove. Source search found no stale title-based or section-1 paths. Safari desktop inspection confirmed the Section 2.1 opening and navigation render correctly. Manuscript comparison confirmed only links and routing frontmatter changed; Rust was unchanged by this reorganization. `git diff --check` passed. Existing optional-content/sitemap and npm user-config notices remain. Nothing committed or published.

## CH02-02 — Read the source

**Authorization:** the user requested moving on to Section 2.2 after the accepted 2.1/site changes were committed. The amended commit is `3fa2dc9`, using the established feat/docs convention now recorded in AGENTS.md.

**Status:** Accepted when the user said “ok great, commit and move on to section 2.3.” Text, Rust, sample, and ordered route are complete. Commit closeout and the next text phase are authorized.

**Scope:** manuscript at `site/src/content/docs/chapter-2/2.2-read-source.mdx`, current-page navigation, and author records. Adds one read statement and source output in main, with one tiny UTF-8 Lox sample. Rust and sample creation followed text approval. Missing-file panic is temporary, with explicit recovery; final input errors and tests remain later work.

**Checks observed for 2.2:** official Rust file-reading/borrowing references and Crafting Interpreters §4.1 checked (including its actual anchor). Site check reported zero diagnostics; build succeeded with the existing optional-content/sitemap notices. All four local routes return HTTP 200; local links and anchors resolve. The focused diff’s before view matches actual main.rs, and its after view matches the complete listing. Safari sampled the desktop explanation/diff and a 390 × 844 viewport. All four Rust files match HEAD; no hello.lox fixture exists. No proposed Rust commands, temporary compilation, or reader-route execution occurred. `git diff --check` passed.

**Implementation evidence:** created hello.lox exactly as displayed; ran the accepted 2.1 executable and observed filename output; applied precisely the main edit; observed the full Lox source output, a missing-file panic with `could not read source file` and exit 101, then successful recovery using hello.lox. No discrepancy required manuscript changes. Independent direct execution confirmed exact stdout/newlines, empty stderr, exit 0 for valid input; empty files, no final newline, UTF-8/CRLF, and spaced paths preserve bytes. Invalid UTF-8 and absent files produce the read panic; absent arguments retain the prior panic. Manifest, lockfile, and ignore file match HEAD. Formatting, Clippy with warnings denied, and Cargo test passed (zero cases). These were local checks, not a new test harness. Other platforms and permission-denied behavior remain unverified. No visual checks repeated for the Rust-only pass.

**Review focus:** the matching section, main, and sample; confirm the file-to-source step and deliberately temporary failure behavior read clearly. No unfinished implementation work remains within 2.2; the user accepted the combined endpoint.

## CH02-03 — Explain input failures

**Authorization:** the user accepted 2.2 and requested a commit followed by Section 2.3. Committed the accepted endpoint and supporting guidance as `9520c43` (`feat: pair section 2.2 with source file reading`) before drafting. The proportional presentation-check rule is included. No push or publication.

**Status:** Accepted when the user said “great, commit and move on to 2.4.” The matching implementation and ordered reader route are verified; commit closeout and the next text phase are authorized.

**Scope:** one manuscript page, navigation, and author records. Implements exactly-one-filename policy in a small borrowed-input function, Option/Result matching, usage and contextual read diagnostics on stderr, and ExitCode from main. Main stays in the same binary; no module, dependency, test, or new fixture. The intermediate endpoint retains the read panic until the next focused edit replaces it. No Rust edits or temporary compilation during text review.

**Visual approach:** existing listings, focused diffs, and terminal contrasts suffice. No new layout, component, or navigation behavior; only another page/link in the existing structure. Following AGENTS.md and the user’s usage preference, no routine browser or screenshot inspection.

**Checks observed for 2.3:** current official ExitCode, iterator collection, Vec/slice conversion, Rust Book slices/match/error handling/stderr, and Crafting Interpreters error-handling references checked. All seven manuscript reference links/fragments resolve. String-based edit validation applies the helper addition and both focused diffs to actual 2.2 and yields the complete final listing; this is source consistency evidence, not compilation or reader-route verification. Site check reported zero diagnostics; build succeeded with the existing optional-content/sitemap notices. Five local pages return HTTP 200 and local links/anchors resolve. All five rlox files match HEAD; no proposed Rust commands or temporary compilation occurred. git diff --check passed.

**Implementation evidence:** executed the displayed route from actual 2.2: observed baseline no-input panic; added the helper; applied the first main edit; ran both usage failures and successful recovery; replaced the read statement; ran the missing-file failure, both usage failures again, and final recovery. All matched. Intermediate direct checks confirmed usage on stderr with exit 1 and empty stdout, while read failure still panicked as labeled. Final direct checks confirmed exact usage stderr/exit 1, contextual read stderr/exit 1 without panic text, empty stdout on failures, and byte-exact successful output/empty stderr/exit 0. Extra-argument validation precedes I/O. Spaced paths, empty/no-final-newline/UTF-8/CRLF source, and invalid UTF-8 were checked with temporary files outside the repository. No manuscript correction was needed. Main matches the complete listing; manifest, lockfile, ignore file, and hello.lox match HEAD. Formatting, Clippy with warnings denied, and Cargo test passed (zero cases). No new test harness or cases added. Verification used the existing macOS toolchain; other platforms and permission-denied behavior remain unverified. No browser/screenshots or unchanged site checks repeated.

**Review focus:** matching text and Rust, especially the helper’s argument rule and useful terminal failure behavior. No unfinished implementation work remains in 2.3; the combined endpoint is accepted.

## CH02-04 — Check the input rules

**Authorization:** the user accepted 2.3 and requested a commit followed by 2.4. Committed the accepted section/code and supporting records as `5d78542` (`feat: pair section 2.3 with input error handling`) before beginning this text phase. Nothing pushed or published.

**Status:** Accepted when the user said “great, commit and move on to 3.1.” Matching tests and the ordered route are verified. The user then clarified that Chapter 3 section planning precedes authoring.

**Scope:** a test-only module in the manuscript, direct tests for one/missing/extra filenames, and an intentional ignored-extra-arguments regression followed by restoration. Current page, navigation, author records, and one test module in main.rs. No production change, dependencies, file fixtures, or process framework. The tests prove argument policy, not file reads or terminal behavior; manual CLI checks remain distinct.

**Visual approach:** existing listings and test diagnostic excerpts suffice; no screenshots or browser automation for routine prose/code edits.

**Checks observed for 2.4:** official Rust testing, test organization, test-running, and assert_eq references checked. Both authored test additions reconstruct the complete listing from actual 2.3; regression/restoration returns to that listing. This validates source consistency only. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. All six pages return HTTP 200, and local links/anchors plus manuscript reference URLs/fragments resolve. All five rlox files match HEAD; no proposed tests executed or compiled. git diff --check passed. No browser/screenshots.

**Implementation evidence:** followed the exact ordered route from accepted 2.3: baseline cargo test had no tests; appended the one-filename test and observed it pass; added the two rejection tests and observed all pass; applied the deliberate is_empty regression and observed only rejects_extra_filenames fail, with left Some("hello.lox") and right None; restored the length guard and observed all three pass. Ran CLI success, usage failure, and successful recovery in order. Independent CLI checks confirmed exact stdout/stderr and exit status. Formatting and Clippy with warnings denied pass. Main matches the complete listing byte for byte; production code, hello.lox, manifest, lockfile, and ignore file match HEAD. No manuscript correction was needed. No repeated site/browser checks; existing macOS toolchain only, not cross-platform verification.

**Review focus:** direct tests and the observed regression/restoration protect the intended filename rule. File-I/O and terminal behavior remain separately checked rather than claimed as unit-test coverage. No unfinished implementation work remains within 2.4; the user accepted the combined endpoint.

## CH03-PLAN — Frame Chapter 3 sections

**Authorization and commit:** the user accepted 2.4 and asked to commit it, then clarified that Chapter 3 planning should precede 3.1. The accepted endpoint was committed; at the user’s correction, the message was amended to `04d4575` (`feat: pair section 2.4 with filename policy tests`). A paired new lesson is a feat even when its Rust changes are tests. No push or publication.

**Status:** Accepted when the user said “great, move on to 3.1.” The roadmap contains a two-part working frame: name the source operation with minimal manual dispatch; replace manual parsing with Clap’s declared interface and generated help. Both carry relevant contract checks; boundaries remain subject to actual writing review.

**Agreed transition:** use `source FILE` to describe the actual capability and replace the bare-filename invocation. Do not claim tokens or execution, add future commands, or build a general hand parser. Source reading stays unchanged. Exact parser types, version/features, test migration, and manifests are not implementation decisions in this planning task.

**Evidence:** inspected actual main/tests, current chapter outline, architecture, and review guidance. Current official Clap derive/Parser documentation supports the intended migration; no version selected or dependency added. Document consistency and git diff whitespace checked. All rlox files match the accepted commit. No MDX, site configuration, runtime, tests, builds, or visual inspection changed during planning.

## CH03-01 — Give the operation a name

**Authorization:** the user approved the Chapter 3 section frame and requested 3.1. The starting point remains `04d4575`. The section frame and this draft are uncommitted; no further commit is authorized.

**Status:** Accepted when the user said “great, commit and move on to 3.2.” Matching implementation and ordered route are verified.

**Scope:** introduce `rlox source FILE` through a small manual command selector, retaining the filename validator and file-read/output behavior. One helper, two main edits, a grouped test import, and direct command-boundary tests. No Clap, dependency, command enum, future commands, or new fixture. The explicitly approved command form replaces the bare-filename invocation. Existing 2.4 complete listings preserve the accepted earlier endpoint.

**Visual approach:** existing source/output listings suffice; no routine browser/screenshots under the user’s proportional-check preference.

**Checks observed for 3.1:** current official slice split_first, Option ?, String::as_str, and Rust Book match documentation checked; all four manuscript reference URLs/fragments resolve. Both focused edits plus helper/test additions reconstruct the complete final listing from actual Chapter 2 source. This checks text consistency, not compilation or reader-route behavior. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. Seven local pages return HTTP 200 and local links/anchors resolve. All five rlox files match HEAD; no proposed Rust commands or temporary compilation. git diff --check passed; no browser/screenshots.

**Implementation evidence:** followed the full displayed route from accepted Chapter 2: observed source-plus-filename rejected by the old program, added the selector, changed main and usage text, observed successful source output, all five usage failures, missing-file diagnostic, and documented recoveries; then updated the test import, added command tests, and ran cargo test. All seven tests pass; formatting and Clippy with warnings denied pass. Main matches the complete listing byte for byte. Direct executable checks verified exact stdout/stderr and exit status for success, missing/unknown/retired command forms, missing/extra filenames, and read failure. Filename validation, existing tests, file-read/output logic, and all other rlox files remain unchanged. No manuscript corrections needed; no repeated site/browser checks. Existing macOS toolchain only; no cross-platform verification.

**Review focus:** matching text/code, the explicit invocation change, and separation of command selection from filename validation. No unfinished implementation work remains within 3.1; the user accepted the combined endpoint.

## CH03-02 — Describe the command with Clap

**Authorization:** the user accepted 3.1 and requested a commit followed by 3.2. Committed the accepted section/code and supporting records as `9d71905` (`feat: pair section 3.1 with source command dispatch`) before drafting. No push or publication.

**Status:** Accepted when the user said “great, commit and move on to outlining the sections for chapter 4.” Matching dependency/code/test migration and ordered route are verified.

**Scope:** declare Cli and Command::Source with Clap derive, retain owned String filename/read/output behavior, replace manual helpers and migrate helper tests to the declared CLI. Include global/command help, typed successful parse, invalid-command cases, usage status 2 versus prior 1, and existing read failure status. No PathBuf, new operation, module/crate split, or fixture.

**Dependency evidence:** current official Clap docs and normalized published Cargo.toml identify 4.6.7, derive support, and Rust 1.85 minimum. This fits the current local toolchain/edition at the metadata level. Crates.io API returned 403; the official docs.rs published manifest supplied the needed metadata. No Cargo command or temporary compilation occurred during text review. The authorized implementation pass subsequently resolved and compiled the declared dependency; evidence follows below.

**Visual approach:** established listings/diffs/terminal excerpts suffice; no routine browser/screenshots.

**Checks observed for 3.2:** manifest/main diffs, CLI declarations, helper removals, and test replacement reconstruct the complete listings from actual 3.1 via string checks only. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. Eight local pages return HTTP 200, local links/anchors resolve, and all six manuscript reference URLs/fragments resolve. All five rlox files match HEAD; no dependency or proposed Rust command executed. git diff --check passed. No browser/screenshots.

**Implementation evidence:** followed the displayed sequence from actual 3.1: old --help failed; applied manifest/import/declaration/main edits; removed superseded helpers/tests; cargo check resolved 21 registry packages and generated Cargo.lock; both help views, source success, all five usage failures, read failure and recovery matched; installed the replacement test module, ran cargo test, and finished with source success. Five tests pass. Formatting and Clippy with warnings denied pass. Main and Cargo.toml exactly match complete listings. Lockfile confirms Clap/builder/derive 4.6.7 and checksums for registry packages. hello.lox and ignore file are unchanged. Direct executable checks confirmed help stdout/empty stderr/status 0, usage empty stdout/stderr/status 2, read empty stdout/contextual stderr/status 1, and byte-exact source stdout/empty stderr/status 0. Library help command and -- separator with a dash-prefixed temporary filename also passed. No manuscript correction was needed. No repeated site/browser checks. Rust/Cargo 1.95.0 Homebrew on macOS; minimum-version, permission-denied paths, and other platforms remain unverified.

**Review focus:** matching section and typed CLI, generated help versus usage/read failures, and retirement of manual parsing/tests. No unfinished implementation work remains within 3.2; the combined endpoint is accepted.

## CH04-PLAN — Frame Chapter 4 sections

**Authorization and commit:** the user accepted 3.2 and requested a commit followed by Chapter 4 section planning. Committed the accepted endpoint and supporting records as `5ddd5a9` (`feat: pair section 3.2 with clap command parsing`). No push or publication.

**Status:** Accepted when the user requested starting 4.1 after reviewing the section frame.

**Proposal:** four adjustable boundaries: safe byte ranges in a small library target within the existing package; file identity and origins; extraction into app/text workspace packages; automated workspace checks. Detailed examples and exclusions are in the roadmap. This preserves chapter coverage without mixing new text algorithms into the filesystem/manifest move. No unused app dependency, new CLI operation, VFS, or future crate scaffolding.

**Evidence and prerequisites:** actual CLI, manifest/lockfile, and tests inspected. At planning time git remote -v was empty; no tracked .github workflow existed. The user subsequently added origin, confirmed below. Current official str::get, Cargo workspace, and GitHub secure-use docs support the proposed direction; exact APIs/action pins remain for authoring. Hosted CI still needs an authorized push and an observed run; adding the remote alone does not supply hosted verification. All rlox files remain identical to the accepted commit. Document consistency and git diff whitespace checked; no new tests, runtime/site builds, browser/screenshots, source, or workflow changes.

## CH04-01 — Locate text with byte ranges

**Authorization:** the user requested starting 4.1, accepting the Chapter 4 frame, and reported adding a remote. Read-only Git inspection confirms origin at https://github.com/williamcotton/beyond-the-interpreter.git. No push or CI run performed or authorized.

**Status:** Accepted when the user said “great, commit and move on to 4.2.” Matching library and ordered reader route are verified.

**Scope:** new library target at rlox/src/lib.rs: TextSize, private-field TextRange with ordering validation, safe str::get slicing, ASCII/reversed/UTF-8/bounds/empty-range tests. CLI and all existing package files remain unchanged. No new package, dependency, FileId, workspace, workflow, or fixture.

**Visual:** static UTF-8 byte-boundary diagram for aé! appears at its first explanation. All data is described in prose; no custom interaction or routine browser/screenshots.

**Checks observed for 4.1:** current official library-target/method/tuple-struct/str::get references checked; all four reader reference URLs/fragments resolve. Initial and added source blocks reconstruct the complete listing as text. UTF-8 fixture bytes and diagram columns checked; corrected a single spacing error in the character row. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. Nine local pages return HTTP 200; local links and anchors resolve. All five rlox files match HEAD and lib.rs does not exist. No proposed Rust commands, temporary implementation, or compilation. git diff --check passed; no browser/screenshots.

**Implementation evidence:** followed the displayed route in order: created lib.rs from the initial listing, added slice and initial tests, ran cargo test --lib; added the UTF-8 test and reran; added bounds/empty-range checks and reran; then ran full cargo test and the CLI. All stages matched the text: four final library tests and five binary tests pass; doc-tests contain no cases. Formatting and Clippy with warnings denied pass. lib.rs matches the complete listing byte for byte; main, manifest, lockfile, fixture, and ignore file match HEAD. Independent CLI check confirmed exact source stdout, empty stderr, and exit 0. No manuscript correction needed; no repeated site/browser checks. Existing macOS toolchain only; no minimum-version or cross-platform verification.

**Review focus:** matching text/library, the two levels of range validation, and unchanged CLI behavior. No unfinished implementation work remains within 4.1; the combined endpoint is accepted.

## CH04-02 — Identify the source file

**Authorization and commit:** the user accepted 4.1 and requested committing it before 4.2. Committed the accepted section/library and supporting records as `123b100` (`feat: pair section 4.1 with source byte ranges`). No push or publication.

**Status:** Accepted when the user said “great, commit and move on to 4.3.” Matching library and ordered route are verified.

**Scope:** FileId and SyntaxOrigin, equality across source identities, and an explicit two-source selection in a test before slicing. New page/navigation/records and additive types/tests in lib.rs; existing range implementation and CLI remain unchanged. No VFS, document database, global ID assignment, paths, new CLI behavior, fixtures, dependencies, or workspace. Existing range logic/tests remain unchanged.

**Visual approach:** two source listings and explicit lookup code suffice; no routine browser/screenshots.

**Checks observed for 4.2:** current official struct/pattern/assert_ne references and Crafting Interpreters location-information section checked; all four manuscript reference URLs/fragments resolve. Additions/import edit reconstruct the complete source listing from actual 4.1 as text; example byte ranges select the stated words. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. Ten local routes return HTTP 200 and local links/anchors resolve. All six rlox files match HEAD; no proposed Rust executed or compiled. git diff --check passed; no browser/screenshots.

**Implementation evidence:** followed the exact ordered route from 4.1: added FileId/SyntaxOrigin, updated the test import, appended the identity test, and ran cargo test --lib successfully; appended the explicit source-selection test and reran library/full tests, then ran the CLI. Six library tests and five CLI tests pass; formatting and Clippy with warnings denied pass. lib.rs matches the complete listing byte for byte. main, manifest/lockfile, fixture, and ignore file match HEAD. Independent CLI check confirmed byte-exact source stdout, empty stderr, and exit 0. No manuscript correction was needed. No repeated site/browser checks. Existing macOS toolchain only; no cross-platform verification.

**Review focus:** matching text/library, file identity versus numeric offsets, and explicit caller-owned lookup. No unfinished implementation work remains within 4.2; the combined endpoint is accepted.

## CH04-03 — Extract the workspace

**Authorization and commit:** the user accepted 4.2 and requested committing it before 4.3. Committed accepted section/library and supporting records as `7ca9e6f` (`feat: pair section 4.2 with source file origins`). No push or publication.

**Status:** Accepted when the user said “great, commit and move on to 4.4.” Workspace migration and ordered route are verified.

**Scope:** unchanged source moves into app/text packages; three complete manifests with virtual workspace resolver 3 and explicit rlox binary target. Keep root fixture/ignore/lockfile and generated target directory; teach root/package/target selection and workspace checks. Clap moves unchanged to app; text has no external dependency and app adds no unused text dependency. No Rust/API/test behavior change, new crate beyond agreed app/text, CI, or future scaffolding. Source references retain complete accepted contents.

**Visual approach:** static resulting file tree and folded full-source reference views; no routine browser/screenshots. Current official Cargo workspace/run/target/resolver docs checked before drafting. Exact migration/lock behavior remains provisional until implementation authorization.

**Checks observed for 4.3:** current official Cargo workspace/run/target/resolver docs checked; all four reference URLs/fragments resolve. Three TOML manifests parse and declare the intended members/target/dependencies; full moved source listings exactly match actual 4.2. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. Eleven local routes return HTTP 200 and local links/anchors resolve. All six rlox files match HEAD; no crates directory, source moves, proposed Cargo command, lockfile edit, or temporary workspace created. git diff --check passed; no browser/screenshots.

**Implementation evidence:** followed the full route from actual 4.2: baseline tests/CLI, exact mkdir/mv/rmdir commands, all three manifest edits, workspace check, implicit/explicit root runs, help, per-package tests, workspace tests, formatting/Clippy, missing-file failure and recovery. All matched. Six text and five app tests pass; formatting and Clippy with warnings denied pass. All manifests/source match complete listings. Both moved sources are byte-identical to the accepted originals. Cargo.lock changes only rlox to rlox-app plus a local rlox-text record; all registry package entries remain identical. Metadata confirms two default members, shared root target, rlox binary/rlox_text library names, and dependency boundaries. No nested locks; fixture/ignore unchanged. Direct execution confirms exact source stdout/empty stderr/exit 0, help stdout/empty stderr/exit 0, and read empty stdout/contextual stderr/exit 1. No manuscript correction needed. No repeated site/browser checks. macOS toolchain only; no other-platform verification.

**Review focus:** matching workspace/text, preserved behavior and source contents, package/target names, and the shared-root route. No unfinished implementation work remains within 4.3; the combined endpoint is accepted.

## CH04-04 — Run the checks automatically

**Authorization and commit:** the user accepted 4.3 and requested committing it before 4.4. Committed accepted workspace/section and supporting records as `13c8e8e` (`feat: pair section 4.3 with app and text workspace`). No push or publication.

**Status:** Accepted when the user said “great, commit and move on to outlining the sections for chapter 5.” Root workflow and hosted verification passed after the explicitly authorized push.

**Agreed root distinction:** the user identified that readers need not mirror the book repository’s parent directory and approved treating the reader’s Rust workspace as their Git root. Reader YAML lives at their `.github/workflows/ci.yml`, alongside Cargo.toml/crates/, with commands at that root. The user confirmed our authoring workflow instead belongs at beyond-the-interpreter/.github/workflows/ci.yml. Its sole layout adaptation is `jobs.checks.defaults.run.working-directory: rlox`, documented in the brief. Do not move up a directory in the reader route or place our active workflow under rlox/.github.

**Scope:** one Ubuntu job for checkout, stable Rust setup, formatting, workspace Clippy and tests. Existing Rust/workspace/CLI unchanged. No matrix/cache/deployment/artifact or extra scripts. GitHub prerequisites, source/workflow commit and push, and observed run/log inspection are taught at need.

**Current references:** official checkout v7.0.1 release/tag verified at commit 3d3c42e5aac5ba805825da76410c181273ba90b1 using official repository refs; current Ubuntu 24.04 image documents rustup. Workflow syntax, minimal profile, secure-use guidance, repository setup and run logs checked. No historical action pin reused.

**Current authorization:** the user explicitly approved updating the codebase, pushing to origin, and using CLI access to inspect CI. This authorizes the concrete section/workflow commit and hosted verification. Local default-toolchain changes, deployment, and Chapter 5 remain excluded.

**Text-phase checks observed for 4.4:** reader YAML parses with the existing site YAML library; required triggers, read permission, verified checkout pin/input, runner, root execution, setup and exact three Cargo checks confirmed. All seven reference URLs/fragments resolve. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. Twelve local routes return HTTP 200 and local links/anchors resolve. All tracked rlox files match HEAD; no root/nested actual workflow, proposed Cargo/setup/publication execution, or hosted run. git diff --check passed; no browser/screenshots.

**Review focus:** reader Rust-root placement versus our explicitly agreed authoring-root adaptation, job setup/step explanations, and the local-to-hosted verification route. The user accepted the combined endpoint.

**Local implementation evidence:** root workflow created; parsed structure equals reader YAML after removing the sole agreed working-directory adaptation. Local format/Clippy/workspace test commands pass (six text and five app tests). No Rust or dependency changes and no local toolchain setup executed. gh authentication and remote reachability verified. Pushed implementation as `96d1d19` to origin/main. [Rust CI run 41619820330](https://github.com/williamcotton/beyond-the-interpreter/actions/runs/41619820330) passed for that exact SHA: checkout, Rust setup, formatting, Clippy, and eleven tests. Logs show Ubuntu 24.04.5 and rustc/Cargo 1.99.1. Verified the run SHA, job/step conclusions, and logs with gh. No corrections were required.

## CH05-PLAN — Frame Chapter 5

**Authorization:** the user accepted 4.4 and requested a commit followed by Chapter 5 section planning. The prior implementation/evidence commits were already present; acceptance was committed as `3eb9e14` (`docs: record acceptance of chapter 4 ci lesson`). No new push performed.

**Status:** Accepted when the user approved the two-section breakdown and requested committing it before drafting 5.1.

**Proposal:** turn characters into ranged tokens, then introduce lookahead through paired operators. These are candidate teaching boundaries, not a fixed section count. Introduce a useful syntax crate with its first scanner behavior and tests; keep CLI unchanged. Place the first character-cursor interaction at the mechanism it explains, with authored observations checked against Rust rather than a JavaScript scanner. Preserve the Chapter 6–8 boundaries. The plan was committed as `f5a2b87` (`docs: outline chapter 5 scanner lessons`). Details remain in the Chapter 5 roadmap frame.

**Evidence:** inspected actual app/text sources and workspace manifests; compared chapter coverage and architecture. Documentation links and whitespace checked. No Rust, manuscript, dependency, CI, or site changes; no unrelated checks or screenshots.

## CH05-01 — Read a character, emit a token

**Authorization:** after accepting the Chapter 5 breakdown, the user requested committing it and moving on to 5.1. Plan committed as `f5a2b87`; current phase is manuscript and author-side visual only.

**Status:** Accepted when the user said “great, commit and then do 5.2.” Matching Rust and cursor observations are verified.

**Deliverables:** [5.1 manuscript](../site/src/content/docs/chapter-5/5.1-first-tokens.mdx), served at <http://localhost:4321/chapter-5/5.1-first-tokens/>; React CharacterCursor component and scoped CSS; Chapter 5 navigation and 4.4 next link; orientation/brief records. Text teaches syntax/text crate wiring, Token/SyntaxKind, char_indices, single-character classification, UTF-8 ranges, Vec collection, empty input and unsupported-character progress. Complete initial files and complete resulting library are shown. No lookahead yet.

**Visual:** two fixed authored observations (`()!=` and `é!`) at the first cursor/range explanation. Shows byte intervals, consumed/unread state, cursor, emitted tokens, previous/next, and a complete textual trace. No JavaScript language engine or general tracing framework. The authored observations have now been compared with the actual native scanner and agree on kind, range, and spelling.

**Checks:** site check zero diagnostics; build succeeds (14 generated pages including 404) with existing optional-content/sitemap notices. Seven manuscript references and fragments resolve, as do affected rendered local links/anchors. Local route HTTP 200. Focused edits reproduce the complete listing exactly; formatting checked through rustfmt stdin without creating/compiling reader code. New interaction checked in headless Chrome after browser connector proved unavailable: both examples, initial/end states, previous/next, selection reset, disabled boundaries, keyboard Tab/Space, textual trace, reduced-motion mode, and 1280/390-pixel layout. Light desktop/mobile and dark mobile inspected; fixed inherited button margin and verified aligned controls/no horizontal overflow. git diff --check passes. No dependency, Rust, or workflow change; no new native tests/builds or push. No temporary Rust implementation.

**Implementation evidence:** followed the displayed route in order from accepted 4.4, creating initial types then appending scan/tests before the first Cargo command. Root/package manifests and library exactly match the displayed files. cargo test -p rlox-syntax passed all three tests and generated only the expected local lock entry; existing registry/local entries unchanged. Formatting, workspace Clippy with warnings denied, and all fourteen tests pass. Temporary native verification linked actual built libraries and compared both component examples’ kinds/ranges/spelling; all match. CLI source bytes/help/usage/read failures preserve expected output streams/statuses. App/text, sample, ignore rules, and workflow unchanged. No manuscript correction, repeated site/browser checks, commit, push, or new hosted run.

**Review focus:** matching section and scanner, useful scope, and whether the cursor explains byte ranges clearly. No unfinished implementation work remains within 5.1.

## CH05-02 — Look ahead before choosing

**Authorization and commit:** the user accepted 5.1 and requested committing it before doing 5.2. Accepted section, native scanner, visual, and records committed as `e5fd3c7` (`feat: pair section 5.1 with single-character scanning`). No push performed.

**Status:** Accepted when the user said “great, commit and then outline chapter 6.” Matching implementation and native visual observations are verified.

**Deliverables:** [5.2 manuscript](../site/src/content/docs/chapter-5/5.2-lookahead.mdx), served at <http://localhost:4321/chapter-5/5.2-lookahead/>; focused LookaheadCursor using the existing cursor presentation; additive inspected-state styling, navigation and records. The accepted 5.1 manuscript body/component/data are preserved. Its next link now points to 5.2.

**Teaching scope:** BangEqual/EqualEqual, explicit mutable peekable iteration, while-let exhaustion, conditional consumption and its closure/reference pattern, extending token ends only for matched '='. Update the existing single-character test so its input still exercises separate characters; add paired/adjacent, failed lookahead before UTF-8 fallback, and trailing-operator checks. Complete resulting library is available separately. Scanner signature and Token remain unchanged; no new crate, dependency, text-type, or CLI change. Chapter 6 topics remain deferred.

**Visual:** fixed authored frames for !=, !(, and trailing =. Inspection visibly leaves the character unread and the logical cursor unchanged; consumption/emission are separate steps. Native token results and iterator observations now agree with all three examples and their authored frame states. Text traces, keyboard controls, labels, byte positions, no automatic animation, and wrapping layout are supplied. No JavaScript scanner or general-purpose trace machinery.

**Checks observed:** site check zero diagnostics; final build succeeds (15 generated pages including 404) with existing notices. Four manuscript references/fragments verified, including corrected precise Crafting Interpreters operators anchor. Affected rendered internal paths/anchors resolve, local page HTTP 200. Displayed changes applied to actual accepted source only in memory; complete resulting view, each meaningful diff’s before/after context, and rustfmt stdin formatting agree. No future Rust files created or compiled. Headless Chrome checked all three cases, correct inspection/consumption/emission states, reset, previous/next, disabled boundaries, keyboard Tab/Space, textual trace, reduced-motion mode, and light desktop/dark mobile at 1280/390 pixels with no horizontal overflow. Accepted 5.1 prose and visual preserved; rlox/.github exactly match HEAD. git diff --check passes. No new Rust tests, dependency changes, push, or Chapter 6 work.

**Implementation evidence:** after explicit text approval/update authorization, followed the displayed enum, loop, operator arms, end-declaration removal, existing test adjustment, and test additions in order. Actual lib.rs matches complete listing exactly. Displayed cargo test -p rlox-syntax, formatting, workspace Clippy with warnings denied, and workspace tests all pass: six syntax tests and seventeen workspace tests. No manifest or lockfile changes. Temporary native verification linked actual built libraries: all visual token kinds/ranges match, and native Peekable operations match every frame’s logical cursor, inspected offset, and emitted count. CLI source bytes/help/usage/read failures retain expected streams/statuses. App/text, sample, ignores, and workflow unchanged. No manuscript correction required; prior presentation checks remain applicable. No new browser/screenshots, repeated site builds, commit, push, or hosted run.

**Review focus:** matching lesson/scanner and whether the lookahead distinction, closure, and token-end reasoning are clear. No unfinished implementation work remains within 5.2.

## Handoff

Accepted starting point: Section 5.1 section/scanner/visual in `e5fd3c7`; fourteen workspace tests previously pass. Section 5.2 is accepted and locally verified. The user authorized committing it followed by Chapter 6 planning only. No push or new manuscript/implementation requested.

**Status meanings:** Proposed = described but not authorized; In progress = authorized work underway; Ready for review = delivered evidence awaiting user judgment, with phase stated; Revision requested = current work remains active; Accepted = explicit user approval with conversation evidence recorded.
