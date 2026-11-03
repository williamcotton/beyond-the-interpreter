# Current assignment

## CH02-02 — Read the source (combined review)

**Status:** Accepted; commit closeout authorized.

**Authorization:** after accepting and committing Section 2.1 and the site reorganization as `3fa2dc9`, the user requested moving on to 2.2. The user subsequently approved the text (“it reads great”) and explicitly authorized updating rlox to match.

**Audience and question:** readers at the accepted filename-selection endpoint. How do we get the contents of the named file into our program?

**Accepted starting point:** Section 2.1’s main selects/displays argument 1; no file I/O or tests. Its complete listing preserves that endpoint.

**Scope and behavior:** draft `site/src/content/docs/chapter-2/2.2-read-source.mdx`, update current navigation and author records. Teach creating `rlox/hello.lox`, borrowing the filename for `std::fs::read_to_string`, extracting a String from Result with temporary expect, and printing the source without an added newline. Only main and the tiny sample file changed during implementation. No module, manifest, helper, dependency, or abstraction is required.

**Teaching sequence:** create a UTF-8 Lox sample; observe accepted code still prints its name; explain read_to_string, owned source versus filename, reference/borrowing, Ok/Err versus Some/None, temporary read-error panic, and print versus println; show focused diff and complete resulting file; inspect source output; explain relative paths, deliberately fail on an absent path, and recover using the existing sample. Source display is distinct from Lox execution.

**References:** official std::fs::read_to_string documentation, Rust Book Reading a File and References and Borrowing, and Crafting Interpreters §4.1 The Interpreter Framework. No version or dependency selected. The used filesystem API predates the current edition/toolchain. &str and ? need no artificial helper here; introduce them when a later edit uses them.

**Visual approach:** paired source/output listings and focused diff answer what changed and what was read. No custom diagram or interaction is needed for this section.

**Verification:** prior text-phase checks covered official references, diff/listing consistency, site check/build, links, and rendered inspection. The implementation pass followed the displayed commands in order from 2.1, including baseline output, file-read output, missing-file failure, and recovery. Main and hello.lox exactly match their complete listings. Formatting, Clippy with warnings denied, and Cargo test passed (zero cases). Direct checks confirmed exact stdout and exit/stderr behavior, including empty/no-final-newline/UTF-8/CRLF inputs, spaced paths, invalid UTF-8, missing files, and missing arguments. No manuscript correction was needed. Verification used the existing macOS toolchain; other platforms and permission-denied behavior were not tested. No browser or screenshot checks repeated for this Rust-only pass, following the user’s usage preference now recorded in AGENTS.md.

**Excluded:** final errors/argument policy (2.3), tests (2.4), ? propagation, language execution, dependencies, future sections, commits, pushes, publication.

**Stopping point:** the user accepted the combined endpoint and requested committing it before moving to Section 2.3’s text phase. Commit this accepted work, including the requested proportional presentation-check guidance. No push or publication is authorized.
