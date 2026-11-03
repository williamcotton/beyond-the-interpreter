# Authoring status

Last updated: 2026-11-02. This file records actual state and decisions, not future assignments. Read the [current brief](LESSON_BRIEF.md) for authorized work and the [roadmap](PROJECT_OUTLINE.md) for chapter coverage.

## Accepted work

- **SITE-01 — Local site and introduction.** Accepted when the user said “ok, looks good” before requesting the initial commits. The introduction is [introduction.mdx](../site/src/content/docs/introduction.mdx), served at <http://localhost:4321/>. It is the accepted orientation/style reference.
- **CH01-01 — Chapter 1, Section 1: A program we can run.** The user approved the text with “this is great, let’s follow the instructions now.” After the implementation report, the user said “I’m happy with what we’ve done so far” and requested this reconciliation. The [section](../site/src/content/docs/chapter-1/1.1-first-program.mdx) and matching [Rust source](../rlox/src/main.rs) are accepted as the first executable endpoint. Local page: <http://localhost:4321/chapter-1/1.1-first-program/>.

**Chapter 1 complete:** the user explicitly chose to keep only Section 1.1, omit dedicated Git/README sections, and introduce tests in Chapter 2. The accepted Rust endpoint still has no test cases.

Section 1.1 demonstrates a code introduction, a meaningful one-line edit, complete resulting file views, and an ordered build/run route. It is not an accepted example of an algorithm explanation or interactive visual; those have yet to be authored.

## Actual repository and capabilities

- `site/`: Astro/Starlight publication with MDX, React integration, and strict TypeScript. The introduction and Chapter 1 are accepted. Chapter 1 consists of Section 1.1 only; the proposed Section 1.2 was removed after scope review. Section 2.1 and its matching Rust endpoint are accepted; Section 2.2 and its matching Rust/sample are accepted; Section 2.3 and its matching Rust are accepted. Navigation includes only authored pages. Starlight supplies listings and a focused diff; no custom diff viewer or trace framework exists.
- `rlox/`: one binary package named `rlox`, Rust edition 2024, no external dependencies. It reads the first supplied filename as UTF-8 and prints its contents without adding a newline. Missing or extra arguments print usage to stderr and fail; failed reads print a contextual stderr diagnostic and fail. Successful reads return success. Files are Cargo.toml, Cargo.lock, src/main.rs, .gitignore, and hello.lox. It does not scan tokens or execute Lox. There are zero Rust test cases.
- `authoring/`: current roadmap, architecture, review guidance, status, and brief. No fixtures/replay framework or future crate scaffold exists.
- Hosted CI, deployment, release artifacts, distribution, and cross-platform coverage: not established.

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

## Handoff

Accepted starting point: Section 2.2 source-reading Rust and hello.lox in `9520c43`. Current authorized action: commit accepted Section 2.3 and its matching Rust, then draft Section 2.4 for text review. Preserve the accepted implementation while drafting. No push or publication is authorized.

**Status meanings:** Proposed = described but not authorized; In progress = authorized work underway; Ready for review = delivered evidence awaiting user judgment, with phase stated; Revision requested = current work remains active; Accepted = explicit user approval with conversation evidence recorded.
