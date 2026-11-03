# Current assignment

## CH02-03 — Explain input failures (combined review)

**Status:** Accepted; commit closeout authorized.

**Authorization:** the user accepted 2.2 and requested committing it, then moving to 2.3. The accepted endpoint and supporting guidance were committed as `9520c43`. The user subsequently approved the text (“ok, looks good”) and explicitly requested updating rlox.

**Reader question:** how can someone running rlox tell what went wrong and how to fix it, without a Rust panic report?

**Accepted starting point:** main selects argument 1 with expect, reads UTF-8 source with expect, and prints it unchanged. hello.lox exists. Complete 2.1/2.2 listings preserve those endpoints.

**Scope and behavior:** one manuscript page at `site/src/content/docs/chapter-2/2.3-input-errors.mdx`, navigation, and author records. Implement one pure argument-policy function in main.rs accepting borrowed strings and selecting exactly one filename. Collect arguments after the executable name, use Option matching for usage failures and Result matching for read failures, write errors to stderr, return ExitCode from main, preserve successful source output. No module, dependency, fixture, or manifest change.

**Teaching sequence:** observe the current missing-input panic; define one-filename policy; explain the helper and its slice/string references; connect it to main through collection, match, and explicit exit status; check missing/extra inputs and successful source output; replace read expect with Result matching and contextual error text; deliberately fail on an absent path and recover. Provide complete final source and label the intermediate read panic until replaced. This is one bounded input-error improvement; human review must judge the new Rust reasoning and may request a split.

**New Rust at use:** Vec<String>, skip/collect, borrowed &[String] and &str, function parameters/return types, length guard/indexing, Option/Result matching, early return, eprintln, ExitCode, tail expressions. No generic helper, explicit lifetimes, error type, library extraction, or ? propagation is required.

**References:** current official Rust Book vectors, slices, match, error-handling refactoring, and stderr; standard-library Iterator::collect, Vec slicing, and ExitCode; Crafting Interpreters §4.1.1 Error handling. Check API compatibility against the existing edition/toolchain without selecting new dependencies.

**Visual approach:** terminal contrasts and focused edits with complete resulting source suffice. No custom visual needed. Follow AGENTS.md’s proportional checks; routine manuscript/listing changes do not require screenshots or browser automation.

**Checks observed:** official references and compatibility checked; both focused edits plus the added helper reconstruct the complete final listing from actual 2.2. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. All five local pages and their links/anchors resolve; all seven manuscript reference URLs/fragments resolve. Rust and hello.lox match the accepted commit. No screenshots/browser automation or proposed Rust execution occurred. git diff --check passed. These were text-phase checks. The authorized implementation pass then followed the displayed route in order, including the baseline panic, helper addition, intermediate usage failures and recovery, final read-error handling, repeated usage failures, and final successful recovery. Every observation matched. Main exactly matches the complete listing. Formatting, Clippy with warnings denied, and Cargo test passed (zero cases). Direct executable checks verified exact stdout/stderr and exit status for usage/read failures and success, argument validation before I/O, source preservation, spaced paths, and invalid UTF-8. No manuscript correction was needed. Checks used the existing macOS toolchain; other platforms and permission-denied behavior were not tested.

**Excluded:** new test cases (2.4), new CLI features/flags, Clap, crate/module extraction, language execution, new dependencies, later sections, commits, pushes, publication.

**Stopping point:** the user accepted Section 2.3 (“great”) and authorized committing it before drafting 2.4. Commit this accepted endpoint, then begin only the next text phase. No push or publication.
