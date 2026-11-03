# Current assignment

## CH04-01 — Locate text with byte ranges (combined review)

**Status:** Accepted; commit closeout authorized.

**Authorization:** after reviewing the Chapter 4 frame, the user requested starting 4.1 and reported adding a Git remote. The user subsequently approved the text (“ok, looks good”) and explicitly requested updating rlox. Accepted starting commit: `5ddd5a9`.

**Reader question:** how do we identify a source fragment and check that its byte offsets select valid UTF-8 text?

**Scope:** one MDX page at `site/src/content/docs/chapter-4/4.1-byte-ranges.mdx`, navigation and author records. Add a new library target at rlox/src/lib.rs, containing handwritten TextSize and TextRange, an order-checking constructor, safe source slicing with str::get, and focused library tests. Keep main/CLI, manifest, dependencies, lockfile, and fixtures unchanged. No workspace or additional package yet.

**Teaching sequence:** locate print with [0,5); explain half-open spans/empty positions and byte units; create complete initial lib.rs with public tuple coordinate/private range fields and constructor; teach standard derives, impl/Self and library target discovery; add slice method and ASCII/order tests; run cargo test --lib; explain a multibyte sample with byte-boundary diagram and add a UTF-8 test; check empty EOF, out-of-bounds, and empty interior UTF-8 positions; run library/full tests and existing CLI; provide complete resulting file.

**New Rust at use:** tuple structs, usize, public visibility/private fields, Debug/Clone/Copy/PartialEq/Eq derives, impl/Self, associated function versus by-value self method, integer comparison, half-open Range indices, str::get, and Cargo library target. Existing Option and unit-test tools are reused. Copy lets small coordinates be reused, while slice returns a view borrowed from its source. No explicit lifetimes, offset arithmetic framework, or new dependency.

**Contract:** new enforces start <= end; it does not validate against an unknown source. slice validates bounds and UTF-8 boundaries against its supplied source. Empty ranges are accepted only at valid boundaries, including EOF. Byte offsets do not represent character or display-column indices. Keep endpoints private for callers outside the library.

**Visual approach:** a static monospace diagram for aé! at the first UTF-8 explanation shows offsets 0..4 and bytes 61 C3 A9 21. Surrounding prose states all meaningful boundaries for accessibility. It answers why [1,3) succeeds while [1,2) does not. No custom component/interaction; source and diagram-data checks suffice under AGENTS.md’s proportional-check rule, without routine browser/screenshots.

**Checks observed:** current official Cargo library-target, Rust Book methods/tuple structs, and str::get docs checked. Initial file plus each addition reconstruct the complete listing by string checks only. UTF-8 diagram byte values and column alignment checked (and one spacing error corrected). Site check reported zero diagnostics; build succeeded with existing optional-content/sitemap notices. Nine local pages and their links/anchors, plus all four manuscript reference URLs/fragments, resolve. All five accepted rlox files match HEAD; lib.rs does not exist. git diff --check passed; no browser/screenshots. These were text-phase checks. The authorized implementation pass created lib.rs from the initial listing, added slice and the first tests, then ran cargo test --lib after each displayed addition (2, then 3, then 4 passing cases). Full cargo test passed all four library and five CLI tests; the final CLI run matched the text. Formatting and Clippy with warnings denied pass. lib.rs matches the complete listing exactly, and all existing rlox files match HEAD. Direct CLI execution confirmed byte-exact stdout, empty stderr, and exit 0. No manuscript correction or repeated site/browser checks needed. Existing macOS toolchain only; no minimum-version or other-platform verification.

**Remote:** origin is configured as https://github.com/williamcotton/beyond-the-interpreter.git, confirmed with read-only inspection. No push/CI run was requested or performed. This updates the earlier planning prerequisite; hosted workflow evidence remains future authorized work.

**Excluded:** CLI range options, FileId/SyntaxOrigin (4.2), workspace extraction (4.3), CI (4.4), Rowan/scanning, new dependencies/fixtures, 4.2 drafting, commits, pushes, publication.

**Stopping point:** the user accepted 4.1 (“great”) and requested committing it, then moving to 4.2 text. Commit the accepted endpoint before drafting. No push or publication authorized.
