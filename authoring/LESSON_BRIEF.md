# Current assignment

## CH03-02 — Describe the command with Clap (combined review)

**Status:** Accepted; commit closeout authorized.

**Authorization:** the user accepted 3.1 and requested a commit followed by 3.2. Committed accepted text/code and supporting records as `9d71905` (`feat: pair section 3.1 with source command dispatch`). The user subsequently approved the text (“ok, looks good”) and explicitly requested updating rlox.

**Reader question:** how do we describe the source command once so its arguments, validation, and help agree?

**Starting point:** manual source FILE command and two borrowed argument helpers, contextual file-read failures, and seven filename/command unit tests. main.rs, Cargo.toml/lock, ignore file, and hello.lox inspected. Complete 3.1 listings preserve that endpoint.

**Scope:** manuscript at `site/src/content/docs/chapter-3/3.2-clap.mdx`, navigation and author records. Implement Clap derive, Cli struct and Command::Source with owned String filename; replace manual parsing and helpers, retain source read/output and errors, migrate tests to try_parse_from with supplied argv. No crate/module extraction, new operations, fixtures, or path-type migration.

**Dependency decision for text review:** exact Clap 4.6.7 with derive feature. Current official docs and published normalized Cargo.toml identify this release and Rust 1.85 minimum, compatible with local Rust 1.95 and project edition 2024. The published dependency manifest was read without modifying rlox or invoking Cargo. The implementation pass resolved and compiled Clap/builder/derive 4.6.7 with the generated Cargo.lock on Rust 1.95. Minimum-version and other-platform builds were not executed. No historic pin reused.

**Teaching sequence:** observe old --help usage failure; show manifest diff/complete manifest; introduce struct, enum, fields, derives, trait import, documentation-based help; show focused main edit and helper removals; explicitly remove obsolete tests as an intermediate endpoint; cargo check generates dependency lock entries; exercise global/command help, success, all prior invalid forms, missing-file failure and recovery; replace unit tests with interface tests, run them, and finish successfully. Provide complete resulting main.rs.

**New Rust at use:** struct and enum declarations, fields and variant data, derive macros and generated trait implementations, field access/pattern binding with owned String, Result::is_err/err, assert!, and a library error-kind enum. String filename preserves current Unicode boundary; no PathBuf or generic abstraction introduced.

**Behavior transition:** source FILE stays the same; Clap supplies help and diagnostics. Requested help exits 0 on stdout, usage failures exit 2 on stderr (previously 1), and file-read failures retain main’s failure status (1 locally). No new source-processing behavior. Explicit test migration retires helper-specific tests; typed success and rejection/help outcomes replace them. Tests do not prove file I/O or process streams; terminal checks do.

**Visual approach:** existing listings, focused edits, and terminal excerpts suffice; no custom component, layout, or navigation behavior change. No routine browser/screenshots.

**Checks observed:** current official references and published manifest checked. Manifest diff and all source edits/removals/replacements reconstruct the complete manifest/main listings from actual 3.1 using string checks only. Site check reported zero diagnostics; build succeeded with existing optional-content/sitemap notices. Eight routes, local links/anchors, and all six manuscript reference URLs/fragments resolve. All five rlox files match HEAD; git diff --check passed. No browser/screenshots. These were text-phase checks. The authorized implementation pass followed the displayed route: old help failure, manifest/declarations/main edits, helper/test removal, cargo check and lockfile generation, both help commands, source success, all usage failures, read failure and recovery, replacement tests, cargo test, and final success. Every observation matched; no manuscript correction was needed. Five tests, formatting, and Clippy with warnings denied pass. Source and manifest match the complete listings. Direct execution verified separate stdout/stderr and status 0 for help/success, 2 for usage, 1 for file-read failure. Library help and -- filename separator behavior also checked. No repeated site/browser checks.

**Excluded:** CLI process-test infrastructure, new flags beyond library help behavior, additional operations, Clap builder API, workspace/crate migration, future chapters, commits, pushes, publication.

**Stopping point:** the user accepted 3.2 (“great”) and requested committing it, then outlining Chapter 4 sections. Commit this accepted endpoint before planning. No Chapter 4 manuscript or implementation, push, or publication authorized.
