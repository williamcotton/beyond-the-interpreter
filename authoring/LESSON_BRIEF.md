# Current assignment

## CH02-PLAN — Frame Chapter 2’s sections

**Status:** Accepted as the working section frame when the user said “ok, first commit these changes and then create chapter 2.” The frame remains adjustable through text review; no Rust implementation is authorized yet.

**Authorization:** after accepting Chapter 1’s single-section boundary and moving tests to Chapter 2, the user asked to frame Chapter 2’s sections and keep code changes small in each section. This explicitly requests a provisional section breakdown for this chapter; it is not a fixed count or a template for later chapters.

**Audience and chapter question:** programmers new to Rust who can already build/run the greeting program. How can `rlox` obtain source from a file and explain why it cannot do so?

**Accepted starting point:** complete Chapter 1 consists of Section 1.1 and its matching dependency-free `rlox` package. It prints a fixed greeting; no arguments, file I/O, or automated test cases exist. The discarded Section 1.2 was never implemented. The accepted source and code listings remain reconstructible.

## Proposed sections

### 2.1 — Give the program a filename

**Question:** how does the path typed after the command reach our program?

Replace the fixed greeting with selecting and displaying one positional filename. Explain Cargo’s `--` separator, the executable-name argument, argument iteration, and the `Option` from selecting a value. A documented temporary `expect` may handle missing input so the main path remains small; label and demonstrate that limitation and replace it in 2.3. A path containing spaces illustrates shell quoting.

**Code boundary:** a small change in `src/main.rs`; no file I/O, collection/parsing framework, helpers, tests, or dependencies. Outcome: the reader can predict which filename is selected. Existing extra-argument behavior must be stated rather than silently pretending full validation exists.

### 2.2 — Read the source

**Question:** how does a path become the text of a Lox program?

Add `std::fs::read_to_string` and print the returned source without adding a newline. Add a small `.lox` example with precise contents. Explain owned `String`, borrowed text/path arguments where actually used, and the operation’s `Result`. Keep any initial `expect` handling explicit and temporary. Missing/unreadable files motivate the following section; do not promise execution or tokenization.

**Code boundary:** one filesystem operation and output change in main, plus the example file. No source container, coordinate types, modules, or general file abstraction. Outcome: the supplied file’s original text appears on stdout. Tests are not prerequisites to this first useful observation.

### 2.3 — Explain input failures

**Question:** what should a caller see when input is absent, ambiguous, or unreadable?

Replace temporary panic handling with usage/file-error messages on stderr and an unsuccessful exit status. Define the input rule explicitly: one filename is required; extra positional arguments are rejected. Put that small argument-selection/validation rule in a focused function, separating it from process I/O so its decisions are understandable and independently checkable. Show absent path, extra path, nonexistent file, and a valid file. Introduce matching/error control flow and exit status at their use.

**Code boundary:** the existing main plus one small argument-policy helper in the same file. Preserve successful source output. No CLI library, error hierarchy, generic dispatcher, library crate, or general test harness. Do not silently select an elaborate error type; settle the smallest teachable signature while drafting.

### 2.4 — Check the input rules

**Question:** how do we preserve the argument decisions without rerunning every case by hand?

Add unit tests beside the argument-policy function: one filename is accepted, no filename is rejected, and extra filenames are rejected. Teach test discovery, assertions, targeted execution, and a deliberate validation regression followed by restoration. Reuse the function introduced for the real input policy; do not change production behavior to manufacture a testable toy. Keep file/terminal smoke checks separate from what the unit tests prove. Formatting can be a brief supporting command.

**Code boundary:** a small test module in the existing source file. No process spawning, `CARGO_BIN_EXE_*`, temporary-directory framework, new test dependency, filesystem wrapper just for testing, or claims that these unit tests validate OS file I/O. Outcome: repeatable evidence for the user-input rules, plus the documented manual file checks.

## Review constraints and handoff

These boundaries keep different changes separate: getting input, performing I/O, deciding failure behavior, and automating checks of that policy. They are provisional and may change after reviewing the actual prose. Small code size alone does not establish reasonable teaching load. If error handling or the test needs too many unfamiliar mechanisms, revise that section’s design or boundary before implementing; do not fill the page with unexplained support code.

**Visual approach:** no algorithm visual is currently justified. Focused code, source examples, and contrasting terminal observations should explain these mechanisms. Reassess while drafting rather than promising an infrastructure component now.

**Excluded:** section prose or code changes in this planning task; scanner/parser work, subcommands/Clap (Chapter 3), workspaces/ranges, dependencies, Git/README sections, pushes and publication. The user subsequently authorized committing this closeout/plan. Later protocol/CLI integration tests remain possible when a concrete capability needs them; this plan does not require them.

**Verification:** inspect current source and roadmap; check prerequisite progression, explicit temporary behavior and replacement, meaningful test coverage/limits, internal consistency, and preservation of manuscript/code. No runtime builds or tests needed for a plan.

**Next permitted action:** commit the Chapter 1 closeout and this plan as requested, then write/render Section 2.1. Keep `rlox/` at the accepted Chapter 1 endpoint until its text is approved and implementation is authorized. Chapter 2’s eventual checkpoint is reading `.lox` files with useful errors and meaningful input-policy tests.
