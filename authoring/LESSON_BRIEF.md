# Current assignment

## CH03-01 — Give the operation a name (combined review)

**Status:** Accepted; commit closeout authorized.

**Authorization:** the user approved the Chapter 3 working section frame (“great, move on to 3.1”). The user subsequently approved the text (“text looks great”) and explicitly requested updating rlox. The accepted starting commit is `04d4575`.

**Reader question:** how does rlox distinguish an operation name from that operation’s filename?

**Scope and behavior:** one page at `site/src/content/docs/chapter-3/3.1-source-command.mdx`, current navigation and author records. Introduce `source FILE` with one small selector that splits the first argument, recognizes source, and delegates remaining arguments to filename_from_args. Change main’s selection and usage text; keep file reading/output untouched. Keep all existing tests and add direct command-boundary tests. The command form replaces `rlox FILE`, as explicitly approved in planning.

**Teaching sequence:** observe the accepted program reject source plus filename; explain first/rest slice splitting, tuple binding, Option’s ? early return, as_str and string/wildcard matching; add selector; connect main and update usage; check source success, missing/unknown/retired command forms, missing/extra filenames, read failure, and recovery; add command tests and run them. Supply the complete resulting source.

**New Rust at use:** split_first, tuple binding, ? on Option, String::as_str, string and wildcard match patterns, grouped use imports, and local binding shadowing in tests. No new command enum/type yet; the next section introduces structured commands with Clap. Existing borrowed return and filename rule remain.

**References:** current official slice split_first, Option ? and String::as_str docs, plus the Rust Book match discussion. The APIs predate the existing toolchain/edition; no dependency/version selection. Source display remains an honest name; no tokens or execution claims.

**Visual approach:** source/output contrasts and focused diffs suffice. No custom visual, layout, or navigation behavior change. Apply AGENTS.md proportional checks without routine browser/screenshots.

**Checks observed:** current official split_first, Option ?, as_str, and match references/compatibility checked. Both focused edits and additions reconstruct the complete listing from actual Chapter 2 source using string consistency checks only. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. Seven routes, local links/anchors, and all four manuscript reference URLs/fragments resolve. At text handoff all five rlox files matched HEAD; git diff --check passed. No browser/screenshots. These were text-phase checks. The authorized implementation pass then followed the entire displayed route: baseline rejection, helper addition, main changes, source output, all five usage failures, read failure, recoveries, test import/additions, and cargo test. Every observation matched. Main exactly matches its complete listing. Seven tests, formatting, and Clippy with warnings denied pass. Direct execution confirmed exact stdout/stderr and exit status for all documented cases. Other rlox files are unchanged. No manuscript correction or repeated site/browser checks needed. Verification used the existing macOS toolchain; no cross-platform claim.

**Excluded:** Clap/dependencies, hand-built help/flags/aliases, additional commands, crate/module extraction, new fixtures, 3.2 drafting, commits, pushes, publication.

**Stopping point:** the user accepted 3.1 (“great”) and requested committing it before moving on to 3.2 text. Commit the accepted section/code, then draft 3.2 only. No push or publication.
