# Authoring status

Last updated: 2026-11-02. This file records actual state and decisions, not future assignments. Read the [current brief](LESSON_BRIEF.md) for authorized work and the [roadmap](PROJECT_OUTLINE.md) for chapter coverage.

## Accepted work

- **SITE-01 — Local site and introduction.** Accepted when the user said “ok, looks good” before requesting the initial commits. The introduction is [index.mdx](../site/src/content/docs/index.mdx), served at <http://localhost:4321/>. It is the accepted orientation/style reference.
- **CH01-01 — Chapter 1, Section 1: A program we can run.** The user approved the text with “this is great, let’s follow the instructions now.” After the implementation report, the user said “I’m happy with what we’ve done so far” and requested this reconciliation. The [section](../site/src/content/docs/foundations/a-third-lox/first-program.mdx) and matching [Rust source](../rlox/src/main.rs) are accepted as the first executable endpoint. Local page: <http://localhost:4321/foundations/a-third-lox/first-program/>.

**Chapter 1 complete:** the user explicitly chose to keep only Section 1.1, omit dedicated Git/README sections, and introduce tests in Chapter 2. The accepted Rust endpoint still has no test cases.

Section 1.1 demonstrates a code introduction, a meaningful one-line edit, complete resulting file views, and an ordered build/run route. It is not an accepted example of an algorithm explanation or interactive visual; those have yet to be authored.

## Actual repository and capabilities

- `site/`: Astro/Starlight publication with MDX, React integration, and strict TypeScript. The introduction and Chapter 1 are accepted. Chapter 1 consists of Section 1.1 only; the proposed Section 1.2 was removed after scope review. Navigation covers only the introduction and Section 1.1. Starlight supplies listings and a focused diff; no custom diff viewer or trace framework exists.
- `rlox/`: one binary package named `rlox`, Rust edition 2024, no external dependencies. It prints `Hello from rlox!`. Files are Cargo.toml, Cargo.lock, src/main.rs, and .gitignore. It does not read files, parse arguments, scan tokens, or execute Lox. There are zero Rust test cases.
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
- Authorized initial commits: `066063b` (root Markdown) and `8b52cb7` (site and ignore rules). Section 1.1, navigation, and its matching Rust endpoint were committed together as `db100b7`. The reconciled guidance was committed separately as `ac27e2e`. Nothing pushed or published.

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

**Status:** Accepted as the working frame when the user asked to commit it and create Chapter 2. The frame records four adjustable boundaries: filename selection; source reading; helpful failures and argument policy; unit tests for that policy. Sections may be revised through actual text review; no permanent section count or file/line quota is established.

**Scope rationale:** give source reading an early observable payoff; replace temporary `expect` handling before the chapter endpoint; test actual input rules with unit tests rather than introducing greeting-only process infrastructure. Keep arguments, file I/O, failure behavior, and testing as distinct changes. A source example is the only proposed new non-Rust file; production code stays in the small binary without a crate split or dependency.

**Evidence and limits:** inspected current main/manifest and Chapter 2 roadmap. Reviewed prerequisites, explicit temporary failure behavior/recovery, test scope, and chapter boundaries. No code, MDX, dependencies, runtime behavior, or checks changed. Reader commands, helper signature, and test details remain to be authored and verified through the text-first loop.

## Handoff

The user authorized committing the Chapter 1 closeout and Chapter 2 frame, then creating Chapter 2. Start with Section 2.1’s manuscript under the established one-section-at-a-time workflow. Preserve the accepted Chapter 1 Rust endpoint until that text is approved and following its instructions is authorized. No Rust, later-section prose, push, or publication is authorized by this handoff.

**Status meanings:** Proposed = described but not authorized; In progress = authorized work underway; Ready for review = delivered evidence awaiting user judgment, with phase stated; Revision requested = current work remains active; Accepted = explicit user approval with conversation evidence recorded.
