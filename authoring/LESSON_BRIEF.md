# Current assignment

## CH04-02 — Identify the source file (combined review)

**Status:** Accepted; commit closeout authorized.

**Authorization:** the user accepted 4.1 and requested a commit followed by 4.2. Committed the accepted text/library and supporting records as `123b100` (`feat: pair section 4.1 with source byte ranges`) before drafting. The user subsequently approved the text (“ok, looks good”) and explicitly requested updating rlox.

**Reader question:** how do we distinguish the same byte range in different source files, then choose the right source for slicing?

**Starting point:** one package with CLI and library targets. TextSize/TextRange provide ordering and UTF-8/bounds checks; four library tests and five CLI tests. No file identities or lookup service. Actual lib.rs inspected; complete 4.1 listings preserve its endpoint.

**Scope:** page at `site/src/content/docs/chapter-4/4.2-file-origins.mdx`, navigation and records. Add FileId(pub usize), SyntaxOrigin with public file/range fields, and two library tests. Keep range implementation, existing tests, CLI, manifest/lockfile, dependencies, and fixtures unchanged. No package/workspace move yet.

**Teaching sequence:** compare [0,5) in print and while sources; assign local source IDs; add public identity/origin types using existing derives; update test import and check distinct origins for equal ranges; run library tests; add an explicit source selection match on FileId in a second test, then slice the chosen text; rerun library/full tests and CLI. Provide complete resulting lib.rs.

**New Rust at use:** named-field construction reuses struct syntax from 3.2; grouped imports, Copy and equality from prior lessons. Introduce assert_ne! and tuple-struct patterns in an explicit test lookup, with a test-setup panic for an unassigned ID. No new production methods, generic abstraction, lifetime, dependency, or error policy.

**Contract:** IDs are labels within the source owner's set, not paths/global names/existence checks. SyntaxOrigin equality includes both file and range. Origin records the association but does not retrieve text or ensure a caller used the correct source. The example owns an explicit two-source mapping; do not build a VFS/database or claim global/stable ID allocation. Numeric range and UTF-8 validation remain unchanged.

**Visual approach:** contrasting source listings and the explicit lookup test answer which source a range refers to. No diagram/component/layout/interaction change; no routine browser/screenshots under AGENTS.md.

**Checks observed:** current official Rust Book struct/pattern and assert_ne docs, plus Crafting Interpreters location-information section checked. Additions/import edit reconstruct the complete listing from actual 4.1 using string checks only; source examples select the stated first five bytes. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. Ten local pages, local links/anchors, and all four reference URLs/fragments resolve. All six rlox files match HEAD; git diff --check passed. No browser/screenshots. These were text-phase checks. The authorized implementation pass added the types/import and identity test, ran cargo test --lib, added the source-selection test, reran library/full tests, and ran the CLI in order. All expected observations matched. Six library tests and five binary tests pass, as do formatting and Clippy with warnings denied. lib.rs matches the complete listing; all other rlox files match HEAD. Direct CLI execution confirmed byte-exact stdout, empty stderr, and exit 0. No manuscript correction or repeated site/browser checks needed. Existing macOS toolchain only; no other-platform verification.

**Excluded:** real file lookup/storage or path interning, revision tracking, workspace (4.3), CI (4.4), new CLI behavior, dependencies/fixtures, 4.3 drafting, commits, pushes, publication.

**Stopping point:** the user accepted 4.2 (“great”) and requested committing it before 4.3 text. Commit the accepted endpoint before drafting; no push or publication authorized.
